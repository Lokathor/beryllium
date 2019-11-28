use super::*;

/// Handle to a [`SDL_GLContext`](https://wiki.libsdl.org/SDL_GL_CreateContext).
#[repr(transparent)]
pub struct GlContext {
  // Note: The Init token is stored in the GlWindow
  pub(crate) ctx: fermium::SDL_GLContext,
}
impl Drop for GlContext {
  fn drop(&mut self) {
    unsafe { fermium::SDL_GL_DeleteContext(self.ctx) }
  }
}

/// A [`Window`] and its [`GlContext`], fused into one.
pub struct GlWindow {
  pub(crate) init_token: Rc<Initialization>,
  pub(crate) win: ManuallyDrop<Window>,
  pub(crate) ctx: ManuallyDrop<GlContext>,
}
impl Drop for GlWindow {
  fn drop(&mut self) {
    unsafe {
      ManuallyDrop::drop(&mut self.ctx);
      ManuallyDrop::drop(&mut self.win);
    }
    WINDOW_EXISTS.store(false, Ordering::SeqCst);
  }
}
impl core::ops::Deref for GlWindow {
  type Target = Window;
  fn deref(&self) -> &Window {
    &self.win
  }
}
impl GlWindow {
  /// Swaps the window buffers.
  ///
  /// Depending on the swap interval, this will block for some period of time or
  /// not at all.
  pub fn swap_window(&self) {
    unsafe { fermium::SDL_GL_SwapWindow(self.win.win) }
  }

  /// Gets the physical size of the draw area.
  /// 
  /// Basically you just pass this to `glViewport`.
  pub fn get_drawable_size(&self) -> (i32, i32) {
    let mut w = 0_i32;
    let mut h = 0_i32;
    unsafe { fermium::SDL_GL_GetDrawableSize(self.win.win, &mut w, &mut h) }
    (w, h)
  }

  /// Loads a function pointer for the named function.
  /// 
  /// ## Safety
  /// 
  /// The pointer given must be a null-terminated string.
  pub unsafe fn get_proc_address(&self, func: *const c_char) -> *mut c_void {
    fermium::SDL_GL_GetProcAddress(func)
  }

  /// If the named extension is supported or not.
  ///
  /// This version turns your `&str` into a null-terminated string and then
  /// calls to [`supports_extension_c`]. If you already have the null-terminated
  /// string you should call the other method directly.
  pub fn supports_extension(&self, extension: &str) -> bool {
    let extension_null = extension.alloc_c_str();
    unsafe { self.supports_extension_c(extension_null.as_ptr()) }
  }

  /// If the named extension is supported or not.
  /// 
  /// ## Safety
  /// 
  /// The pointer given must be a null-terminated string.
  pub unsafe fn supports_extension_c(&self, extension: *const c_char) -> bool {
    fermium::SDL_GL_ExtensionSupported(extension) == fermium::SDL_TRUE
  }

  /// Gets the current swap interval.
  pub fn get_swap_interval(&self) -> Option<SwapInterval> {
    match unsafe { fermium::SDL_GL_GetSwapInterval() } {
      0 => Some(SwapInterval::Immediate),
      1 => Some(SwapInterval::Vsync),
      -1 => Some(SwapInterval::AdaptiveVsync),
      _ => None,
    }
  }

  /// Sets the desired swap interval.
  pub fn set_swap_interval(&self, interval: SwapInterval) -> i32 {
    unsafe { fermium::SDL_GL_SetSwapInterval(interval as i32) }
  }

  /// Obtains the actual current value of the given attribute.
  pub fn gl_get_attribute(&self, attr: SdlGlAttr) -> Result<i32, String> {
    let mut out = 0_i32;
    let ret = unsafe {
      fermium::SDL_GL_GetAttribute(attr as fermium::SDL_GLattr, &mut out)
    };
    if ret >= 0 {
      Ok(out)
    } else {
      Err(self.init_token.get_error())
    }
  }
}

/// The various attributes that you can request a specific value for.
///
/// See [`SDL::gl_set_attribute`]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
pub enum SdlGlAttr {
  /// the minimum number of bits for the red channel of the color buffer;
  /// defaults to 3.
  RedSize = fermium::SDL_GL_RED_SIZE,

