use super::*;

/// The user moved the mouse wheel.
#[derive(Debug, Clone, Copy)]
pub struct MouseWheelEvent {
  /// When?
  pub timestamp: u32,
  /// What window?
  pub window_id: u32,
  /// What mouse? or `SDL_TOUCH_MOUSEID` for the touch screen.
  pub mouse_id: u32,
  /// The X delta (right is +, left is -).
  pub x_delta: i32,
  /// The Y delta (away from user is +, towards user is -).
  pub y_delta: i32,
  /// If this is set then the delta values are reverse of the above.
  pub is_normal: bool,
}

impl From<fermium::SDL_MouseWheelEvent> for MouseWheelEvent {
  fn from(ev: fermium::SDL_MouseWheelEvent) -> Self {
    Self {
      timestamp: ev.timestamp,
      window_id: ev.windowID,
      mouse_id: ev.which,
      x_delta: ev.x,
      y_delta: ev.y,
      is_normal: ev.direction == fermium::SDL_MOUSEWHEEL_NORMAL as u32,
    }
  }
}
impl From<MouseWheelEvent> for fermium::SDL_MouseWheelEvent {
  fn from(ev: MouseWheelEvent) -> Self {
    Self {
      type_: fermium::SDL_MOUSEWHEEL as u32,
      timestamp: ev.timestamp,
      windowID: ev.window_id,
      which: ev.mouse_id,
      x: ev.x_delta,
      y: ev.y_delta,
      direction: if ev.is_normal {
        fermium::SDL_MOUSEWHEEL_NORMAL
      } else {
        fermium::SDL_MOUSEWHEEL_FLIPPED
      } as u32,
    }
  }
}
