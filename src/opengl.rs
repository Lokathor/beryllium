use super::*;

/// Attributes that you can use to control OpenGL's loading and context creation
/// process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
pub enum GLattr {
  /// the minimum number of bits for the red channel of the color buffer; defaults to 3
  RedSize = SDL_GL_RED_SIZE,

  /// the minimum number of bits for the green channel of the color buffer; defaults to 3
  GreenSize = SDL_GL_GREEN_SIZE,

  /// the minimum number of bits for the blue channel of the color buffer; defaults to 2
  BlueSize = SDL_GL_BLUE_SIZE,

  /// the minimum number of bits for the alpha channel of the color buffer; defaults to 0
  AlphaSize = SDL_GL_ALPHA_SIZE,

  /// the minimum number of bits for frame buffer size; defaults to 0
  BufferSize = SDL_GL_BUFFER_SIZE,

  /// whether the output is single or double buffered; defaults to double buffering on
  DoubleBuffer = SDL_GL_DOUBLEBUFFER,

  /// the minimum number of bits in the depth buffer; defaults to 16
  DepthSize = SDL_GL_DEPTH_SIZE,

  /// the minimum number of bits in the stencil buffer; defaults to 0
  StencilSize = SDL_GL_STENCIL_SIZE,

  /// the minimum number of bits for the red channel of the accumulation buffer; defaults to 0
  AccumRedSize = SDL_GL_ACCUM_RED_SIZE,

  /// the minimum number of bits for the green channel of the accumulation buffer; defaults to 0
  AccumGreenSize = SDL_GL_ACCUM_GREEN_SIZE,

  /// the minimum number of bits for the blue channel of the accumulation buffer; defaults to 0
  AccumBlueSize = SDL_GL_ACCUM_BLUE_SIZE,

  /// the minimum number of bits for the alpha channel of the accumulation buffer; defaults to 0
  AccumAlphaSize = SDL_GL_ACCUM_ALPHA_SIZE,

  /// whether the output is stereo 3D; defaults to off
  Stereo = SDL_GL_STEREO,

  /// the number of buffers used for multisample anti-aliasing; defaults to 0; see Remarks for details
  MultisampleBuffers = SDL_GL_MULTISAMPLEBUFFERS,

  /// the number of samples used around the current pixel used for multisample anti-aliasing; defaults to 0; see Remarks for details
  MultisampleSamples = SDL_GL_MULTISAMPLESAMPLES,

  /// set to 1 to require hardware acceleration, set to 0 to force software rendering; defaults to allow either
  AcceleratedVisuals = SDL_GL_ACCELERATED_VISUAL,

  /// OpenGL context major version
  ContextMajorVersion = SDL_GL_CONTEXT_MAJOR_VERSION,

  /// OpenGL context minor version
  ContextMinorVersion = SDL_GL_CONTEXT_MINOR_VERSION,

  /// some combination of 0 or more of elements of the SDL_GLcontextFlag enumeration; defaults to 0
  ContextFlags = SDL_GL_CONTEXT_FLAGS,

  /// type of GL context (Core, Compatibility, ES), default value depends on platform
  ContextProfileMask = SDL_GL_CONTEXT_PROFILE_MASK,

  /// OpenGL context sharing; defaults to 0
  ShareWithCurrentContext = SDL_GL_SHARE_WITH_CURRENT_CONTEXT,

  /// requests sRGB capable visual; defaults to 0
  FramebufferSRGBCapable = SDL_GL_FRAMEBUFFER_SRGB_CAPABLE,

  /// sets context the release behavior; defaults to 1
  ContextReleaseBehavior = SDL_GL_CONTEXT_RELEASE_BEHAVIOR,
}

/// Requests an OpenGL Compatibility context.
pub const CONTEXT_PROFILE_COMPATIBILITY: i32 = SDL_GL_CONTEXT_PROFILE_COMPATIBILITY as i32;

/// Requests an OpenGL Core context.
pub const CONTEXT_PROFILE_CORE: i32 = SDL_GL_CONTEXT_PROFILE_CORE as i32;

/// Requests an OpenGL ES context.
pub const CONTEXT_PROFILE_ES: i32 = SDL_GL_CONTEXT_PROFILE_ES as i32;

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
pub const CONTEXT_DEBUG_FLAG: i32 = SDL_GL_CONTEXT_DEBUG_FLAG as i32;

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
pub const CONTEXT_FORWARD_COMPATIBLE_FLAG: i32 = SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG as i32;

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
pub const CONTEXT_ROBUST_ACCESS_FLAG: i32 = SDL_GL_CONTEXT_ROBUST_ACCESS_FLAG as i32;

/// A flag for use with [`GLattr::ContextFlags`].
///
/// This flag maps to `GLX_CONTEXT_RESET_ISOLATION_BIT_ARB` in the
/// `GLX_ARB_robustness_isolation` extension for X11 and
/// `WGL_CONTEXT_RESET_ISOLATION_BIT_ARB` in the `WGL_ARB_robustness_isolation`
/// extension for Windows. This flag is currently ignored on other targets that
/// don't support equivalent functionality. This flag is intended to require the
/// GL to make promises about what to do in the face of driver or hardware
/// failure.
pub const CONTEXT_RESET_ISOLATION_FLAG: i32 = SDL_GL_CONTEXT_RESET_ISOLATION_FLAG as i32;

/// Handle for an OpenGL context.
///
/// # General Safety
///
/// The context must be current when you call any method here.
#[derive(Debug)]
#[repr(transparent)]
pub struct GLContext<'sdl, 'win> {
  pub(crate) ctx: SDL_GLContext,
  pub(crate) _marker: PhantomData<&'win Window<'sdl>>,
}
impl<'sdl, 'win> Drop for GLContext<'sdl, 'win> {
  fn drop(&mut self) {
    unsafe { SDL_GL_DeleteContext(self.ctx) }
  }
}
impl<'sdl, 'win> GLContext<'sdl, 'win> {
  /// Checks if the given extension is available in this context.
  pub unsafe fn extension_supported(&self, name: &str) -> bool {
    let name_null: Vec<u8> = name.bytes().chain(Some(0)).collect();
    SDL_TRUE == SDL_GL_ExtensionSupported(name_null.as_ptr() as *const c_char)
  }

  /// Obtains the actual value of the attribute for this context.
  ///
  /// Note that the context you request and the context that you get might not
  /// match, because yay.
  pub unsafe fn get_attribute(&self, attr: GLattr) -> Result<i32, String> {
    let mut output = 0;
    if 0 == SDL_GL_GetAttribute(attr as fermium::SDL_GLattr::Type, &mut output) {
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
    SDL_GL_GetSwapInterval()
  }

  /// Attempts to set the swap interval value.
  ///
  /// The values work as per [swap_interval](GLContext::swap_interval).
  ///
  /// Note that if you request adaptive vsync and get an error it is likely that
  /// standard vsync might still be available as a fallback.
  pub unsafe fn set_swap_interval(&self, interval: i32) -> Result<(), String> {
    let out = SDL_GL_SetSwapInterval(interval);
    if out == 0 {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  /// Checks if this context is current.
  pub fn is_current(&self) -> bool {
    let cur = unsafe { SDL_GL_GetCurrentContext() };
    self.ctx == cur
  }
}
