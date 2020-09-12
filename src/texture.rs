use core::ptr::NonNull;

use alloc::rc::Rc;

use fermium::SDL_Texture;

use crate::Renderer;

pub struct Texture {
  nn: NonNull<SDL_Texture>,
  rend: Rc<Renderer>,
}
