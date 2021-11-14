use core::ptr::NonNull;

use fermium::prelude::{
  SDL_SetWindowTitle, SDL_Window, SDL_WindowFlags, SDL_WINDOW_ALLOW_HIGHDPI, SDL_WINDOW_BORDERLESS,
  SDL_WINDOW_FULLSCREEN, SDL_WINDOW_HIDDEN, SDL_WINDOW_INPUT_GRABBED, SDL_WINDOW_MAXIMIZED,
  SDL_WINDOW_METAL, SDL_WINDOW_MINIMIZED, SDL_WINDOW_OPENGL, SDL_WINDOW_RESIZABLE,
  SDL_WINDOW_VULKAN,
};
use zstring::ZStr;

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct WindowFlags(pub(crate) SDL_WindowFlags);
impl_bit_ops_for_tuple_newtype!(WindowFlags);
impl WindowFlags {
  pub const FULLSCREEN: Self = Self(SDL_WINDOW_FULLSCREEN);
  pub const OPENGL: Self = Self(SDL_WINDOW_OPENGL);
  pub const HIDDEN: Self = Self(SDL_WINDOW_HIDDEN);
  pub const BORDERLESS: Self = Self(SDL_WINDOW_BORDERLESS);
  pub const RESIZABLE: Self = Self(SDL_WINDOW_RESIZABLE);
  pub const MAXIMIZED: Self = Self(SDL_WINDOW_MAXIMIZED);
  pub const MINIMIZED: Self = Self(SDL_WINDOW_MINIMIZED);
  pub const GRABBED: Self = Self(SDL_WINDOW_INPUT_GRABBED);
  pub const ALLOW_HIGHDPI: Self = Self(SDL_WINDOW_ALLOW_HIGHDPI);
  pub const VULKAN: Self = Self(SDL_WINDOW_VULKAN);
  pub const METAL: Self = Self(SDL_WINDOW_METAL);
}

pub struct Window {
  pub(crate) win: NonNull<SDL_Window>,
}
impl Window {
  #[inline]
  pub fn set_title(&self, new_title: ZStr<'_>) {
    unsafe { SDL_SetWindowTitle(self.win.as_ptr(), new_title.as_ptr().cast()) }
  }
}
