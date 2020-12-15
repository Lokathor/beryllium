use alloc::{boxed::Box, string::String, vec::Vec};

use fermium::error::{SDL_ClearError, SDL_GetError};

/// Error type for the crate.
///
/// SDL2 only offers error strings, so this is just a boxed string.
///
/// The boxing allows for `Result<T, BerylliumError>`, used frequently
/// throughout the crate, to be more space efficient. This in turn makes it more
/// register efficient when it's returned across function boundaries.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BerylliumError(pub Box<String>);
impl core::fmt::Display for BerylliumError {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.0)
  }
}
#[cfg(feature = "std")]
impl std::error::Error for BerylliumError {}

/// The most common result type within this crate.
pub type BerylliumResult<T> = Result<T, BerylliumError>;

/// Gets the thread-local error string.
pub fn get_error_string() -> String {
  let mut v: Vec<u8> = Vec::new();
  unsafe {
    let mut p: *const u8 = SDL_GetError() as *const u8;
    while *p != 0 {
      v.push(*p);
      p = p.offset(1);
    }
  }
  match String::from_utf8(v) {
    Ok(s) => s,
    Err(utf8_error) => {
      String::from_utf8_lossy(utf8_error.as_bytes()).into_owned()
    }
  }
}

/// Clears the thread-local error string.
#[inline]
pub fn clear_error_string() {
  unsafe { SDL_ClearError() }
}

/// Gets the thread-local error as a [`BerylliumError`].
#[inline]
#[must_use]
pub fn get_error() -> BerylliumError {
  BerylliumError(Box::new(get_error_string()))
}
