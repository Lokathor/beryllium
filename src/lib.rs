#![no_std]
#![allow(unused_imports)]

extern crate alloc;
use alloc::{string::String, vec::Vec};

#[cfg(any(target_os = "macos", target_os = "ios"))]
extern crate std;

mod sdl;
pub use sdl::*;

mod event;
pub use event::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WindowID(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct MouseID(u32);
impl MouseID {
  pub const fn is_touch_mouse(self) -> bool {
    self.0 == fermium::SDL_TOUCH_MOUSEID
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct MouseButtonState(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct JoystickID(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct AudioDeviceID(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TouchID(i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FingerID(i64);

/// Attempts to make some bytes into a string without allocating.
///
/// On error, this falls back to lossy allocating.
fn bytes_to_string(v: Vec<u8>) -> String {
  match String::from_utf8(v) {
    Ok(s) => s,
    Err(from_utf8_error) => {
      let bytes = from_utf8_error.as_bytes();
      let cow = String::from_utf8_lossy(bytes);
      cow.into_owned()
    }
  }
}

/// Gets the current SDL error string of this thread.
pub(crate) fn sdl_get_error() -> String {
  /// This is the size of the TLS error buffer in current SDL, so we will
  /// pre-allocate this much to save time. If the error buffer size grows in the
  /// future then our vec will just realloc on long strings.
  const ERR_MAX_STRLEN: usize = 128;
  unsafe {
    let mut buf = Vec::with_capacity(ERR_MAX_STRLEN);
    let mut p: *const u8 = fermium::SDL_GetError() as _;
    while *p != 0 {
      buf.push(*p);
      p = p.add(1);
    }
    bytes_to_string(buf)
  }
}

pub fn sdl_get_version() -> (u8, u8, u8) {
  use fermium::{SDL_GetVersion, SDL_version};
  let mut version = SDL_version::default();
  unsafe { SDL_GetVersion(&mut version) };
  (version.major, version.minor, version.patch)
}
