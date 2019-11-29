use super::*;

/// A mouse button was pressed or released.
#[derive(Debug, Clone, Copy)]
pub struct MouseButtonEvent {
  /// When
  pub timestamp: u32,
  /// Which window was involved?
  pub window_id: u32,
  /// Which mouse was involved?
  pub mouse_id: u32,
  /// The button ID.
  pub button: u8,
  /// The button was pressed.
  pub is_pressed: bool,
  /// The number of repeated clicks.
  pub clicks: u8,
  /// The mouse's x position (relative to the window).
  pub x_pos: i32,
  /// The mouse's y position (relative to the window).
  pub y_pos: i32,
}
impl From<fermium::SDL_MouseButtonEvent> for MouseButtonEvent {
  fn from(ev: fermium::SDL_MouseButtonEvent) -> Self {
    Self {
      timestamp: ev.timestamp,
      window_id: ev.windowID,
      mouse_id: ev.which,
      button: ev.button,
      is_pressed: u32::from(ev.state) == fermium::SDL_PRESSED,
      clicks: ev.clicks,
      x_pos: ev.x,
      y_pos: ev.y,
    }
  }
}
impl From<MouseButtonEvent> for fermium::SDL_MouseButtonEvent {
  fn from(ev: MouseButtonEvent) -> Self {
    Self {
      type_: (if ev.is_pressed {
        fermium::SDL_MOUSEBUTTONDOWN
      } else {
        fermium::SDL_MOUSEBUTTONUP
      }) as u32,
      timestamp: ev.timestamp,
      windowID: ev.window_id,
      which: ev.mouse_id,
      button: ev.button,
      state: (if ev.is_pressed {
        fermium::SDL_PRESSED
      } else {
        fermium::SDL_RELEASED
      }) as u8,
      clicks: ev.clicks,
      padding1: 0,
      x: ev.x_pos,
      y: ev.y_pos,
    }
  }
}
