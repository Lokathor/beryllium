use super::*;

/// Attributes that you can use to control OpenGL's loading and context creation
/// process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
pub enum GLattr {
  /// the minimum number of bits for the red channel of the color buffer; defaults to 3
  RedSize = fermium::SDL_GL_RED_SIZE,

  /// the minimum number of bits for the green channel of the color buffer; defaults to 3
  GreenSize = fermium::SDL_GL_GREEN_SIZE,

  /// the minimum number of bits for the blue channel of the color buffer; defaults to 2
  BlueSize = fermium::SDL_GL_BLUE_SIZE,

  /// the minimum number of bits for the alpha channel of the color buffer; defaults to 0
  AlphaSize = fermium::SDL_GL_ALPHA_SIZE,

  /// the minimum number of bits for frame buffer size; defaults to 0
  BufferSize = fermium::SDL_GL_BUFFER_SIZE,

  /// whether the output is single or double buffered; defaults to double buffering on
  DoubleBuffer = fermium::SDL_GL_DOUBLEBUFFER,

  /// the minimum number of bits in the depth buffer; defaults to 16
  DepthSize = fermium::SDL_GL_DEPTH_SIZE,

  /// the minimum number of bits in the stencil buffer; defaults to 0
  StencilSize = fermium::SDL_GL_STENCIL_SIZE,

  /// the minimum number of bits for the red channel of the accumulation buffer; defaults to 0
  AccumRedSize = fermium::SDL_GL_ACCUM_RED_SIZE,

  /// the minimum number of bits for the green channel of the accumulation buffer; defaults to 0
  AccumGreenSize = fermium::SDL_GL_ACCUM_GREEN_SIZE,

  /// the minimum number of bits for the blue channel of the accumulation buffer; defaults to 0
  AccumBlueSize = fermium::SDL_GL_ACCUM_BLUE_SIZE,

  /// the minimum number of bits for the alpha channel of the accumulation buffer; defaults to 0
  AccumAlphaSize = fermium::SDL_GL_ACCUM_ALPHA_SIZE,

  /// whether the output is stereo 3D; defaults to off
  Stereo = fermium::SDL_GL_STEREO,

  /// the number of buffers used for multisample anti-aliasing; defaults to 0; see Remarks for details
  MultisampleBuffers = fermium::SDL_GL_MULTISAMPLEBUFFERS,

  /// the number of samples used around the current pixel used for multisample anti-aliasing; defaults to 0; see Remarks for details
  MultisampleSamples = fermium::SDL_GL_MULTISAMPLESAMPLES,

  /// set to 1 to require hardware acceleration, set to 0 to force software rendering; defaults to allow either
  AcceleratedVisuals = fermium::SDL_GL_ACCELERATED_VISUAL,

  /// OpenGL context major version
  ContextMajorVersion = fermium::SDL_GL_CONTEXT_MAJOR_VERSION,

  /// OpenGL context minor version
  ContextMinorVersion = fermium::SDL_GL_CONTEXT_MINOR_VERSION,

  /// some combination of 0 or more of elements of the SDL_GLContextFlag enumeration; defaults to 0
  ContextFlags = fermium::SDL_GL_CONTEXT_FLAGS,

  /// type of GL context (Core, Compatibility, ES), default value depends on platform
  ContextProfileMask = fermium::SDL_GL_CONTEXT_PROFILE_MASK,

  /// OpenGL context sharing; defaults to 0
  ShareWithCurrentContext = fermium::SDL_GL_SHARE_WITH_CURRENT_CONTEXT,

  /// requests sRGB capable visual; defaults to 0
  FramebufferSRGBCapable = fermium::SDL_GL_FRAMEBUFFER_SRGB_CAPABLE,

  /// sets context the release behavior; defaults to 1
  ContextReleaseBehavior = fermium::SDL_GL_CONTEXT_RELEASE_BEHAVIOR,
}

/// Requests an OpenGL Compatibility context.
pub const CONTEXT_PROFILE_COMPATIBILITY: i32 = fermium::SDL_GL_CONTEXT_PROFILE_COMPATIBILITY as i32;

/// Requests an OpenGL Core context.
pub const CONTEXT_PROFILE_CORE: i32 = fermium::SDL_GL_CONTEXT_PROFILE_CORE as i32;

/// Requests an OpenGL ES context.
pub const CONTEXT_PROFILE_ES: i32 = fermium::SDL_GL_CONTEXT_PROFILE_ES as i32;

/// A flag for use with [`GLattr::ContextFlags`].
///
/// This flag maps to `GLX_CONTEXT_DEBUG_BIT_ARB` in the
/// `GLX_ARB_create_context` extension for X11 and `WGL_CONTEXT_DEBUG_BIT_ARB`
/// in the `WGL_ARB_create_context` extension for Windows. This flag is
/// currently ignored on other targets that don't support equivalent
/// functionality. This flag is intended to put the GL into a "debug" mode which
/// might offer better developer insights, possibly at a loss of performance
/// (although a given GL implementation may or may not do anything differently
/// in the presence of this flag).
pub const CONTEXT_DEBUG_FLAG: i32 = fermium::SDL_GL_CONTEXT_DEBUG_FLAG as i32;

/// A flag for use with [`GLattr::ContextFlags`].
///
/// This flag maps to `GLX_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB` in the
/// `GLX_ARB_create_context` extension for X11 and
/// `WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB` in the `WGL_ARB_create_context`
/// extension for Windows. This flag is currently ignored on other targets that
/// don't support equivalent functionality. This flag is intended to put the GL
/// into a "forward compatible" mode, which means that no deprecated
/// functionality will be supported, possibly at a gain in performance, and only
/// applies to GL 3.0 and later contexts.
pub const CONTEXT_FORWARD_COMPATIBLE_FLAG: i32 =
  fermium::SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG as i32;

