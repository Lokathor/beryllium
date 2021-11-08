#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

use alloc::{boxed::Box, string::String, vec::Vec};
use fermium::{
  audio::SDL_GetNumAudioDevices,
  prelude::{SDL_EventState, SDL_GetTicks, SDL_DISABLE, SDL_DROPFILE, SDL_DROPTEXT, SDL_ENABLE},
};
use init::Sdl;

macro_rules! impl_bit_ops_for_tuple_newtype {
  ($t:ty) => {
    impl core::ops::BitAnd for $t {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
      }
    }
    impl core::ops::BitAndAssign for $t {
      #[inline]
      #[must_use]
      fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
      }
    }
    impl core::ops::BitOr for $t {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
      }
    }
    impl core::ops::BitOrAssign for $t {
      #[inline]
      #[must_use]
      fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
      }
    }
    impl core::ops::BitXor for $t {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
      }
    }
    impl core::ops::BitXorAssign for $t {
      #[inline]
      #[must_use]
      fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
      }
    }
    impl core::ops::Not for $t {
      type Output = Self;
      #[inline]
      #[must_use]
      fn not(self) -> Self::Output {
        Self(!self.0)
      }
    }
  };
}

pub mod controller;
pub mod event;
pub mod init;
pub mod joystick;
pub mod window;

/// Converts a `Vec<u8>` into a `String` using the minimum amount of
/// re-allocation.
#[inline]
fn min_alloc_lossy_into_string(bytes: Vec<u8>) -> String {
  match String::from_utf8(bytes) {
    Ok(s) => s,
    Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
  }
}

/// Gets the current (thread local) SDL2 error message.
///
/// You shouldn't need to call this, but you can I guess.
#[inline]
pub fn get_error() -> SdlError {
  use fermium::error::SDL_GetErrorMsg;

  let mut v: Vec<u8> = Vec::with_capacity(1024);
  let mut p = v.as_mut_ptr();
  unsafe { SDL_GetErrorMsg(p.cast(), v.capacity().try_into().unwrap()) };
  let mut len = 0;
  while unsafe { *p } != 0 {
    len += 1;
    p = unsafe { p.add(1) };
  }
  unsafe { v.set_len(len) };
  SdlError(Box::new(min_alloc_lossy_into_string(v)))
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct SdlError(Box<String>);
impl core::fmt::Display for SdlError {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    core::fmt::Display::fmt(self.0.as_str(), f)
  }
}
#[cfg(feature = "std")]
impl std::error::Error for SdlError {}

pub type SdlResult<T> = Result<T, SdlError>;

impl Sdl {
  #[inline]
  pub fn get_ticks(&self) -> u32 {
    unsafe { SDL_GetTicks() }
  }

  /// Attempts to get the number of audio devices available.
  ///
  /// * `is_capture`: if recording devices should be checked for (otherwise
  ///   playback devices are checked for).
  ///
  /// If an explicit list is not available -1 will be returned. This is not
  /// necessarily an error. Various audio devices, particularly remote devices,
  /// might still be available.
  #[inline]
  pub fn get_num_audio_devices(&self, is_capture: bool) -> i32 {
    unsafe { SDL_GetNumAudioDevices(is_capture as _) }
  }

  #[inline]
  pub fn allow_drop_events(&self, enabled: bool) {
    unsafe {
      SDL_EventState(SDL_DROPFILE, if enabled { SDL_ENABLE } else { SDL_DISABLE });
      SDL_EventState(SDL_DROPTEXT, if enabled { SDL_ENABLE } else { SDL_DISABLE });
    }
  }
}
