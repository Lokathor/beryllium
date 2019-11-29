#![cfg_attr(not(any(target_os = "macos", target_os = "ios")), no_std)]
#![warn(missing_docs)]
#![allow(unused_imports)]

//! A wrapper lib for SDL2, hereafter referred to as just "SDL" for simplicity.
//!
//! The bindings themselves are provided by
//! [`fermium`](https://docs.rs/fermium), this crate attempts to make it safe
//! and easy to use from Rust.
//!
//! ## Restrictions
//!
//! * The library is very incomplete
//! * The library only lets you have a single window because that's all that I
//!   personally need and it's a lot easier to make all the safety stuff work
//!   out that way. If you want more than one window you can fork this and try
//!   to figure all that out.
//!
//! ## `no_std` Support
//!
//! Yes on Win/Linux, no on Mac. On Windows and Linux you can start SDL from any
//! thread as long as you stick to controlling it from just that thread. On Mac
//! you _must_ start SDL from the main thread, which this library checks, which
//! requires the standard library because of how the `objc` crate is written.

/*

Current TODO:

?

STRETCH GOALS:

SDL_WasInit
message boxes?
window flags newtype
mouse button enum
KeySym modifiers newtype
MouseMotionEvent.state newtype
SDL_ShowCursor
SDL_WarpMouseInWindow
SDL_WarpMouseGlobal

Ideas:

struct CStrFormatter(*const c_char);
impl CStrFormatter { pub unsafe fn new(ptr: *const c_char) -> Self { .. }
impl core::fmt::Debug for CStrFormatter { ... }

NEXT FERMIUM:

Expose `SDL_GetErrorMsg`, which is apparently thread safe?

*/

pub use fermium;
pub(crate) use fermium::{c_char, c_void};

extern crate alloc;
use alloc::{borrow::Cow, format, string::String, sync::Arc, vec, vec::Vec};

use core::{
  convert::TryFrom,
  marker::PhantomData,
  mem::ManuallyDrop,
  ptr::null_mut,
  sync::atomic::{AtomicBool, Ordering},
};

// Declaration MUST stay before all modules because Rust is stupid.
macro_rules! cow_str {
  ($l:literal) => {
    alloc::borrow::Cow::Borrowed($l)
  };
  ($i:ident) => {
    alloc::borrow::Cow::Borrowed($i)
  };
  ($($tokens:tt)*) => {
    alloc::borrow::Cow::Owned(format!($($tokens)*))
  };
}

mod initialization;
pub use initialization::InitFlags;
pub(crate) use initialization::*;
mod sdl;
pub use sdl::*;
mod window;
pub use window::*;
mod event;
pub use event::*;
mod audio;
pub use audio::*;
mod controller;
pub use controller::*;

/// Clone On Write, specific to `&str` and `String`.
///
/// Used where possible to save on allocations.
pub type CowStr = Cow<'static, str>;

trait StrExt {
  fn alloc_c_str(&self) -> Vec<c_char>;
}
impl StrExt for str {
  fn alloc_c_str(&self) -> Vec<c_char> {
    self
      .bytes()
      .map(|c| c as c_char)
      .take_while(|&c| c != 0)
      .chain(Some(0))
      .collect()
  }
}

/// Obtains the current SDL error message.
///
/// ## Safety
///
/// * This is an unsynchronized global. Data races and such.
/// * For the safe version see [`SDL::get_error`].
pub unsafe fn get_error_unchecked() -> String {
  // SDL_ERRBUFIZE is 1024
  let mut v = Vec::with_capacity(1024);
  let mut err_p = fermium::SDL_GetError();
  while *err_p != 0 {
    v.push(*err_p as u8);
    err_p = err_p.offset(1);
  }
  String::from_utf8_lossy(&v).into_owned()
}