/// A flag for use with [`GLattr::ContextFlags`].
///
/// This flag maps to `GLX_CONTEXT_ROBUST_ACCESS_BIT_ARB` in the
/// `GLX_ARB_create_context_robustness` extension for X11 and
/// `WGL_CONTEXT_ROBUST_ACCESS_BIT_ARB` in the `WGL_ARB_create_context_robustness`
/// extension for Windows. This flag is currently ignored on other targets that
/// don't support equivalent functionality. This flag is intended to require a
/// GL context that supports the GL_ARB_robustness extension--a mode that offers
/// a few APIs that are safer than the usual defaults (think `snprintf()` vs
/// `sprintf()`).
pub const CONTEXT_ROBUST_ACCESS_FLAG: i32 = fermium::SDL_GL_CONTEXT_ROBUST_ACCESS_FLAG as i32;

/// A flag for use with [`GLattr::ContextFlags`].
///
/// This flag maps to `GLX_CONTEXT_RESET_ISOLATION_BIT_ARB` in the
/// `GLX_ARB_robustness_isolation` extension for X11 and
/// `WGL_CONTEXT_RESET_ISOLATION_BIT_ARB` in the `WGL_ARB_robustness_isolation`
/// extension for Windows. This flag is currently ignored on other targets that
/// don't support equivalent functionality. This flag is intended to require the
/// GL to make promises about what to do in the face of driver or hardware
/// failure.
pub const CONTEXT_RESET_ISOLATION_FLAG: i32 = fermium::SDL_GL_CONTEXT_RESET_ISOLATION_FLAG as i32;

/// A GLWindow is a [Window] with an OpenGL context bundled in.
///
/// This will [Deref](core::ops::Deref) to the inner window, or you can call
/// OpenGL related methods.
///
/// ## General OpenGL Unsafety
///
/// It's possible to have more than one OpenGL context in the world. All of the
/// `unsafe` methods here require that this context is the current one when
/// calling them. Use [GLWindow::is_current] check and [GLWindow::make_current]
/// if needed. Of course, if you only have a single OpenGL context in your
/// program you'll always have the current one.
#[derive(Debug)]
pub struct GLWindow<'sdl> {
  pub(crate) ctx: fermium::SDL_GLContext,
  pub(crate) window: Window<'sdl>,
}
impl<'sdl> Drop for GLWindow<'sdl> {
  fn drop(&mut self) {
    unsafe { fermium::SDL_GL_DeleteContext(self.ctx) }
  }
}
impl<'sdl> Deref for GLWindow<'sdl> {
  type Target = Window<'sdl>;

  fn deref(&self) -> &Self::Target {
    &self.window
  }
}
impl<'sdl> GLWindow<'sdl> {
  /// Checks if the given extension is available in this context.
  pub unsafe fn extension_supported(&self, name: &str) -> bool {
    let name_null: Vec<u8> = name.bytes().chain(Some(0)).collect();
    fermium::SDL_TRUE == fermium::SDL_GL_ExtensionSupported(name_null.as_ptr() as *const c_char)
  }

  /// Obtains the actual value of the attribute for this context.
  ///
  /// Note that the context you request and the context that you get might not
  /// match, because yay.
  pub unsafe fn get_attribute(&self, attr: GLattr) -> Result<i32, String> {
    let mut output = 0;
    if 0 == fermium::SDL_GL_GetAttribute(attr as fermium::SDL_GLattr, &mut output) {
      Ok(output)
    } else {
      Err(get_error())
    }
  }

  /// Determines the swap interval of the video display.
  ///
  /// * 0: No vsync
  /// * 1: Vsync
  /// * -1: "adaptive vsync", late swaps will happen immediately
  ///
  /// If the swap interval can't be determined this returns 0 as a "safe
  /// default". You can also call [get_error](get_error) to potentially find out
  /// more.
  pub unsafe fn swap_interval(&self) -> i32 {
    fermium::SDL_GL_GetSwapInterval()
  }

  /// Attempts to set the swap interval value.
  ///
  /// The values work as per [swap_interval](GLWindow::swap_interval).
  ///
  /// Note that if you request adaptive vsync and get an error it is likely that
  /// standard vsync might still be available as a fallback.
  pub unsafe fn set_swap_interval(&self, interval: i32) -> Result<(), String> {
    let out = fermium::SDL_GL_SetSwapInterval(interval);
    if out == 0 {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  /// Checks if this context is current.
  pub fn is_current(&self) -> bool {
    let cur = unsafe { fermium::SDL_GL_GetCurrentContext() };
    self.ctx == cur
  }

  /// Makes the given context the current context in this window.
  pub fn make_current(&self) -> Result<(), String> {
    let out = unsafe { fermium::SDL_GL_MakeCurrent(self.window.ptr, self.ctx) };
    if out == 0 {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  /// Obtains the size of the drawable space in the window.
  ///
  /// This gives you a number of "physical pixels", so it might be different
  /// from the "logical pixels" value you get when you call
  /// [size](Window::size). This is primarily for use with
  /// [glViewport](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewport.xhtml)
  pub fn drawable_size(&self) -> (i32, i32) {
    let mut w = 0;
    let mut h = 0;
    unsafe { fermium::SDL_GL_GetDrawableSize(self.window.ptr, &mut w, &mut h) };
    (w, h)
  }

  /// Swaps the window's OpenGL buffers.
  ///
  /// If double buffering isn't enabled this just does nothing.
  pub unsafe fn swap_window(&self) {
    fermium::SDL_GL_SwapWindow(self.window.ptr)
  }
}
