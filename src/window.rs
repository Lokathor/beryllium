#![allow(unused_imports)]

use core::ptr::NonNull;

use fermium::{
  prelude::{
    SDL_SetWindowTitle, SDL_Window, SDL_WindowFlags, SDL_TRUE, SDL_WINDOW_ALLOW_HIGHDPI,
    SDL_WINDOW_BORDERLESS, SDL_WINDOW_FULLSCREEN, SDL_WINDOW_HIDDEN, SDL_WINDOW_INPUT_GRABBED,
    SDL_WINDOW_MAXIMIZED, SDL_WINDOW_METAL, SDL_WINDOW_MINIMIZED, SDL_WINDOW_OPENGL,
    SDL_WINDOW_RESIZABLE, SDL_WINDOW_VULKAN,
  },
  syswm::{SDL_GetWindowWMInfo, SDL_SysWMinfo},
  version::SDL_VERSION,
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

#[cfg(feature = "use-raw-window-handle")]
unsafe impl raw_window_handle::HasRawWindowHandle for Window {
  /// If this can't get the window info, it'll give an empty win32 handle, which
  /// is the best that can be done because this trait doesn't allow for failure.
  fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
    use raw_window_handle::{RawWindowHandle, Win32Handle};
    let mut info = SDL_SysWMinfo::default();
    SDL_VERSION(&mut info.version);
    if unsafe { SDL_GetWindowWMInfo(self.win.as_ptr(), &mut info) } == SDL_TRUE {
      unsafe { info.try_into() }.unwrap_or(RawWindowHandle::Win32(Win32Handle::empty()))
    } else {
      RawWindowHandle::Win32(Win32Handle::empty())
    }
  }
}
