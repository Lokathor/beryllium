use core::{
  marker::PhantomData,
  sync::atomic::{AtomicBool, Ordering},
};

use alloc::sync::Arc;
use fermium::prelude::*;

use crate::error::{get_error, SdlError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct InitFlags(SDL_InitFlags);
impl InitFlags {
  pub const AUDIO: Self = Self(SDL_INIT_AUDIO);
  pub const EVENTS: Self = Self(SDL_INIT_EVENTS);
  pub const EVERYTHING: Self = Self(SDL_INIT_EVERYTHING);
  pub const GAMECONTROLLER: Self = Self(SDL_INIT_GAMECONTROLLER);
  pub const HAPTIC: Self = Self(SDL_INIT_HAPTIC);
  pub const JOYSTICK: Self = Self(SDL_INIT_JOYSTICK);
  pub const SENSOR: Self = Self(SDL_INIT_SENSOR);
  pub const TIMER: Self = Self(SDL_INIT_TIMER);
  pub const VIDEO: Self = Self(SDL_INIT_VIDEO);
}
impl core::ops::BitOr for InitFlags {
  type Output = Self;
  #[inline]
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}

static SDL_IS_ACTIVE: AtomicBool = AtomicBool::new(false);

#[repr(transparent)]
pub(crate) struct SdlInit(PhantomData<*mut ()>);
impl SdlInit {
  #[inline]
  pub fn try_new_arc(flags: InitFlags) -> Result<Arc<Self>, SdlError> {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
      use objc::{class, msg_send, sel, sel_impl};
      let is_main: bool = unsafe { msg_send![class!(NSThread), isMainThread] };
      if !is_main {
        return Err(SdlError::new("beryllium: can only be init on the main thread."));
      }
    }
    match SDL_IS_ACTIVE.compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire) {
      Ok(_) => {
        let ret = unsafe { SDL_Init(flags.0) };
        if ret == 0 {
          #[allow(clippy::arc_with_non_send_sync)]
          Ok(Arc::new(Self(PhantomData)))
        } else {
          Err(get_error())
        }
      }
      Err(_) => Err(SdlError::new("beryllium: Double initialization.")),
    }
  }
}
impl Drop for SdlInit {
  #[inline]
  fn drop(&mut self) {
    unsafe { SDL_Quit() }
    SDL_IS_ACTIVE.store(false, Ordering::Release);
  }
}
