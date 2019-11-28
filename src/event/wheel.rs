use super::*;

#[derive(Debug, Clone, Copy)]
pub struct MouseWheelEvent {
  timestamp: u32,
  window_id: u32,
  mouse_id: u32,
  x_delta: i32,
  y_delta: i32,
  is_normal: bool,
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
