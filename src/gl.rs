use super::*;

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
