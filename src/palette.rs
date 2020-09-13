use core::{convert::TryInto, ptr::NonNull};

use fermium::SDL_Palette;

use crate::{sdl_get_error, SdlError};

pub struct Palette {
  nn: NonNull<SDL_Palette>,
}
impl Drop for Palette {
  fn drop(&mut self) {
    unsafe { fermium::SDL_FreePalette(self.nn.as_ptr()) }
  }
}
impl Palette {
  pub fn new(num_colors: usize) -> Result<Self, SdlError> {
    NonNull::new(unsafe {
      fermium::SDL_AllocPalette(num_colors.try_into().unwrap_or(i32::MAX))
    })
    .ok_or_else(sdl_get_error)
    .map(|nn| Palette { nn })
  }

  pub fn len(&self) -> usize {
    unsafe { (*self.nn.as_ptr()).ncolors as usize }
  }

  pub fn get_colors(&self, buf: &mut [[u8; 4]]) {
    let len = self.len();
    let buf = &mut buf[..len];
    let src = unsafe {
      core::slice::from_raw_parts((*self.nn.as_ptr()).colors.cast(), len)
    };
    buf.copy_from_slice(src);
  }

  pub fn set_colors(
    &self, buf: &[[u8; 4]], offset: usize,
  ) -> Result<(), SdlError> {
    let ret = unsafe {
      fermium::SDL_SetPaletteColors(
        self.nn.as_ptr(),
        buf.as_ptr().cast(),
        offset.try_into().unwrap(),
        buf.len().try_into().unwrap(),
      )
    };
    if ret >= 0 {
      Ok(())
    } else {
      Err(sdl_get_error())
    }
  }
}
