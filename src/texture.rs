use core::ptr::NonNull;

use alloc::rc::Rc;

use fermium::SDL_Texture;

use crate::Renderer;

pub struct Texture {
  pub(crate) nn: NonNull<SDL_Texture>,
  // Note(Lokathor): As long as the texture lives, we have to also keep the
  // renderer that created it alive.
  #[allow(dead_code)]
  pub(crate) rend: Rc<Renderer>,
}
impl Drop for Texture {
  fn drop(&mut self) {
    unsafe { fermium::SDL_DestroyTexture(self.nn.as_ptr()) }
  }
}
