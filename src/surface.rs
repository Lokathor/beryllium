use crate::{
  error::{get_error, SdlError},
  init::SdlInit,
  Sdl,
};
use alloc::sync::Arc;
use core::ptr::NonNull;
use fermium::prelude::*;
use pixel_formats::r8g8b8a8_Srgb;

#[repr(C)]
pub struct Surface {
  pub(crate) surf: NonNull<SDL_Surface>,
  pub(crate) parent: Arc<SdlInit>,
}
impl Drop for Surface {
  #[inline]
  fn drop(&mut self) {
    unsafe { SDL_FreeSurface(self.surf.as_ptr()) };
  }
}
impl Sdl {
  #[inline]
  pub fn create_surface_from(
    &self, pixels: &[r8g8b8a8_Srgb], width: i32, height: i32,
  ) -> Result<Surface, SdlError> {
    if width <= 0
      || height <= 0
      || width.checked_mul(height).unwrap_or_default() as usize != pixels.len()
    {
      return Err(SdlError::new("illegal input dimensions"));
    }
    let depth = 32;
    let pitch = width * 4;
    const R_MASK: u32 = unsafe { core::mem::transmute(r8g8b8a8_Srgb { r: 255, g: 0, b: 0, a: 0 }) };
    const G_MASK: u32 = unsafe { core::mem::transmute(r8g8b8a8_Srgb { g: 255, r: 0, b: 0, a: 0 }) };
    const B_MASK: u32 = unsafe { core::mem::transmute(r8g8b8a8_Srgb { b: 255, g: 0, r: 0, a: 0 }) };
    const A_MASK: u32 = unsafe { core::mem::transmute(r8g8b8a8_Srgb { a: 255, g: 0, b: 0, r: 0 }) };
    let p = unsafe {
      SDL_CreateRGBSurfaceFrom(
        pixels.as_ptr() as *mut c_void,
        width,
        height,
        depth,
        pitch,
        R_MASK,
        G_MASK,
        B_MASK,
        A_MASK,
      )
    };
    match NonNull::new(p) {
      Some(surf) => Ok(Surface { surf, parent: self.init.clone() }),
      None => Err(get_error()),
    }
  }
}
