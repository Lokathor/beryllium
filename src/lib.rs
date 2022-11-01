#![no_std]
#![allow(dead_code)]
#![warn(clippy::missing_inline_in_public_items)]

extern crate alloc;

use core::{
  marker::PhantomData,
  sync::atomic::{AtomicBool, Ordering},
};

use alloc::{
  boxed::Box,
  string::{String, ToString},
  sync::Arc,
  vec::Vec,
};
use fermium::prelude::*;

#[repr(transparent)]
#[allow(clippy::box_collection)]
pub struct SdlError(Box<String>);
impl SdlError {
  #[inline]
  pub fn new(s: &str) -> Self {
    Self(Box::new(s.to_string()))
  }
}

fn get_error() -> SdlError {
  unsafe {
    let mut v: Vec<u8> = Vec::with_capacity(1024);
    let capacity = v.capacity();
    SDL_GetErrorMsg(v.as_mut_ptr().cast(), capacity.try_into().unwrap());
    let mut len = 0;
    let mut p = v.as_mut_ptr();
    while *p != 0 {
      p = p.add(1);
      len += 1;
    }
    v.set_len(len);
    match String::from_utf8(v) {
      Ok(s) => SdlError(Box::new(s)),
      Err(e) => SdlError(Box::new(String::from_utf8_lossy(e.as_bytes()).into_owned())),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct InitFlags(u32);

static SDL_IS_ACTIVE: AtomicBool = AtomicBool::new(false);

#[repr(transparent)]
pub struct SdlInit(PhantomData<*mut ()>);
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
        let ret = unsafe { SDL_Init(SDL_InitFlags(flags.0)) };
        if ret == 0 {
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
