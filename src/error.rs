use alloc::{
  boxed::Box,
  string::{String, ToString},
  vec::Vec,
};
use fermium::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[allow(clippy::box_collection)]
pub struct SdlError(Box<String>);
impl SdlError {
  #[inline]
  pub fn new(s: &str) -> Self {
    Self(Box::new(s.to_string()))
  }
}

#[inline]
pub fn get_error() -> SdlError {
  unsafe {
    let mut v: Vec<u8> = Vec::with_capacity(1024);
    let capacity = v.capacity();
    SDL_GetErrorMsg(v.as_mut_ptr().cast(), capacity.try_into().unwrap());
    let mut len = 0;
    let mut p = v.as_mut_ptr();
    while *p != 0 && len <= capacity {
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
