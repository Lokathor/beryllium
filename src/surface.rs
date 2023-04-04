use crate::init::SdlInit;
use alloc::sync::Arc;
use core::ptr::NonNull;
use fermium::prelude::*;

#[repr(C)]
pub struct Surface {
  surf: NonNull<SDL_Surface>,
  parent: Arc<SdlInit>,
}
impl Drop for Surface {
  #[inline]
  fn drop(&mut self) {
    unsafe { SDL_FreeSurface(self.surf.as_ptr()) };
  }
}
