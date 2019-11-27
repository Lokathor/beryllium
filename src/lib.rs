#![cfg_attr(not(any(target_os = "macos", target_os = "ios")), no_std)]
#![warn(missing_docs)]
#![allow(unused_imports)]

//! A wrapper lib for SDL2, hereafter referred to as just "SDL" for simplicity.
//!
//! The bindings themselves are provided by
//! [`fermium`](https://docs.rs/fermium), this crate attempts to make it safe
//! and easy to use from Rust.

/*

TODO items for even a basic experience:

Open a Window with OpenGL support (should be a fused op).
Poll Events
Swap The Window Buffer

STRETCH GOALS:

keyboard input
mouse input
joystick / controller
sound
Message Box

*/

pub use fermium;

extern crate alloc;
use alloc::{borrow::Cow, format, rc::Rc, string::String, vec, vec::Vec};

use core::{
  marker::PhantomData,
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

/// Clone On Write, specific to `&str` and `String`.
///
/// Used where possible to save on allocations.
pub type CowStr = Cow<'static, str>;

mod initialization;
pub(crate) use initialization::*;

mod sdl;
pub use sdl::*;
mod gl;
pub use gl::*;

/// Obtains the current SDL error message.
///
/// ## Safety
///
/// * If you call this from a thread that **doesn't** have the SDL token while
///   SDL is active it's possible to be reading the error buffer while the other
///   thread is causing an error and writing to the buffer (data race).
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
