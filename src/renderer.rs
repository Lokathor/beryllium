use core::{convert::TryInto, ops::Deref, ptr::NonNull};

use alloc::{rc::Rc, sync::Arc};

use fermium::SDL_Renderer;

use crate::{
  sdl_get_error, Initialization, PixelFormatEnum, SdlError, Surface, Texture,
  Window, WindowCreationFlags,
};

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
  ) -> Result<Self, SdlError> {
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

  pub fn clear(&self) -> Result<(), SdlError> {
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

  pub fn create_texture(
    &self, pixel_format: PixelFormatEnum, access: TextureAccess, w: u32, h: u32,
  ) -> Result<Texture, SdlError> {
    NonNull::new(unsafe {
      fermium::SDL_CreateTexture(
        self.rend.nn.as_ptr(),
        pixel_format.0,
        access as _,
        w.try_into().unwrap(),
        h.try_into().unwrap(),
      )
    })
    .ok_or_else(sdl_get_error)
    .map(|nn| Texture { nn, rend: self.rend.clone() })
  }

  pub fn create_texture_from_surface(
    &self, surface: &Surface,
  ) -> Result<Texture, SdlError> {
    NonNull::new(unsafe {
      fermium::SDL_CreateTextureFromSurface(
        self.rend.nn.as_ptr(),
        surface.nn.as_ptr(),
      )
    })
    .ok_or_else(sdl_get_error)
    .map(|nn| Texture { nn, rend: self.rend.clone() })
  }
}

pub enum TextureAccess {
  /// Changes rarely, not lockable.
  Static = fermium::SDL_TEXTUREACCESS_STATIC as _,
  /// Changes frequently, lockable.
  Streaming = fermium::SDL_TEXTUREACCESS_STREAMING as _,
  /// Can be used as a render target.
  Target = fermium::SDL_TEXTUREACCESS_TARGET as _,
}