  /// the minimum number of bits for the green channel of the color buffer;
  /// defaults to 3
  GreenSize = fermium::SDL_GL_GREEN_SIZE,

  /// the minimum number of bits for the blue channel of the color buffer;
  /// defaults to 2
  BlueSize = fermium::SDL_GL_BLUE_SIZE,

  /// the minimum number of bits for the alpha channel of the color buffer;
  /// defaults to 0
  AlphaSize = fermium::SDL_GL_ALPHA_SIZE,

  /// the minimum number of bits for frame buffer size; defaults to 0
  BufferSize = fermium::SDL_GL_BUFFER_SIZE,

  /// whether the output is single or double buffered; defaults to double
  /// buffering on
  DoubleBuffer = fermium::SDL_GL_DOUBLEBUFFER,

  /// the minimum number of bits in the depth buffer; defaults to 16
  DepthSize = fermium::SDL_GL_DEPTH_SIZE,

  /// the minimum number of bits in the stencil buffer; defaults to 0
  StencilSize = fermium::SDL_GL_STENCIL_SIZE,

  /// the minimum number of bits for the red channel of the accumulation
  /// buffer; defaults to 0
  AccumRedSize = fermium::SDL_GL_ACCUM_RED_SIZE,

  /// the minimum number of bits for the green channel of the accumulation
  /// buffer; defaults to 0
  AccumGreenSize = fermium::SDL_GL_ACCUM_GREEN_SIZE,

  /// the minimum number of bits for the blue channel of the accumulation
  /// buffer; defaults to 0
  AccumBlueSize = fermium::SDL_GL_ACCUM_BLUE_SIZE,

  /// the minimum number of bits for the alpha channel of the accumulation
  /// buffer; defaults to 0
  AccumAlphaSize = fermium::SDL_GL_ACCUM_ALPHA_SIZE,

  /// whether the output is stereo 3D; defaults to off
  Stereo = fermium::SDL_GL_STEREO,

  /// the number of buffers used for multisample anti-aliasing; defaults to 0;
  /// see Remarks for details
  MultisampleBuffers = fermium::SDL_GL_MULTISAMPLEBUFFERS,

  /// the number of samples used around the current pixel used for multisample
  /// anti-aliasing; defaults to 0; see Remarks for details
  MultisampleSamples = fermium::SDL_GL_MULTISAMPLESAMPLES,

  /// set to 1 to require hardware acceleration, set to 0 to force software
  /// rendering; defaults to allow either
  AcceleratedVisual = fermium::SDL_GL_ACCELERATED_VISUAL,

  /// OpenGL context major version; see Remarks for details
  ContextMajorVersion = fermium::SDL_GL_CONTEXT_MAJOR_VERSION,

  /// OpenGL context minor version; see Remarks for details
  ContextMinorVersion = fermium::SDL_GL_CONTEXT_MINOR_VERSION,

  /// some combination of 0 or more of elements of the SDL_GLcontextFlag
  /// enumeration; defaults to 0
  ContextFlags = fermium::SDL_GL_CONTEXT_FLAGS,

  /// type of GL context (Core, Compatibility, ES). See SDL_GLprofile; default
  /// value depends on platform
  ContextProfileMask = fermium::SDL_GL_CONTEXT_PROFILE_MASK,

  /// OpenGL context sharing; defaults to 0
  ShareWithCurrentContext = fermium::SDL_GL_SHARE_WITH_CURRENT_CONTEXT,

  /// requests sRGB capable visual; defaults to 0 (>= SDL 2.0.1)
  FramebufferSrgbCapable = fermium::SDL_GL_FRAMEBUFFER_SRGB_CAPABLE,

  /// sets context the release behavior; defaults to 1 (>= SDL 2.0.4)
  ContextReleaseBehavior = fermium::SDL_GL_CONTEXT_RELEASE_BEHAVIOR,
}

/// The swap interval affects the video card's swapping of the video buffer.
#[repr(i32)]
pub enum SwapInterval {
  /// Swap the buffers immediately. Can cause screen tearing.
  Immediate = 0,
  /// Block until the vsync point and swap buffers then.
  Vsync = 1,
  /// Like `Vsync` except if you're only "slightly" past the vsync point it'll
  /// swap right away instead of waiting an entire frame to swap.
  AdaptiveVsync = -1,
}
