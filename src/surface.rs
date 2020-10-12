use core::{
  convert::TryInto,
  ops::{Deref, Index, IndexMut},
  ptr::NonNull,
};

use tinyvec::TinyVec;

use fermium::{SDL_PixelFormat, SDL_Surface};

use crate::{sdl_get_error, PixelFormat, PixelFormatEnum, SdlError};

/*
Some day maybe support SDL_CreateRGBSurfaceFrom and SDL_CreateRGBSurfaceWithFormatFrom,
but that would need to be a whole separate type with a lifetime and PhantomData and all that.
*/

pub struct Surface {
  pub(crate) nn: NonNull<SDL_Surface>,
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
  ) -> Result<Self, SdlError> {
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
  ) -> Result<Self, SdlError> {
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

  pub fn load_from_bmp(filename: &str) -> Result<Self, SdlError> {
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
  pub fn lock(&mut self) -> Result<SurfaceLock<'_>, SdlError> {
    let ret = unsafe { fermium::SDL_LockSurface(self.nn.as_ptr()) };
    if ret >= 0 {
      Ok(SurfaceLock { surface: self })
    } else {
      Err(sdl_get_error())
    }
  }

  pub fn pixel_format(&self) -> &PixelFormat {
    unsafe {
      let f: *const SDL_PixelFormat = (*self.nn.as_ptr()).format;
      assert!(!f.is_null());
      core::mem::transmute::<&*const SDL_PixelFormat, &PixelFormat>(&f)
    }
  }

  /// Width in pixels
  pub fn width(&self) -> usize {
    unsafe { (*self.nn.as_ptr()).w as usize }
  }
  /// Height in pixels
  pub fn height(&self) -> usize {
    unsafe { (*self.nn.as_ptr()).h as usize }
  }
  /// Pitch between row starts, in bytes.
  pub fn pitch(&self) -> isize {
    unsafe { (*self.nn.as_ptr()).h as isize }
  }
}

pub struct SurfaceLock<'s> {
  surface: &'s mut Surface,
}
impl<'s> Drop for SurfaceLock<'s> {
  fn drop(&mut self) {
    unsafe { fermium::SDL_UnlockSurface(self.surface.nn.as_ptr()) }
  }
}
impl<'s> Deref for SurfaceLock<'s> {
  type Target = Surface;
  fn deref(&self) -> &Self::Target {
    &self.surface
  }
}
impl<'s> Index<(usize, usize)> for SurfaceLock<'s> {
  type Output = [u8];
  fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
    unsafe {
      assert!(x < self.width());
      assert!(y < self.height());
      let bytes_per_pixel = self.pixel_format().bytes_per_pixel();
      let base = self.pixels();
      let row_start = base.offset(self.pitch() * (y as isize));
      let pixel = row_start.add(x * bytes_per_pixel);
      core::slice::from_raw_parts(pixel, bytes_per_pixel)
    }
  }
}
impl<'s> IndexMut<(usize, usize)> for SurfaceLock<'s> {
  fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
    unsafe {
      assert!(x < self.width());
      assert!(y < self.height());
      let bytes_per_pixel = self.pixel_format().bytes_per_pixel();
      let base = self.pixels_mut();
      let row_start = base.offset(self.pitch() * (y as isize));
      let pixel = row_start.add(x * bytes_per_pixel);
      core::slice::from_raw_parts_mut(pixel, bytes_per_pixel)
    }
  }
}
impl<'s> SurfaceLock<'s> {
  fn pixels(&self) -> *const u8 {
    unsafe { (*self.surface.nn.as_ptr()).pixels as *const u8 }
  }
  fn pixels_mut(&mut self) -> *mut u8 {
    unsafe { (*self.surface.nn.as_ptr()).pixels as *mut u8 }
  }
}
