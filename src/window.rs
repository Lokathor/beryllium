use core::ptr::NonNull;

use alloc::{rc::Rc, sync::Arc};

use tinyvec::TinyVec;

use fermium::SDL_Window;

use crate::{sdl_get_error, Initialization, SdlError};

/// The "superclass" for the SDL window types.
///
/// You don't ever make one of these directly. Instead, each drawing API has its
/// own window "subtype" (`GlWindow`, `VkWindow`, `RendererWindow`, `RawWindow`)
/// that can [`Deref`](core::ops::Deref) into this type.
pub struct Window {
  nn: NonNull<SDL_Window>,
  // Note(Lokathor): As long as the window lives, we have to also keep SDL
  // itself alive.
  #[allow(dead_code)]
  init: Arc<Initialization>,
}
impl Drop for Window {
  // Note(Lokathor): The drop for the Arc runs *after* this drop code.
  fn drop(&mut self) {
    unsafe { fermium::SDL_DestroyWindow(self.nn.as_ptr()) }
  }
}

impl Window {
  /// Makes a new window.
  ///
  /// * If `pos` is `None` you get a centered window.
  /// * `w` and `h` can't exceed 16_384.
  pub(crate) fn new(
    init: Arc<Initialization>, title: &str, pos: Option<[i32; 2]>,
    [w, h]: [u32; 2], flags: WindowCreationFlags,
  ) -> Result<Self, SdlError> {
    let title_null: TinyVec<[u8; 64]> =
      title.as_bytes().iter().copied().chain(Some(0)).collect();
    const CENTERED: i32 = fermium::SDL_WINDOWPOS_CENTERED;
    let [p_x, p_y] = pos.unwrap_or([CENTERED, CENTERED]);
    NonNull::new(unsafe {
      fermium::SDL_CreateWindow(
        title_null.as_ptr().cast(),
        p_x,
        p_y,
        w as i32,
        h as i32,
        flags.pack_to_u32(),
      )
    })
    .ok_or_else(sdl_get_error)
    .map(|nn| Window { init, nn })
  }

  pub(crate) fn as_ptr(&self) -> *mut SDL_Window {
    self.nn.as_ptr()
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowScreenCoverage {
  Windowed,
  Fullscreen,
  FullscreenDesktop,
}
impl Default for WindowScreenCoverage {
  fn default() -> Self {
    Self::Windowed
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
  Standard,
  Minimized,
  Maximized,
}
impl Default for WindowState {
  fn default() -> Self {
    Self::Standard
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WindowCreationFlags {
  pub coverage: WindowScreenCoverage,
  pub is_vulkan: bool,
  pub is_hidden: bool,
  pub is_borderless: bool,
  pub is_resizable: bool,
  pub state: WindowState,
  pub is_input_grabbed: bool,
  pub allow_high_dpi: bool,
}
impl WindowCreationFlags {
  fn pack_to_u32(self) -> u32 {
    let mut out = 0;
    match self.coverage {
      WindowScreenCoverage::Windowed => (),
      WindowScreenCoverage::Fullscreen => out |= fermium::SDL_WINDOW_FULLSCREEN,
      WindowScreenCoverage::FullscreenDesktop => {
        out |= fermium::SDL_WINDOW_FULLSCREEN_DESKTOP
      }
    }
    match self.state {
      WindowState::Standard => (),
      WindowState::Minimized => out |= fermium::SDL_WINDOW_MINIMIZED,
      WindowState::Maximized => out |= fermium::SDL_WINDOW_MAXIMIZED,
    }
    out |= if self.is_vulkan {
      fermium::SDL_WINDOW_VULKAN
    } else {
      fermium::SDL_WINDOW_OPENGL
    };
    out |= if self.is_hidden { fermium::SDL_WINDOW_HIDDEN } else { 0 };
    out |= if self.is_borderless { fermium::SDL_WINDOW_BORDERLESS } else { 0 };
    out |= if self.is_resizable { fermium::SDL_WINDOW_RESIZABLE } else { 0 };
    out |= if self.is_input_grabbed { fermium::SDL_WINDOW_VULKAN } else { 0 };
    out |=
      if self.allow_high_dpi { fermium::SDL_WINDOW_ALLOW_HIGHDPI } else { 0 };
    out as u32
  }
}
