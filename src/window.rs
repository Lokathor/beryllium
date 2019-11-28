use super::*;

/// A handle to an [`SDL_Window`](https://wiki.libsdl.org/SDL_CreateWindow).
///
/// This is the "Drawing Style Agnostic" core window stuff that's common to all
/// drawing APIs. You can't actually make one of these. Instead, you make a
/// window that's fused to the drawing API involved.
///
/// * [`SDL::crate_gl_window`](SDL::crate_gl_window) gives a [`GlWindow`]
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
