#![no_std]
#![warn(missing_docs)]

//! A wrapper lib for SDL2, hereafter referred to as just "SDL" for simplicity. 
//!
//! The bindings themselves are provided by `fermium`, this crate attempts to
//! make it safe and easy to use from Rust.

pub use fermium;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use alloc::format;
use alloc::string::String;
use alloc::borrow::Cow;
use alloc::rc::Rc;

use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, Ordering};

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

mod init;
pub use init::*;

/// Obtains the current SDL error message.
///
/// ## Safety
///
/// * If you call this from a thread that **doesn't** have the SDL token while
///   SDL is active it's possible to be reading the error buffer while the other
///   thread is causing an error and writing to the buffer (data race).
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
