#![no_std]
#![allow(unused_imports)]

extern crate alloc;
use alloc::{boxed::Box, string::String, vec::Vec};

#[cfg(any(target_os = "macos", target_os = "ios", feature = "std"))]
extern crate std;

mod sdl;
pub use sdl::*;

mod event;
pub use event::*;

mod window;
pub use window::*;

mod renderer;
pub use renderer::*;

mod surface;
pub use surface::*;

mod pixel_format_enum;
pub use pixel_format_enum::*;

mod pixel_format;
pub use pixel_format::*;

mod palette;
pub use palette::*;

mod texture;
pub use texture::*;

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

/// An error string from SDL.
#[derive(Debug, Default)]
pub struct SdlError(
  // You  may not like it, but this is what peak performance looks like.
  Box<String>,
);
impl core::fmt::Display for SdlError {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.0)
  }
}
#[cfg(feature = "std")]
impl std::error::Error for SdlError {}

/// Gets the current SDL error string of this thread.
pub(crate) fn sdl_get_error() -> SdlError {
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
    SdlError(Box::new(bytes_to_string(buf)))
  }
}

/// Gets the SDL version that the program is actually using.
///
/// This *might* be a later version than the one you compiled against. However,
/// SDL's dynamic loading system will not allow and ABI-incompatible version to
/// be loaded.
pub fn sdl_get_version() -> (u8, u8, u8) {
  let mut version = fermium::SDL_version::default();
  unsafe { fermium::SDL_GetVersion(&mut version) };
  (version.major, version.minor, version.patch)
}
