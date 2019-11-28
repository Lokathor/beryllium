use super::*;

pub(crate) static WINDOW_EXISTS: AtomicBool = AtomicBool::new(false);

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
  pub fn init(flags: InitFlags) -> Result<Self, CowStr> {
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
  /// The final context that you get might differ from your request. Use
  /// [`gl_get_attribute`](SDL::gl_get_attribute) after you've made your
  /// [`GLWindow`] to examine your actual context.
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

  /// Resets all GL Attributes to their default values.
  pub fn gl_reset_attributes(&self) {
    unsafe { fermium::SDL_GL_ResetAttributes() }
  }

  /// Makes a window with a GL context in one step.
  ///
  /// beryllium currently only allows one window
  pub fn create_gl_window(
    &self,
    title: &str,
    pos: WindowPosition,
    width: u32,
    height: u32,
    flags: u32,
  ) -> Result<GlWindow, String> {
    if WINDOW_EXISTS.swap(true, Ordering::SeqCst) {
      Err(String::from("beryllium: There's already a window!"))
    } else {
      // make a window
      let title_null = title.alloc_c_str();
      let (x, y) = pos.what_sdl_wants();
      let win = unsafe {
        fermium::SDL_CreateWindow(
          title_null.as_ptr(),
          x,
          y,
          width as i32,
          height as i32,
          flags | (fermium::SDL_WINDOW_OPENGL as u32),
        )
      };
      if win.is_null() {
        return Err(self.get_error());
      }
      // now it'll drop
      let win = Window { win };

      // make a context
      let ctx = unsafe { fermium::SDL_GL_CreateContext(win.win) };
      if ctx.is_null() {
        return Err(self.get_error());
      }
      // now it'll drop
      let ctx = GlContext { ctx };

      Ok(GlWindow {
        init_token: self.init_token.clone(),
        win: ManuallyDrop::new(win),
        ctx: ManuallyDrop::new(ctx),
      })
    }
  }

  /// Blocks for at least `ms` milliseconds before returning.
  ///
  /// It might be longer than that because of OS scheduling.
  pub fn delay_ms(&self, ms: u32) {
    unsafe { fermium::SDL_Delay(ms) }
  }

  /// Blocks for the given [`Duration`](core::time::Duration) with millisecond
  /// granularity.
  /// 
  /// If the duration is more than `u32::max_value()` milliseconds.
  /// 
  /// 1) Seriously, what the hell? Are you okay friend? Sleeping that much?
  /// 2) It uses more than one sleep in a loop because you do you.
  pub fn delay_duration(&self, duration: core::time::Duration) {
    let mut ms_remaining = duration.as_millis();
    const TIME_CHUNK: u128 = u32::max_value() as u128;
    while ms_remaining > TIME_CHUNK {
      unsafe { fermium::SDL_Delay(TIME_CHUNK as u32) }
      ms_remaining -= TIME_CHUNK;
    }
    self.delay_ms(ms_remaining as u32)
  }
}
