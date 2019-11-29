use super::*;

/// Event for the mouse moving around.
#[derive(Debug, Clone, Copy)]
pub struct MouseMotionEvent {
  /// When?
  pub timestamp: u32,
  /// What window?
  pub window_id: u32,
  /// The mouse ID of an actual mouse, or `SDL_TOUCH_MOUSEID`
  pub mouse_id: u32,
  /// The button state while the event is going on.
  pub state: u32,
  /// The X position (relative to the window).
  pub x_pos: i32,
  /// The Y position (relative to the window).
  pub y_pos: i32,
  /// The X delta.
  pub x_delta: i32,
  /// The Y delta.
  pub y_delta: i32,
}
impl From<fermium::SDL_MouseMotionEvent> for MouseMotionEvent {
  fn from(ev: fermium::SDL_MouseMotionEvent) -> Self {
    Self {
      timestamp: ev.timestamp,
      window_id: ev.windowID,
      mouse_id: ev.which,
      state: ev.state,
      x_pos: ev.x,
      y_pos: ev.y,
      x_delta: ev.xrel,
      y_delta: ev.yrel,
    }
  }
}

impl From<MouseMotionEvent> for fermium::SDL_MouseMotionEvent {
  fn from(ev: MouseMotionEvent) -> Self {
    Self {
      type_: fermium::SDL_MOUSEMOTION as u32,
      timestamp: ev.timestamp,
      windowID: ev.window_id,
      which: ev.mouse_id,
      state: ev.state,
      x: ev.x_pos,
      y: ev.y_pos,
      xrel: ev.x_delta,
      yrel: ev.y_delta,
    }
  }
}
