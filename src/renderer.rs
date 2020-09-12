use core::{ops::Deref, ptr::NonNull};

use alloc::{rc::Rc, string::String, sync::Arc};

use fermium::SDL_Renderer;

use crate::{sdl_get_error, Initialization, Window, WindowCreationFlags};

pub(crate) struct Renderer {
  nn: NonNull<SDL_Renderer>,
  // Note(Lokathor): As long as the renderer lives, we have to also keep the
  // window that created it alive.
  #[allow(dead_code)]
  win: Rc<Window>,
}
impl Drop for Renderer {
  // Note(Lokathor): The drop for the Rc runs *after* this drop code.
  fn drop(&mut self) {
    unsafe {
      fermium::SDL_DestroyRenderer(self.nn.as_ptr());
    }
  }
}

pub struct RendererWindow {
  rend: Rc<Renderer>,
  win: Rc<Window>,
}
impl Deref for RendererWindow {
  type Target = Window;
  #[inline]
  #[must_use]
  fn deref(&self) -> &Self::Target {
    &self.win
  }
}
impl RendererWindow {
  pub(crate) fn new(
    init: Arc<Initialization>, title: &str, pos: Option<[i32; 2]>,
    size: [u32; 2], flags: WindowCreationFlags,
  ) -> Result<Self, String> {
    let win = Rc::new(Window::new(init, title, pos, size, flags)?);
    let nn = NonNull::new(unsafe {
      fermium::SDL_CreateRenderer(
        win.as_ptr(),
        -1,
        (fermium::SDL_RENDERER_ACCELERATED | fermium::SDL_RENDERER_PRESENTVSYNC)
          as u32,
      )
    })
    .ok_or_else(sdl_get_error)?;
    let rend = Rc::new(Renderer { nn, win: win.clone() });
    Ok(RendererWindow { win, rend })
  }

  pub fn clear(&self) -> Result<(), String> {
    let ret = unsafe { fermium::SDL_RenderClear(self.rend.nn.as_ptr()) };
    if ret >= 0 {
      Ok(())
    } else {
      Err(sdl_get_error())
    }
  }

  pub fn present(&self) {
    unsafe { fermium::SDL_RenderPresent(self.rend.nn.as_ptr()) }
  }
}
