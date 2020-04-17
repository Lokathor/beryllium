use super::*;

mod gl_window;
pub use gl_window::*;

#[cfg(feature = "extern_crate_raw_window_handle")]
mod raw_window;
#[cfg(feature = "extern_crate_raw_window_handle")]
pub use raw_window::*;

/// A handle to an [`SDL_Window`](https://wiki.libsdl.org/SDL_CreateWindow).
///
/// This is the "Drawing Style Agnostic" core window stuff that's common to all
/// drawing APIs. You can't actually make one of these. Instead, you make a
/// window that's fused to the drawing API involved.
///
/// * [`SDL::crate_gl_window`](SDL::create_gl_window) gives a [`GlWindow`]
/// * ...more to come some day! (Vk and SDL Renderer)
#[repr(transparent)]
pub struct Window {
  // Note: The Init token is stored in the abstraction struct that bundles a
  // window with whatever else is being used to draw to the window (OGL, VK, or
  // SDL_Renderer) so that we don't have needless duplication.
  pub(crate) win: *mut fermium::SDL_Window,
}
impl Drop for Window {
  fn drop(&mut self) {
    unsafe { fermium::SDL_DestroyWindow(self.win) }
  }
}

impl Window {
  /// Use this to move the mouse to a given position within the window.
  ///
  /// This generates a mouse motion event.
  pub fn warp_mouse_in_window(&self, x: i32, y: i32) {
    unsafe { fermium::SDL_WarpMouseInWindow(self.win, x, y) }
  }

  /// Assigns a new window title.
  /// 
  /// See [`SDL_SetWindowTitle`](https://wiki.libsdl.org/SDL_SetWindowTitle)
  pub fn set_title(&mut self, title: &str) {
    let v: Vec<u8> = title.bytes().chain(Some(0)).collect();
    unsafe { fermium::SDL_SetWindowTitle(self.win, v.as_ptr() as *const c_char) }
  }
}

/// The starting position for a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowPosition {
  /// The (x,y) position specified.
  XY(i32, i32),
  /// Center the window on the screen.
  Centered,
  /// Put the window anywhere.
  Undefined,
}
impl Default for WindowPosition {
  /// ```rust
  /// use beryllium::WindowPosition;
  /// assert_eq!(WindowPosition::default(), WindowPosition::Undefined);
  /// ```
  fn default() -> Self {
    WindowPosition::Undefined
  }
}
impl WindowPosition {
  pub(crate) fn what_sdl_wants(self) -> (i32, i32) {
    match self {
      WindowPosition::XY(x, y) => (x, y),
      WindowPosition::Centered => (
        fermium::SDL_WINDOWPOS_CENTERED_MASK as i32,
        fermium::SDL_WINDOWPOS_CENTERED_MASK as i32,
      ),
      WindowPosition::Undefined => (
        fermium::SDL_WINDOWPOS_UNDEFINED_MASK as i32,
        fermium::SDL_WINDOWPOS_UNDEFINED_MASK as i32,
      ),
    }
  }
}

/// Allows you to specify the flags for window creation.
///
/// See [`SDL_WindowFlags`](https://wiki.libsdl.org/SDL_WindowFlags)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct WindowFlags(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl WindowFlags {
  /// Window is visible.
  pub const Shown: WindowFlags = WindowFlags(fermium::SDL_WINDOW_SHOWN as u32);

  /// Window should support OpenGL.
  pub const OpenGL: WindowFlags =
    WindowFlags(fermium::SDL_WINDOW_OPENGL as u32);

  /// Window should support Vulkan.
  pub const Vulkan: WindowFlags =
    WindowFlags(fermium::SDL_WINDOW_VULKAN as u32);

  // TODO: more flags later.
}
impl core::ops::BitOr for WindowFlags {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self {
    Self(self.0 | rhs.0)
  }
}
impl core::ops::BitOrAssign for WindowFlags {
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}

#[derive(Debug, Clone, Copy)]
pub struct DPI {
  pub diagonal: f32,
  pub horizontal: f32,
  pub vertical: f32,
}
