use super::*;

#[derive(Debug, Clone, Copy)]
pub struct WindowEvent {
  timestamp: u32,
  window_id: u32,
  event: WindowEventEnum,
}
impl TryFrom<fermium::SDL_WindowEvent> for WindowEvent {
  type Error = ();

  fn try_from(ev: fermium::SDL_WindowEvent) -> Result<WindowEvent, ()> {
    Ok(Self {
      timestamp: ev.timestamp,
      window_id: ev.windowID,
      event: match ev.event as fermium::SDL_WindowEventID {
        fermium::SDL_WINDOWEVENT_SHOWN => WindowEventEnum::Shown,
        fermium::SDL_WINDOWEVENT_HIDDEN => WindowEventEnum::Hidden,
        fermium::SDL_WINDOWEVENT_EXPOSED => WindowEventEnum::Exposed,
        fermium::SDL_WINDOWEVENT_MOVED => {
          WindowEventEnum::Moved { x: ev.data1, y: ev.data2 }
        }
        fermium::SDL_WINDOWEVENT_RESIZED => {
          WindowEventEnum::Resized { w: ev.data1 as u32, h: ev.data2 as u32 }
        }
        fermium::SDL_WINDOWEVENT_SIZE_CHANGED => WindowEventEnum::SizeChanged,
        fermium::SDL_WINDOWEVENT_MINIMIZED => WindowEventEnum::Minimized,
        fermium::SDL_WINDOWEVENT_MAXIMIZED => WindowEventEnum::Maximized,
        fermium::SDL_WINDOWEVENT_RESTORED => WindowEventEnum::Restored,
        fermium::SDL_WINDOWEVENT_ENTER => WindowEventEnum::MouseEnter,
        fermium::SDL_WINDOWEVENT_LEAVE => WindowEventEnum::MouseLeave,
        fermium::SDL_WINDOWEVENT_FOCUS_GAINED => {
          WindowEventEnum::KeyboardFocusGained
        }
        fermium::SDL_WINDOWEVENT_FOCUS_LOST => {
          WindowEventEnum::KeyboardFocusLost
        }
        fermium::SDL_WINDOWEVENT_CLOSE => WindowEventEnum::Close,
        fermium::SDL_WINDOWEVENT_TAKE_FOCUS => WindowEventEnum::TakeFocus,
        fermium::SDL_WINDOWEVENT_HIT_TEST => WindowEventEnum::HitTest,
        _ => return Err(()),
      },
    })
  }
}

impl From<WindowEvent> for fermium::SDL_WindowEvent {
  fn from(ev: WindowEvent) -> Self {
    let (data1, data2) = ev.event.data1data2();
    Self {
      type_: fermium::SDL_WINDOWEVENT as u32,
      timestamp: ev.timestamp,
      windowID: ev.window_id,
      event: fermium::SDL_WindowEventID::from(ev.event) as u8,
      padding1: 0,
      padding2: 0,
      padding3: 0,
      data1,
      data2,
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum WindowEventEnum {
  Shown,
  Hidden,
  Exposed,
  Moved { x: i32, y: i32 },
  Resized { w: u32, h: u32 },
  SizeChanged,
  Minimized,
  Maximized,
  Restored,
  MouseEnter,
  MouseLeave,
  KeyboardFocusGained,
  KeyboardFocusLost,
  Close,
  TakeFocus,
  HitTest,
}
impl WindowEventEnum {
  fn data1data2(self) -> (i32, i32) {
    match self {
      WindowEventEnum::Moved { x, y } => (x, y),
      WindowEventEnum::Resized { w, h } => (w as i32, h as i32),
      _ => (0, 0),
    }
  }
}
impl From<WindowEventEnum> for fermium::SDL_WindowEventID {
  fn from(wee: WindowEventEnum) -> Self {
    match wee {
      WindowEventEnum::Shown => fermium::SDL_WINDOWEVENT_SHOWN,
      WindowEventEnum::Hidden => fermium::SDL_WINDOWEVENT_HIDDEN,
      WindowEventEnum::Exposed => fermium::SDL_WINDOWEVENT_EXPOSED,
      WindowEventEnum::Moved { .. } => fermium::SDL_WINDOWEVENT_MOVED,
      WindowEventEnum::Resized { .. } => fermium::SDL_WINDOWEVENT_RESIZED,
      WindowEventEnum::SizeChanged => fermium::SDL_WINDOWEVENT_SIZE_CHANGED,
      WindowEventEnum::Minimized => fermium::SDL_WINDOWEVENT_MINIMIZED,
      WindowEventEnum::Maximized => fermium::SDL_WINDOWEVENT_MAXIMIZED,
      WindowEventEnum::Restored => fermium::SDL_WINDOWEVENT_RESTORED,
      WindowEventEnum::MouseEnter => fermium::SDL_WINDOWEVENT_ENTER,
      WindowEventEnum::MouseLeave => fermium::SDL_WINDOWEVENT_LEAVE,
      WindowEventEnum::KeyboardFocusGained => {
        fermium::SDL_WINDOWEVENT_FOCUS_GAINED
      }
      WindowEventEnum::KeyboardFocusLost => fermium::SDL_WINDOWEVENT_FOCUS_LOST,
      WindowEventEnum::Close => fermium::SDL_WINDOWEVENT_CLOSE,
      WindowEventEnum::TakeFocus => fermium::SDL_WINDOWEVENT_TAKE_FOCUS,
      WindowEventEnum::HitTest => fermium::SDL_WINDOWEVENT_HIT_TEST,
    }
  }
}
