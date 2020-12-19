#![no_std]
#![warn(missing_docs)]

//! Easy to use wrapper of the [`fermium`](https://docs.rs/fermium) crate.

use core::ptr::null_mut;

extern crate alloc;

use alloc::{sync::Arc, vec::Vec};

use fermium::{c_char, prelude::*};

macro_rules! err_guard {
  ($is_err:expr) => {
    if $is_err {
      return Err(crate::get_error());
    }
  };
}

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

fn make_null_str(s: &str) -> Vec<c_char> {
  s.as_bytes().iter().copied().map(|u| u as c_char).chain(Some(0)).collect()
}

mod error;
pub use error::*;

mod init;
pub use init::*;

mod message_box;
pub use message_box::*;

/// Handle to the SDL2 API.
pub struct Sdl(Arc<Init>);

impl Sdl {
  /// Initialize SDL2 according to the flags given.
  ///
  /// * You can't double-initialize SDL2.
  pub fn init(flags: InitFlags) -> BerylliumResult<Self> {
    let init = Init::new(flags)?;
    Ok(Self(Arc::new(init)))
  }
}
