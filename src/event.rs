use super::*;

/// The various events that can happen.
#[derive(Debug, Clone, Copy)]
pub enum Event {
  /// Quit was requested by the user
  Quit {
    /// Time, in milliseconds, since SDL2 was initialized.
    timestamp: u32,
  },
  /// Event for any time the user moves the mouse within a window, or if
  /// `warp_mouse_in_window` is called.
  MouseMotion {
    /// Time, in milliseconds, since SDL2 was initialized.
    timestamp: u32,
    /// Window with mouse focus, if any
    window_id: u32,
    /// The mouse that generated this event. If `None` it was a touch event.
    mouse_id: Option<u32>,
    /// State of the mouse buttons during this event
    state: MouseButtonState,
    /// X, relative to the window
    x: i32,
    /// Y, relative to the window
    y: i32,
    /// Change in X position
    delta_x: i32,
    /// Change in Y position
    delta_y: i32,
  },
  /// Generated whenever a mouse button is pressed or released.
  MouseButtonEvent {
    /// Time, in milliseconds, since SDL2 was initialized.
    timestamp: u32,
    /// Window with mouse focus, if any
    window_id: u32,
    /// The mouse that generated this event. If `None` it was a touch event.
    mouse_id: Option<u32>,
    /// The button that changed
    button: MouseButton,
    /// If the button is now pressed or released
    is_pressed: bool,
    /// 1 for single-click, 2 for double-click, etc
    clicks: u8,
    /// X, relative to the window
    x: i32,
    /// Y, relative to the window
    y: i32,
  },
  /// Generated whenever the user moves the mouse wheel.
  MouseWheel {
    /// Time, in milliseconds, since SDL2 was initialized.
    timestamp: u32,
    /// Window with mouse focus, if any
    window_id: u32,
    /// The mouse that generated this event. If `None` it was a touch event.
    mouse_id: Option<u32>,
    /// Horizontal scroll, negative Left or positive Right
    x: i32,
    /// Vertical scroll, negative to User or positive away from User
    y: i32,
    /// Mouse wheel isn't consistent on all platforms. If this bool is set, the
    /// meaning of the `x` and `y` field is inverted compared to normal.
    is_flipped: bool,
  },
  /// It's always possible that we'll load some future version which will have
  /// event variants we don't understand, which we have to just ignore.
  UnknownEventType,
}
impl From<SDL_Event> for Event {
  /// Parses "without fail", but will turn unknown events into `UnknownEventType`.
  ///
  /// So, it's not lossless I guess. Whatever.
  fn from(event: SDL_Event) -> Self {
    unsafe {
      match event.type_ as SDL_EventType::Type {
        SDL_QUIT => Event::Quit {
          timestamp: event.quit.timestamp,
        },
        SDL_MOUSEMOTION => Event::MouseMotion {
          timestamp: event.motion.timestamp,
          window_id: event.motion.windowID,
          mouse_id: if event.motion.which == SDL_TOUCH_MOUSEID {
            None
          } else {
            Some(event.motion.which)
          },
          state: MouseButtonState(event.motion.state),
          x: event.motion.x,
          y: event.motion.y,
          delta_x: event.motion.xrel,
          delta_y: event.motion.yrel,
        },
        SDL_MOUSEBUTTONDOWN | SDL_MOUSEBUTTONUP => Event::MouseButtonEvent {
          timestamp: event.button.timestamp,
          window_id: event.button.windowID,
          mouse_id: if event.button.which == SDL_TOUCH_MOUSEID {
            None
          } else {
            Some(event.button.which)
          },
          button: MouseButton::from(event.button.button),
          is_pressed: u32::from(event.button.state) == SDL_PRESSED,
          clicks: event.button.clicks,
          x: event.button.x,
          y: event.button.y,
        },
        SDL_MOUSEWHEEL => Event::MouseWheel {
          timestamp: event.wheel.timestamp,
          window_id: event.wheel.windowID,
          mouse_id: if event.wheel.which == SDL_TOUCH_MOUSEID {
            None
          } else {
            Some(event.wheel.which)
          },
          x: event.wheel.x,
          y: event.wheel.y,
          is_flipped: event.wheel.direction as fermium::SDL_MouseWheelDirection::Type
            == fermium::SDL_MouseWheelDirection::SDL_MOUSEWHEEL_FLIPPED,
        },
        _ => Event::UnknownEventType,
      }
    }
  }
}

/// The possible mouse buttons.
#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
  /// Left side
  Left,
  /// Middle, usually a mouse wheel click
  Middle,
  /// Right side
  Right,
  /// Extra button 1
  X1,
  /// Extra button 2
  X2,
  /// Unknown mouse button
  Unknown,
}
impl From<u8> for MouseButton {
  fn from(button_byte: u8) -> Self {
    match u32::from(button_byte) {
      SDL_BUTTON_LEFT => MouseButton::Left,
      SDL_BUTTON_MIDDLE => MouseButton::Middle,
      SDL_BUTTON_RIGHT => MouseButton::Right,
      SDL_BUTTON_X1 => MouseButton::X1,
      SDL_BUTTON_X2 => MouseButton::X1,
      _ => MouseButton::Unknown,
    }
  }
}

// Note(Lokathor): `bindgen` doesn't generate these things itself.
macro_rules! sdl_button {
  ($x:expr) => {
    1 << ($x - 1)
  };
}
const SDL_BUTTON_LMASK: u32 = sdl_button!(SDL_BUTTON_LEFT);
const SDL_BUTTON_MMASK: u32 = sdl_button!(SDL_BUTTON_MIDDLE);
const SDL_BUTTON_RMASK: u32 = sdl_button!(SDL_BUTTON_RIGHT);
const SDL_BUTTON_X1MASK: u32 = sdl_button!(SDL_BUTTON_X1);
const SDL_BUTTON_X2MASK: u32 = sdl_button!(SDL_BUTTON_X2);

/// Holds flags for the state of all mouse buttons at any given moment.
#[derive(Debug, Clone, Copy)]
pub struct MouseButtonState(u32);
impl MouseButtonState {
  phantom_fields! {
    self.0: u32,
    left: SDL_BUTTON_LMASK,
    middle: SDL_BUTTON_MMASK,
    right: SDL_BUTTON_RMASK,
    x1: SDL_BUTTON_X1MASK,
    x2: SDL_BUTTON_X2MASK,
  }
}
