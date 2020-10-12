use core::ptr::NonNull;

use fermium::{SDL_Palette, SDL_PixelFormat};

use crate::{sdl_get_error, Palette, PixelFormatEnum, SdlError};

/// Information about a pixel format.
///
/// Internally these are ref counted and usually handed out from a pool that SDL
/// manages. As a result, they're generally read-only. The only exception is
/// that you can change the *content* (but not *size*) of a palette associated
/// with a pixel format (each palette pixel format allocates a separate palette
/// when created).
#[repr(transparent)]
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

  pub fn pixel_format_enum(&self) -> PixelFormatEnum {
    PixelFormatEnum(unsafe { (*self.nn.as_ptr()).format })
  }

  pub fn palette(&self) -> &Option<Palette> {
    unsafe {
      let p: *const SDL_Palette = (*self.nn.as_ptr()).palette;
      core::mem::transmute::<&*const SDL_Palette, &Option<Palette>>(&p)
    }
  }

  pub fn bits_per_pixel(&self) -> usize {
    unsafe { (*self.nn.as_ptr()).BitsPerPixel as usize }
  }

  pub fn bytes_per_pixel(&self) -> usize {
    unsafe { (*self.nn.as_ptr()).BytesPerPixel as usize }
  }

  /// 0 for palette formats.
  pub fn r_mask(&self) -> u32 {
    unsafe { (*self.nn.as_ptr()).Rmask }
  }
  /// 0 for palette formats.
  pub fn g_mask(&self) -> u32 {
    unsafe { (*self.nn.as_ptr()).Gmask }
  }
  /// 0 for palette formats.
  pub fn b_mask(&self) -> u32 {
    unsafe { (*self.nn.as_ptr()).Bmask }
  }
  /// 0 for palette formats or for formats without alpha.
  pub fn a_mask(&self) -> u32 {
    unsafe { (*self.nn.as_ptr()).Amask }
  }
}
