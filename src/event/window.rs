use super::*;

/// An event happened to the window.
#[derive(Debug, Clone, Copy)]
pub struct WindowEvent {
  /// When?
  pub timestamp: u32,
  /// Which window?
  pub window_id: u32,
  /// Event details, see [`WindowEventEnum`](WindowEventEnum).
  pub event: WindowEventEnum,
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

/// The types of window event that can happen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowEventEnum {
  /// The window is now shown.
  Shown,
  /// The window is now hidden.
  Hidden,
  /// The window surface was exposed and should redraw.
  Exposed,
  /// The window was moved
  Moved {
    /// New X, relative to the monitor.
    x: i32,
    /// New Y, relative to the monitor.
    y: i32,
  },
  /// The window has a new size. Always preceded by `SizeChanged`.
  Resized {
    /// New width (in logical screen units maybe?).
    w: u32,
    /// New height
    h: u32,
  },
  /// The window size changed.
  ///
  /// The new size will be reported in a `Resized` event if the size change was
  /// caused by something _other_ than you telling SDL to change the window
  /// size.
  SizeChanged,
  /// The window is now minimized.
  Minimized,
  /// The window is now maximized.
  Maximized,
  /// The window is restored from a minimized state.
  Restored,
  /// The mouse entered the window area.
  MouseEnter,
  /// The mouse left the window area.
  MouseLeave,
  /// The keyboard focus is on this window now.
  KeyboardFocusGained,
  /// The keyboard focus left this window.
  KeyboardFocusLost,
  /// The window manager wants to close the window.
  Close,
  /// The window is being offered focus.
  ///
  /// Call `SDL_SetWindowInputFocus` on yourself, or a sub-window, or ignore
  /// it.
  TakeFocus,
  /// The window had a hit test that wasn't `SDL_HITTEST_NORMAL`.
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
