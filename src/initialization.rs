use super::*;

static I_THINK_THAT_SDL_IS_ACTIVE: AtomicBool = AtomicBool::new(false);
#[derive(Debug, PartialEq)]
#[repr(C)]
pub(crate) struct Initialization(PhantomData<*mut ()>);
impl Initialization {
  pub fn new(flags: u32) -> Result<Self, CowStr> {
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
  pub fn get_error(&self) -> String {
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
