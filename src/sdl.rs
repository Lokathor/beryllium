use super::*;

/// A handle to the SDL API.
///
/// Having one of these is the proof that you've called `SDL_Init`.
///
/// * Most of SDL requires you to have initialized the API.
/// * SDL is generally not thread-safe, and the GUI can only be used from the
///   main thread on Mac, so basically
pub struct SDL {
  #[allow(unused)]
  init_token: Rc<Initialization>,
}
impl SDL {
  /// Initializes SDL with the flags given.
  ///
  /// ## Failure
  ///
  /// * Fails on Mac if you're not on the main thread (according to `NSThread`).
  /// * Fails if SDL is currently initialized.
  /// * Fails if `SDL_Init` fails for whatever reason.
  pub fn init(flags: u32) -> Result<Self, CowStr> {
    Ok(Self { init_token: Rc::new(Initialization::new(flags)?) })
  }

  /// Obtains the current SDL error string.
  ///
  /// In practice it's unlikely that you will need to call this yourself.
  /// Essentially all APIs that can error will call this for you when an error
  /// does happen.
  pub fn get_error(&self) -> String {
    self.init_token.get_error()
  }

  /// Sets an OpenGL attribute to the given value.
  ///
  /// Make all of these calls **before** making your OpenGL-enabled Window.
  ///
  /// The final context that you get might differ from your request. Use the
  /// context's get_attribute method to check the values that you care about.
  ///
  /// ## Failure
  ///
  /// The `SdlGlAttr` will only let you set valid attribute names, but there's
  /// no checking on Rust's part that the value you pass is allowed for that
  /// attribute. If you pass an invalid value SDL will generate an error.
  pub fn gl_set_attribute(
    &self,
    attr: SdlGlAttr,
    value: i32,
  ) -> Result<(), String> {
    let ret = unsafe {
      fermium::SDL_GL_SetAttribute(attr as fermium::SDL_GLattr, value)
    };
    if ret >= 0 {
      Ok(())
    } else {
      Err(self.get_error())
    }
  }
}
