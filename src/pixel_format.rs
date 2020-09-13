use core::ptr::NonNull;

use fermium::SDL_PixelFormat;

use crate::{sdl_get_error, PixelFormatEnum, SdlError};

pub struct PixelFormat {
  nn: NonNull<SDL_PixelFormat>,
}
impl Drop for PixelFormat {
  fn drop(&mut self) {
    unsafe { fermium::SDL_FreeFormat(self.nn.as_ptr()) }
  }
}
impl PixelFormat {
  pub fn new(format: PixelFormatEnum) -> Result<Self, SdlError> {
    NonNull::new(unsafe { fermium::SDL_AllocFormat(format.0) })
      .ok_or_else(sdl_get_error)
      .map(|nn| PixelFormat { nn })
  }
}
