use crate::{
  error::{get_error, SdlError},
  init::SdlInit,
  Sdl,
};
use alloc::{string::String, sync::Arc};
use core::{
  ops::Deref,
  ptr::NonNull,
  sync::atomic::{AtomicBool, Ordering},
};
use fermium::prelude::*;

mod gl;
pub use gl::*;

#[cfg(feature = "vulkan")]
mod vk;
#[cfg(feature = "vulkan")]
pub use vk::*;

mod renderer;
pub use renderer::*;

pub struct CreateWinArgs<'s> {
  pub title: &'s str,
  pub width: i32,
  pub height: i32,
  pub allow_high_dpi: bool,
  pub borderless: bool,
  pub resizable: bool,
}
impl CreateWinArgs<'_> {
  fn window_flags(&self) -> SDL_WindowFlags {
    let mut out = 0;
    if self.allow_high_dpi {
      out |= SDL_WINDOW_ALLOW_HIGHDPI.0;
    }
    if self.borderless {
      out |= SDL_WINDOW_BORDERLESS.0
    }
    if self.resizable {
      out |= SDL_WINDOW_RESIZABLE.0
    }
    SDL_WindowFlags(out)
  }
}
impl Default for CreateWinArgs<'_> {
  #[inline]
  fn default() -> Self {
    Self {
      title: "DefaultName",
      width: 800,
      height: 600,
      allow_high_dpi: true,
      borderless: false,
      resizable: false,
    }
  }
}

/// Provides the methods common to all types of window.
#[repr(C)]
pub struct CommonWindow {
  win: NonNull<SDL_Window>,
}
impl CommonWindow {
  /// Gets the window size in logical "screen units".
  ///
  /// If High DPI is used, this will generally be *less* than the number of
  /// actual pixels within the window.
  #[inline]
  pub fn get_window_size(&self) -> (i32, i32) {
    let mut width = 0_i32;
    let mut height = 0_i32;
    unsafe { SDL_GetWindowSize(self.win.as_ptr(), &mut width, &mut height) }
    (width, height)
  }

  /// Sets the window size in logical "screen units".
  #[inline]
  pub fn set_window_size(&self, width: i32, height: i32) {
    unsafe { SDL_SetWindowSize(self.win.as_ptr(), width, height) }
  }

  #[inline]
  pub fn set_title(&self, title: &str) {
    let new_title = alloc::format!("{title}\0");
    unsafe { SDL_SetWindowTitle(self.win.as_ptr(), new_title.as_ptr().cast()) }
  }
}
