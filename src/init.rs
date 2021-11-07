use alloc::{boxed::Box, string::String, sync::Arc};
use core::{
  marker::PhantomData,
  sync::atomic::{AtomicBool, Ordering},
};
use fermium::{
  SDL_Init, SDL_InitFlags, SDL_Quit, SDL_INIT_AUDIO, SDL_INIT_EVENTS, SDL_INIT_EVERYTHING,
  SDL_INIT_GAMECONTROLLER, SDL_INIT_HAPTIC, SDL_INIT_JOYSTICK, SDL_INIT_SENSOR, SDL_INIT_TIMER,
  SDL_INIT_VIDEO,
};

use crate::{SdlError, SdlResult};

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct InitFlags(SDL_InitFlags);
impl_bit_ops_for_tuple_newtype!(InitFlags);
impl InitFlags {
  pub const AUDIO: Self = Self(SDL_INIT_AUDIO);
  pub const EVENTS: Self = Self(SDL_INIT_EVENTS);
  pub const EVERYTHING: Self = Self(SDL_INIT_EVERYTHING);
  pub const CONTROLLER: Self = Self(SDL_INIT_GAMECONTROLLER);
  pub const HAPTIC: Self = Self(SDL_INIT_HAPTIC);
  pub const JOYSTICK: Self = Self(SDL_INIT_JOYSTICK);
  pub const SENSOR: Self = Self(SDL_INIT_SENSOR);
  pub const TIMER: Self = Self(SDL_INIT_TIMER);
  pub const VIDEO: Self = Self(SDL_INIT_VIDEO);
}

static SDL_IS_INIT: AtomicBool = AtomicBool::new(false);

pub(crate) struct Initialization {
  _no_send_or_sync: PhantomData<*mut ()>,
}
impl Initialization {
  fn try_init(flags: InitFlags) -> SdlResult<Self> {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
      use objc::{class, msg_send, sel, sel_impl};
      let is_main: bool = unsafe { msg_send![class!(NSThread), isMainThread] };
      if !is_main {
        return Err(SdlError(Box::new(String::from(
          "beryllium: SDL can only be initialized on the main thread.",
        ))));
      }
    }
    match SDL_IS_INIT.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
      Ok(_) => {
        let ret = unsafe { SDL_Init(flags.0) };
        if ret == 0 {
          Ok(Self { _no_send_or_sync: PhantomData })
        } else {
          Err(crate::get_error())
        }
      }
      Err(_) => Err(SdlError(Box::new(String::from("beryllium: SDL2 is already initialized")))),
    }
  }
}
impl Drop for Initialization {
  fn drop(&mut self) {
    match SDL_IS_INIT.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst) {
      Ok(_) => unsafe { SDL_Quit() },
      Err(_) => panic!("beryllium: SDL tried to quit when it already wasn't initialized."),
    }
  }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct Sdl(Arc<Initialization>);

impl Sdl {
  pub fn init(flags: InitFlags) -> SdlResult<Self> {
    Initialization::try_init(flags).map(|i| Self(Arc::new(i)))
  }
}
