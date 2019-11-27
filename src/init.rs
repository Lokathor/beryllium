
use super::*;

static I_THINK_THAT_SDL_IS_ACTIVE: AtomicBool = AtomicBool::new(false);
#[derive(Debug, PartialEq)]
#[repr(C)]
struct Initialization(PhantomData<*mut ()>);
impl Initialization {
  fn new(flags: u32) -> Result<Self, CowStr> {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
      use objc::{class, msg_send, sel, sel_impl};
      let is_main: bool = unsafe { msg_send![class!(NSThread), isMainThread] };
      if !is_main {
        return Err(cow_str!("SDL must be initialized on the main thread."));
      }
    }
    if I_THINK_THAT_SDL_IS_ACTIVE.swap(true, Ordering::SeqCst) {
      // Reminder: swap gives the old value back.
      Err(cow_str!("SDL is currently initialized."))
    } else if unsafe { fermium::SDL_Init(flags) } == 0 {
      Ok(Initialization(PhantomData))
    } else {
      // Safety: Because we hold the Atomic right now no one else can be safely
      // reading the buffer.
      let out = unsafe { get_error_unchecked() };
      I_THINK_THAT_SDL_IS_ACTIVE.store(false, Ordering::SeqCst);
      Err(Cow::Owned(out))
    }
  }

  /// Gets the current SDL error message.
  fn get_error(&self) -> String {
    // Safety: Because you have &self, and Initialization isn't Send, no one
    // else can safely be accessing the error buffer right now.
    unsafe { get_error_unchecked() }
  }
}
impl Drop for Initialization {
  fn drop(&mut self) {
    unsafe { fermium::SDL_Quit() }
    I_THINK_THAT_SDL_IS_ACTIVE.store(false, Ordering::SeqCst);
  }
}

/// A handle to the SDL API.
///
/// Having one of these is the proof that you've called `SDL_Init`.
///
/// * Most of SDL requires you to have initialized the API.
/// * SDL is generally not thread-safe, and the GUI can only be used from the
///   main thread on Mac, so basically 
pub struct SDL {
  #[allow(unused)]
  init_token: Rc<Initialization>
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
    Ok(Self {
      init_token: Rc::new(Initialization::new(flags)?)
    })
  }

  /// Obtains the current SDL error string.
  pub fn get_error(&self) -> String {
    self.init_token.get_error()
  }
}
