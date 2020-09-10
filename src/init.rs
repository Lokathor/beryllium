use core::{
  marker::PhantomData,
  sync::atomic::{AtomicBool, Ordering},
};

use alloc::{string::String, sync::Arc};

use fermium::{SDL_Init, SDL_Quit};

use crate::sdl_get_error;

static SDL_ACTIVE: AtomicBool = AtomicBool::new(false);

pub(crate) struct Initialization(PhantomData<*mut u8>);

impl Drop for Initialization {
  fn drop(&mut self) {
    unsafe { SDL_Quit() }
    SDL_ACTIVE.store(false, Ordering::SeqCst)
  }
}

pub(crate) fn sdl_init(
  flags: InitFlags,
) -> Result<Arc<Initialization>, String> {
  if SDL_ACTIVE.compare_and_swap(false, true, Ordering::SeqCst) {
    // true came back, so SDL was on, so this is a double init.
    Err(String::from("beryllium: SDL is already active!"))
  } else {
    // false came back, so SDL was not on, so we begin normal initialization.
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
      use objc::{class, msg_send, sel, sel_impl};
      let is_main: bool = unsafe { msg_send![class!(NSThread), isMainThread] };
      if !is_main {
        SDL_ACTIVE.store(false, Ordering::SeqCst);
        return Err(String::from(
          "beryllium: SDL must be initialized on the main thread.",
        ));
      }
    }
    let ret = unsafe { SDL_Init(flags.0) };
    if ret < 0 {
      Err(sdl_get_error())
    } else {
      Ok(Arc::new(Initialization(PhantomData)))
    }
  }
}

pub struct InitFlags(u32);
// TODO: init flags
