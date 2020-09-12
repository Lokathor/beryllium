use core::{convert::TryInto, ptr::NonNull};

use alloc::string::String;

use tinyvec::TinyVec;

use fermium::SDL_Surface;

use crate::{sdl_get_error, PixelFormatEnum};

/*
Some day maybe support SDL_CreateRGBSurfaceFrom and SDL_CreateRGBSurfaceWithFormatFrom,
but that would need to be a whole separate type with a lifetime and PhantomData and all that.
*/

pub struct Surface {
  nn: NonNull<SDL_Surface>,
}
impl Drop for Surface {
  fn drop(&mut self) {
    unsafe { fermium::SDL_FreeSurface(self.nn.as_ptr()) }
  }
}
impl Surface {
  pub fn new(
    width: usize, height: usize, bit_depth: usize, r_mask: u32, g_mask: u32,
    b_mask: u32, a_mask: u32,
  ) -> Result<Self, String> {
    NonNull::new(unsafe {
      fermium::SDL_CreateRGBSurface(
        0,
        width.try_into().unwrap(),
        height.try_into().unwrap(),
        bit_depth.try_into().unwrap(),
        r_mask,
        g_mask,
        b_mask,
        a_mask,
      )
    })
    .ok_or_else(sdl_get_error)
    .map(|nn| Surface { nn })
  }

  pub fn new_with_format(
    width: usize, height: usize, bit_depth: usize,
    pixel_format: PixelFormatEnum,
  ) -> Result<Self, String> {
    NonNull::new(unsafe {
      fermium::SDL_CreateRGBSurfaceWithFormat(
        0,
        width.try_into().unwrap(),
        height.try_into().unwrap(),
        bit_depth.try_into().unwrap(),
        pixel_format.0,
      )
    })
    .ok_or_else(sdl_get_error)
    .map(|nn| Surface { nn })
  }

  pub fn load_from_bmp(filename: &str) -> Result<Self, String> {
    let filename_null: TinyVec<[u8; 64]> =
      filename.as_bytes().iter().copied().chain(Some(0)).collect();
    let rw_ops = unsafe {
      fermium::SDL_RWFromFile(
        filename_null.as_ptr().cast(),
        b"rb\0".as_ptr().cast(),
      )
    };
    if rw_ops.is_null() {
      return Err(sdl_get_error());
    } else {
      NonNull::new(unsafe { fermium::SDL_LoadBMP_RW(rw_ops, true as _) })
        .ok_or_else(sdl_get_error)
        .map(|nn| Surface { nn })
    }
  }

  // TODO: this is actually not needed for most surfaces, only ones that have
  // RLE acceleration applied. It's not expensive for other surfaces to do the
  // lock/unlock, but it's not very ergonomic.
  pub fn lock(&mut self) -> Result<SurfaceLock<'_>, String> {
    let ret = unsafe { fermium::SDL_LockSurface(self.nn.as_ptr()) };
    if ret >= 0 {
      Ok(SurfaceLock { surface: self })
    } else {
      Err(sdl_get_error())
    }
  }
}

pub struct SurfaceLock<'s> {
  surface: &'s mut Surface
}
impl<'s> Drop for SurfaceLock<'s> {
  fn drop(&mut self) {
    unsafe { fermium::SDL_UnlockSurface(self.surface.nn.as_ptr()) }
  }
}
