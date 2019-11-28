use super::*;

#[derive(Debug, Clone, Copy)]
pub struct MouseMotionEvent {
  pub timestamp: u32,
  pub window_id: u32,
  /// The mouse ID of an actual mouse, or `SDL_TOUCH_MOUSEID`
  pub mouse_id: u32,
  // TODO: newtype this
  pub state: u32,
  pub x_pos: i32,
  pub y_pos: i32,
  pub x_delta: i32,
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
