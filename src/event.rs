use super::*;

/// The various events that can happen.
#[derive(Debug, Clone, Copy)]
pub enum Event {
  /// A controller axis is in a new position.
  ControllerAxis {
    /// When the event happened
    timestamp: u32,
    /// The controller
    joystick_id: JoystickID,
    /// The new axis
    axis: ControllerAxis,
    /// The new position. For sticks it will be full negative to positive range,
    /// for triggers it will be 0 to maximum.
    value: i16,
  },
  /// A controller button is in a new state.
  ///
  /// Note(Lokathor): Limited testing shows that some controllers will repeat
  /// the same state for a button without the button changing sates in between.
  /// For example, some controllers will repeatedly send "button up" events even
  /// though the button never went down.
  ControllerButton {
    /// When the event happened
    timestamp: u32,
    /// The controller
    joystick_id: JoystickID,
    /// The button
    button: ControllerButton,
    /// If the button is now pressed or released.
    pressed: bool,
  },
  /// A controller was added.
  ControllerDeviceAdded {
    /// When the event happened
    timestamp: u32,
    /// The controller
    joystick_id: JoystickID,
  },
  /// The button mapping layout for a controller has changed.
  ControllerDeviceRemapped {
    /// When the event happened
    timestamp: u32,
    /// The controller
    joystick_id: JoystickID,
  },
  /// A controller was removed.
  ControllerDeviceRemoved {
    /// When the event happened
    timestamp: u32,
    /// The controller
    joystick_id: JoystickID,
  },
  /// Keyboard button event information (a press or release).
  Keyboard {
    /// When the event happened
    timestamp: u32,
    /// The window that was focused when the event happened.
    window_id: u32,
    /// If the key was pressed or released by this event.
    is_key_down: bool,
    /// If this is a "key repeat" event (usually 0 or 1, though possibly more).
    repeat_count: u8,
    /// The information about the key being pressed or released.
    key_info: KeyInfo,
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
  /// The window has gained mouse focus.
  ///
  /// Inverse of [`Event::MouseLeftWindow`].
  MouseEnteredWindow {
    /// When the event happened
    timestamp: u32,
    /// The window which gained mouse focus.
    window_id: u32,
  },
  /// The window has lost mouse focus.
  ///
  /// Inverse of [`Event::MouseEnteredWindow`].
  MouseLeftWindow {
    /// When the event happened
    timestamp: u32,
    /// The window which lost mouse focus.
    window_id: u32,
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
  /// Quit was requested by the user
  Quit {
    /// Time, in milliseconds, since SDL2 was initialized.
    timestamp: u32,
  },
  /// The window manager requests that the window be closed.
  WindowClosed {
    /// When the event happened
    timestamp: u32,
    /// The window which wants to be closed.
    window_id: u32,
  },
  /// The window has gained keyboard focus.
  ///
  /// Inverse of [`Event::WindowLostFocus`].
  WindowGainedFocus {
    /// When the event happened
    timestamp: u32,
    /// The window which gained keyboard focus.
    window_id: u32,
  },
  /// The window has lost keyboard focus.
  ///
  /// Inverse of [`Event::WindowGainedFocus`].
  WindowLostFocus {
    /// When the event happened
    timestamp: u32,
    /// The window which lost keyboard focus.
    window_id: u32,
  },
  /// The window has been moved.
  WindowMoved {
    /// When the event happened
    timestamp: u32,
    /// The window which experienced a size change.
    window_id: u32,
    /// The new x position of the window.
    x: i32,
    /// The new y position of the window.
    y: i32,
  },
  /// The size of the window has been changed externally.
  ///
  /// This event is always preceded by a [`Event::WindowSizeChanged`], however
  /// the inverse is not always true.
  ///
  /// The difference between these two events is that `Event::WindowResized` is
  /// only generated if the resize is triggered externally (by a user, their
  /// window manager, etc), and *not* by calls to `beryllium` functions which
  /// may change the size of a window, such as `Window::set_size`,
  /// `Window::set_display_mode`, whereas [`Event::WindowSizeChanged`] is called
  /// whenever the size of the window is changed, regardless of the cause.
  WindowResized {
    /// When the event happened
    timestamp: u32,
    /// The window which experienced a size change.
    window_id: u32,
    /// The new width of the window.
    width: i32,
    /// The new height of the window.
    height: i32,
  },
  /// Something has changed the size of the window.
  ///
  /// If (and only if) this was done by an external action (the user or window
  /// manager changing the size of the window, for example), this event will be
  /// followed by an [`Event::WindowResized`].
  WindowSizeChanged {
    /// When the event happened
    timestamp: u32,
    /// The window which experienced a size change.
    window_id: u32,
    /// The new width of the window.
    width: i32,
    /// The new height of the window.
    height: i32,
  },
  /// The window has been hidden.
  ///
  /// Inverse of [`Event::WindowShown`].
  WindowHidden {
    /// When the event happened
    timestamp: u32,
    /// The window which gained or lost mouse focus.
    window_id: u32,
  },
  /// The window has been minimized.
  WindowMinimized {
    /// When the event happened
    timestamp: u32,
    /// The window which has been minimized.
    window_id: u32,
  },
  /// The window needs repainting for one reason or another.
  ///
  /// This maps to `SDL_WINDOWEVENT_EXPOSED`, but has been renamed in order to
  /// make it more clear what the event indicates.
  WindowNeedsRepaint {
    /// When the event happened
    timestamp: u32,
    /// The window which gained or lost mouse focus.
    window_id: u32,
  },
  /// The window has been shown.
  ///
  /// Inverse of [`Event::WindowHidden`].
  WindowShown {
    /// When the event happened
    timestamp: u32,
    /// The window which gained or lost mouse focus.
    window_id: u32,
  },
  /// The window has been maximized.
  WindowMaximized {
    /// When the event happened
    timestamp: u32,
    /// The window which has been maximized.
    window_id: u32,
  },
  /// The window has been restored to normal size and position.
  WindowRestored {
    /// When the event happened
    timestamp: u32,
    /// The window which has been restored.
    window_id: u32,
  },
  /// It's always possible that we'll load some future version which will have
  /// event variants we don't understand, which we have to just ignore.
  UnknownEventType,
}
impl From<fermium::SDL_Event> for Event {
  /// Parses "without fail", but will turn unknown events into `UnknownEventType`.
  ///
  /// So, it's not lossless I guess. Whatever.
  fn from(event: fermium::SDL_Event) -> Self {
    unsafe {
      match event.type_ as fermium::SDL_EventType {
        SDL_QUIT => Event::Quit {
          timestamp: event.quit.timestamp,
        },
        SDL_MOUSEMOTION => Event::MouseMotion {
          timestamp: event.motion.timestamp,
          window_id: event.motion.windowID,
          mouse_id: if event.motion.which == fermium::SDL_TOUCH_MOUSEID {
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
        fermium::SDL_MOUSEBUTTONDOWN | fermium::SDL_MOUSEBUTTONUP => Event::MouseButtonEvent {
          timestamp: event.button.timestamp,
          window_id: event.button.windowID,
          mouse_id: if event.button.which == fermium::SDL_TOUCH_MOUSEID {
            None
          } else {
            Some(event.button.which)
          },
          button: MouseButton::from(event.button.button),
          is_pressed: u32::from(event.button.state) == fermium::SDL_PRESSED,
          clicks: event.button.clicks,
          x: event.button.x,
          y: event.button.y,
        },
        SDL_MOUSEWHEEL => Event::MouseWheel {
          timestamp: event.wheel.timestamp,
          window_id: event.wheel.windowID,
          mouse_id: if event.wheel.which == fermium::SDL_TOUCH_MOUSEID {
            None
          } else {
            Some(event.wheel.which)
          },
          x: event.wheel.x,
          y: event.wheel.y,
          is_flipped: event.wheel.direction as fermium::SDL_MouseWheelDirection
            == fermium::SDL_MOUSEWHEEL_FLIPPED,
        },
        fermium::SDL_KEYDOWN | fermium::SDL_KEYUP => Event::Keyboard {
          timestamp: event.key.timestamp,
          window_id: event.key.windowID,
          is_key_down: u32::from(event.key.state) == fermium::SDL_PRESSED,
          repeat_count: event.key.repeat,
          key_info: KeyInfo::from(event.key.keysym),
        },
        SDL_WINDOWEVENT => match event.window.event as fermium::SDL_WindowEventID {
          SDL_WINDOWEVENT_MOVED => Event::WindowMoved {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
            x: event.window.data1,
            y: event.window.data2,
          },
          SDL_WINDOWEVENT_RESIZED => Event::WindowResized {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
            width: event.window.data1,
            height: event.window.data2,
          },
          SDL_WINDOWEVENT_SIZE_CHANGED => Event::WindowSizeChanged {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
            width: event.window.data1,
            height: event.window.data2,
          },
          SDL_WINDOWEVENT_CLOSE => Event::WindowClosed {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_SHOWN => Event::WindowShown {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_HIDDEN => Event::WindowHidden {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_EXPOSED => Event::WindowNeedsRepaint {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_MINIMIZED => Event::WindowMinimized {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_MAXIMIZED => Event::WindowMaximized {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_RESTORED => Event::WindowRestored {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_ENTER => Event::MouseEnteredWindow {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_LEAVE => Event::MouseLeftWindow {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_FOCUS_GAINED => Event::WindowGainedFocus {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_FOCUS_LOST => Event::WindowLostFocus {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_CONTROLLERAXISMOTION => Event::ControllerAxis {
            timestamp: event.caxis.timestamp,
            joystick_id: JoystickID(event.caxis.which),
            axis: ControllerAxis::from(event.caxis.axis),
            value: event.caxis.value,
          },
          _ => Event::UnknownEventType,
        },
        fermium::SDL_CONTROLLERBUTTONDOWN | fermium::SDL_CONTROLLERBUTTONUP => Event::ControllerButton {
          timestamp: event.cbutton.timestamp,
          joystick_id: JoystickID(event.cbutton.which),
          button: ControllerButton::from(event.cbutton.button),
          pressed: u32::from(event.cbutton.state) == fermium::SDL_PRESSED,
        },
        SDL_CONTROLLERDEVICEADDED => Event::ControllerDeviceAdded {
          timestamp: event.cdevice.timestamp,
          joystick_id: JoystickID(event.cdevice.which),
        },
        SDL_CONTROLLERDEVICEREMOVED => Event::ControllerDeviceRemoved {
          timestamp: event.cdevice.timestamp,
          joystick_id: JoystickID(event.cdevice.which),
        },
        SDL_CONTROLLERDEVICEREMAPPED => Event::ControllerDeviceRemapped {
          timestamp: event.cdevice.timestamp,
          joystick_id: JoystickID(event.cdevice.which),
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
      SDL_BUTTON_X2 => MouseButton::X2,
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
const SDL_BUTTON_LMASK: u32 = sdl_button!(fermium::SDL_BUTTON_LEFT);
const SDL_BUTTON_MMASK: u32 = sdl_button!(fermium::SDL_BUTTON_MIDDLE);
const SDL_BUTTON_RMASK: u32 = sdl_button!(fermium::SDL_BUTTON_RIGHT);
const SDL_BUTTON_X1MASK: u32 = sdl_button!(fermium::SDL_BUTTON_X1);
const SDL_BUTTON_X2MASK: u32 = sdl_button!(fermium::SDL_BUTTON_X2);

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

/// Information
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KeyInfo {
  /// The scancode (if any) is the physical key pressed.
  pub scancode: Option<Scancode>,
  /// The keycode (if any) is the logical key pressed.
  pub keycode: Option<Keycode>,
  /// The key modifiers that were active during this event.
  pub modifiers: KeyModifiers,
}
impl From<fermium::SDL_Keysym> for KeyInfo {
  fn from(keysym: fermium::SDL_Keysym) -> Self {
    Self {
      scancode: Scancode::try_from(keysym.scancode).ok(),
      keycode: Keycode::try_from(keysym.sym).ok(),
      modifiers: KeyModifiers(keysym.mod_),
    }
  }
}

/// Different keycode values that can come in.
///
/// A keycode is a "virtual" key value, and which key on the keyboard counts as
/// which keycode can change according to the user's software configuration.
/// Note that with most layouts there are keys that don't have any Keycode
/// associated with them. Also note that not all Keycode values can be produced
/// by a given layout.
///
/// Some names have abbreviations:
///
/// * AC = Application Control
/// * Kbd = Keyboard
/// * KP = Keypad
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
pub enum Keycode {
  _0 = fermium::SDLK_0,
  _1 = fermium::SDLK_1,
  _2 = fermium::SDLK_2,
  _3 = fermium::SDLK_3,
  _4 = fermium::SDLK_4,
  _5 = fermium::SDLK_5,
  _6 = fermium::SDLK_6,
  _7 = fermium::SDLK_7,
  _8 = fermium::SDLK_8,
  _9 = fermium::SDLK_9,
  AC_Back = fermium::SDLK_AC_BACK,
  AC_Bookmarks = fermium::SDLK_AC_BOOKMARKS,
  AC_Forward = fermium::SDLK_AC_FORWARD,
  AC_Home = fermium::SDLK_AC_HOME,
  AC_Refresh = fermium::SDLK_AC_REFRESH,
  AC_Search = fermium::SDLK_AC_SEARCH,
  AC_Stop = fermium::SDLK_AC_STOP,
  Again = fermium::SDLK_AGAIN,
  AltErase = fermium::SDLK_ALTERASE,
  Ampersand = fermium::SDLK_AMPERSAND,
  App1 = fermium::SDLK_APP1,
  App2 = fermium::SDLK_APP2,
  Application = fermium::SDLK_APPLICATION,
  Asterisk = fermium::SDLK_ASTERISK,
  At = fermium::SDLK_AT,
  AudioFastForward = fermium::SDLK_AUDIOFASTFORWARD,
  AudioMute = fermium::SDLK_AUDIOMUTE,
  AudioNext = fermium::SDLK_AUDIONEXT,
  AudioPlay = fermium::SDLK_AUDIOPLAY,
  AudioPrev = fermium::SDLK_AUDIOPREV,
  AudioRewind = fermium::SDLK_AUDIOREWIND,
  AudioStop = fermium::SDLK_AUDIOSTOP,
  Backquote = fermium::SDLK_BACKQUOTE,
  Backslash = fermium::SDLK_BACKSLASH,
  Backspace = fermium::SDLK_BACKSPACE,
  BrightnessDown = fermium::SDLK_BRIGHTNESSDOWN,
  BrightnessUp = fermium::SDLK_BRIGHTNESSUP,
  Calculator = fermium::SDLK_CALCULATOR,
  Cancel = fermium::SDLK_CANCEL,
  CapsLock = fermium::SDLK_CAPSLOCK,
  Caret = fermium::SDLK_CARET,
  Clear = fermium::SDLK_CLEAR,
  ClearAgain = fermium::SDLK_CLEARAGAIN,
  Colon = fermium::SDLK_COLON,
  Comma = fermium::SDLK_COMMA,
  Computer = fermium::SDLK_COMPUTER,
  Copy = fermium::SDLK_COPY,
  CrSel = fermium::SDLK_CRSEL,
  CurrencySubUnit = fermium::SDLK_CURRENCYSUBUNIT,
  CurrencyUnit = fermium::SDLK_CURRENCYUNIT,
  Cut = fermium::SDLK_CUT,
  DecimalSeparator = fermium::SDLK_DECIMALSEPARATOR,
  Delete = fermium::SDLK_DELETE,
  DisplaySwitch = fermium::SDLK_DISPLAYSWITCH,
  Dollar = fermium::SDLK_DOLLAR,
  Down = fermium::SDLK_DOWN,
  Eject = fermium::SDLK_EJECT,
  End = fermium::SDLK_END,
  Equals = fermium::SDLK_EQUALS,
  Escape = fermium::SDLK_ESCAPE,
  Exclamation = fermium::SDLK_EXCLAIM,
  Execute = fermium::SDLK_EXECUTE,
  ExSel = fermium::SDLK_EXSEL,
  F1 = fermium::SDLK_F1,
  F2 = fermium::SDLK_F2,
  F3 = fermium::SDLK_F3,
  F4 = fermium::SDLK_F4,
  F5 = fermium::SDLK_F5,
  F6 = fermium::SDLK_F6,
  F7 = fermium::SDLK_F7,
  F8 = fermium::SDLK_F8,
  F9 = fermium::SDLK_F9,
  F10 = fermium::SDLK_F10,
  F11 = fermium::SDLK_F11,
  F12 = fermium::SDLK_F12,
  F13 = fermium::SDLK_F13,
  F14 = fermium::SDLK_F14,
  F15 = fermium::SDLK_F15,
  F16 = fermium::SDLK_F16,
  F17 = fermium::SDLK_F17,
  F18 = fermium::SDLK_F18,
  F19 = fermium::SDLK_F19,
  F20 = fermium::SDLK_F20,
  F21 = fermium::SDLK_F21,
  F22 = fermium::SDLK_F22,
  F23 = fermium::SDLK_F23,
  F24 = fermium::SDLK_F24,
  Find = fermium::SDLK_FIND,
  Greater = fermium::SDLK_GREATER,
  Hash = fermium::SDLK_HASH,
  Help = fermium::SDLK_HELP,
  Home = fermium::SDLK_HOME,
  Insert = fermium::SDLK_INSERT,
  KbdIlluminationDown = fermium::SDLK_KBDILLUMDOWN,
  KbdIlluminationToggle = fermium::SDLK_KBDILLUMTOGGLE,
  KbdIlluminationUp = fermium::SDLK_KBDILLUMUP,
  KP_0 = fermium::SDLK_KP_0,
  KP_00 = fermium::SDLK_KP_00,
  KP_000 = fermium::SDLK_KP_000,
  KP_1 = fermium::SDLK_KP_1,
  KP_2 = fermium::SDLK_KP_2,
  KP_3 = fermium::SDLK_KP_3,
  KP_4 = fermium::SDLK_KP_4,
  KP_5 = fermium::SDLK_KP_5,
  KP_6 = fermium::SDLK_KP_6,
  KP_7 = fermium::SDLK_KP_7,
  KP_8 = fermium::SDLK_KP_8,
  KP_9 = fermium::SDLK_KP_9,
  KP_A = fermium::SDLK_KP_A,
  KP_Ampersand = fermium::SDLK_KP_AMPERSAND,
  KP_At = fermium::SDLK_KP_AT,
  KP_B = fermium::SDLK_KP_B,
  KP_Backspace = fermium::SDLK_KP_BACKSPACE,
  KP_Binary = fermium::SDLK_KP_BINARY,
  KP_C = fermium::SDLK_KP_C,
  KP_Clear = fermium::SDLK_KP_CLEAR,
  KP_ClearEntry = fermium::SDLK_KP_CLEARENTRY,
  KP_Colon = fermium::SDLK_KP_COLON,
  KP_Comma = fermium::SDLK_KP_COMMA,
  KP_D = fermium::SDLK_KP_D,
  KP_DblAmpersand = fermium::SDLK_KP_DBLAMPERSAND,
  KP_DblVerticalBar = fermium::SDLK_KP_DBLVERTICALBAR,
  KP_Decimal = fermium::SDLK_KP_DECIMAL,
  KP_Divide = fermium::SDLK_KP_DIVIDE,
  KP_E = fermium::SDLK_KP_E,
  KP_Enter = fermium::SDLK_KP_ENTER,
  KP_Equals = fermium::SDLK_KP_EQUALS,
  KP_EqualsAs400 = fermium::SDLK_KP_EQUALSAS400,
  KP_Exclamation = fermium::SDLK_KP_EXCLAM,
  KP_F = fermium::SDLK_KP_F,
  KP_Greater = fermium::SDLK_KP_GREATER,
  KP_Hash = fermium::SDLK_KP_HASH,
  KP_Hexadecimal = fermium::SDLK_KP_HEXADECIMAL,
  KP_LeftBrace = fermium::SDLK_KP_LEFTBRACE,
  KP_LeftParen = fermium::SDLK_KP_LEFTPAREN,
  KP_Less = fermium::SDLK_KP_LESS,
  KP_MemAdd = fermium::SDLK_KP_MEMADD,
  KP_MemClear = fermium::SDLK_KP_MEMCLEAR,
  KP_MemDivide = fermium::SDLK_KP_MEMDIVIDE,
  KP_MemMultiply = fermium::SDLK_KP_MEMMULTIPLY,
  KP_MemRecall = fermium::SDLK_KP_MEMRECALL,
  KP_MemStore = fermium::SDLK_KP_MEMSTORE,
  KP_MemSubtract = fermium::SDLK_KP_MEMSUBTRACT,
  KP_Minus = fermium::SDLK_KP_MINUS,
  KP_Multiply = fermium::SDLK_KP_MULTIPLY,
  KP_Octal = fermium::SDLK_KP_OCTAL,
  KP_Percent = fermium::SDLK_KP_PERCENT,
  KP_Period = fermium::SDLK_KP_PERIOD,
  KP_Plus = fermium::SDLK_KP_PLUS,
  KP_PlusMinus = fermium::SDLK_KP_PLUSMINUS,
  KP_Power = fermium::SDLK_KP_POWER,
  KP_RightBrace = fermium::SDLK_KP_RIGHTBRACE,
  KP_RightParen = fermium::SDLK_KP_RIGHTPAREN,
  KP_Space = fermium::SDLK_KP_SPACE,
  KP_Tab = fermium::SDLK_KP_TAB,
  KP_VerticalBar = fermium::SDLK_KP_VERTICALBAR,
  KP_Xor = fermium::SDLK_KP_XOR,
  LeftAlt = fermium::SDLK_LALT,
  LeftCtrl = fermium::SDLK_LCTRL,
  Left = fermium::SDLK_LEFT,
  LeftBracket = fermium::SDLK_LEFTBRACKET,
  LeftParen = fermium::SDLK_LEFTPAREN,
  Less = fermium::SDLK_LESS,
  LeftGUI = fermium::SDLK_LGUI,
  LeftShift = fermium::SDLK_LSHIFT,
  Mail = fermium::SDLK_MAIL,
  MediaSelect = fermium::SDLK_MEDIASELECT,
  Menu = fermium::SDLK_MENU,
  Minus = fermium::SDLK_MINUS,
  Mode = fermium::SDLK_MODE,
  Mute = fermium::SDLK_MUTE,
  NumLockClear = fermium::SDLK_NUMLOCKCLEAR,
  Oper = fermium::SDLK_OPER,
  Out = fermium::SDLK_OUT,
  PageDown = fermium::SDLK_PAGEDOWN,
  PageUp = fermium::SDLK_PAGEUP,
  Paste = fermium::SDLK_PASTE,
  Pause = fermium::SDLK_PAUSE,
  Percent = fermium::SDLK_PERCENT,
  Period = fermium::SDLK_PERIOD,
  Plus = fermium::SDLK_PLUS,
  Power = fermium::SDLK_POWER,
  PrintScreen = fermium::SDLK_PRINTSCREEN,
  Prior = fermium::SDLK_PRIOR,
  Question = fermium::SDLK_QUESTION,
  Quote = fermium::SDLK_QUOTE,
  DblQuote = fermium::SDLK_QUOTEDBL,
  RightAlt = fermium::SDLK_RALT,
  RightCtrl = fermium::SDLK_RCTRL,
  Return = fermium::SDLK_RETURN,
  Return2 = fermium::SDLK_RETURN2,
  RightGUI = fermium::SDLK_RGUI,
  Right = fermium::SDLK_RIGHT,
  RightBracket = fermium::SDLK_RIGHTBRACKET,
  RightParen = fermium::SDLK_RIGHTPAREN,
  RightShift = fermium::SDLK_RSHIFT,
  ScrollLock = fermium::SDLK_SCROLLLOCK,
  Select = fermium::SDLK_SELECT,
  Semicolon = fermium::SDLK_SEMICOLON,
  Separator = fermium::SDLK_SEPARATOR,
  Slash = fermium::SDLK_SLASH,
  Sleep = fermium::SDLK_SLEEP,
  Space = fermium::SDLK_SPACE,
  Stop = fermium::SDLK_STOP,
  SysReq = fermium::SDLK_SYSREQ,
  Tab = fermium::SDLK_TAB,
  ThousandsSeparator = fermium::SDLK_THOUSANDSSEPARATOR,
  Underscore = fermium::SDLK_UNDERSCORE,
  Undo = fermium::SDLK_UNDO,
  Unknown = fermium::SDLK_UNKNOWN,
  Up = fermium::SDLK_UP,
  VolumeDown = fermium::SDLK_VOLUMEDOWN,
  VolumeUp = fermium::SDLK_VOLUMEUP,
  WWW = fermium::SDLK_WWW,
  A = fermium::SDLK_a,
  B = fermium::SDLK_b,
  C = fermium::SDLK_c,
  D = fermium::SDLK_d,
  E = fermium::SDLK_e,
  F = fermium::SDLK_f,
  G = fermium::SDLK_g,
  H = fermium::SDLK_h,
  I = fermium::SDLK_i,
  J = fermium::SDLK_j,
  K = fermium::SDLK_k,
  L = fermium::SDLK_l,
  M = fermium::SDLK_m,
  N = fermium::SDLK_n,
  O = fermium::SDLK_o,
  P = fermium::SDLK_p,
  Q = fermium::SDLK_q,
  R = fermium::SDLK_r,
  S = fermium::SDLK_s,
  T = fermium::SDLK_t,
  U = fermium::SDLK_u,
  V = fermium::SDLK_v,
  W = fermium::SDLK_w,
  X = fermium::SDLK_x,
  Y = fermium::SDLK_y,
  Z = fermium::SDLK_z,
}
impl TryFrom<i32> for Keycode {
  type Error = ();
  fn try_from(t: i32) -> Result<Self, Self::Error> {
    #[allow(non_upper_case_globals)]
    match t as fermium::SDLK {
      fermium::SDLK_0 => Ok(Keycode::_0),
      fermium::SDLK_1 => Ok(Keycode::_1),
      fermium::SDLK_2 => Ok(Keycode::_2),
      fermium::SDLK_3 => Ok(Keycode::_3),
      fermium::SDLK_4 => Ok(Keycode::_4),
      fermium::SDLK_5 => Ok(Keycode::_5),
      fermium::SDLK_6 => Ok(Keycode::_6),
      fermium::SDLK_7 => Ok(Keycode::_7),
      fermium::SDLK_8 => Ok(Keycode::_8),
      fermium::SDLK_9 => Ok(Keycode::_9),
      fermium::SDLK_AC_BACK => Ok(Keycode::AC_Back),
      fermium::SDLK_AC_BOOKMARKS => Ok(Keycode::AC_Bookmarks),
      fermium::SDLK_AC_FORWARD => Ok(Keycode::AC_Forward),
      fermium::SDLK_AC_HOME => Ok(Keycode::AC_Home),
      fermium::SDLK_AC_REFRESH => Ok(Keycode::AC_Refresh),
      fermium::SDLK_AC_SEARCH => Ok(Keycode::AC_Search),
      fermium::SDLK_AC_STOP => Ok(Keycode::AC_Stop),
      fermium::SDLK_AGAIN => Ok(Keycode::Again),
      fermium::SDLK_ALTERASE => Ok(Keycode::AltErase),
      fermium::SDLK_AMPERSAND => Ok(Keycode::Ampersand),
      fermium::SDLK_APP1 => Ok(Keycode::App1),
      fermium::SDLK_APP2 => Ok(Keycode::App2),
      fermium::SDLK_APPLICATION => Ok(Keycode::Application),
      fermium::SDLK_ASTERISK => Ok(Keycode::Asterisk),
      fermium::SDLK_AT => Ok(Keycode::At),
      fermium::SDLK_AUDIOFASTFORWARD => Ok(Keycode::AudioFastForward),
      fermium::SDLK_AUDIOMUTE => Ok(Keycode::AudioMute),
      fermium::SDLK_AUDIONEXT => Ok(Keycode::AudioNext),
      fermium::SDLK_AUDIOPLAY => Ok(Keycode::AudioPlay),
      fermium::SDLK_AUDIOPREV => Ok(Keycode::AudioPrev),
      fermium::SDLK_AUDIOREWIND => Ok(Keycode::AudioRewind),
      fermium::SDLK_AUDIOSTOP => Ok(Keycode::AudioStop),
      fermium::SDLK_BACKQUOTE => Ok(Keycode::Backquote),
      fermium::SDLK_BACKSLASH => Ok(Keycode::Backslash),
      fermium::SDLK_BACKSPACE => Ok(Keycode::Backspace),
      fermium::SDLK_BRIGHTNESSDOWN => Ok(Keycode::BrightnessDown),
      fermium::SDLK_BRIGHTNESSUP => Ok(Keycode::BrightnessUp),
      fermium::SDLK_CALCULATOR => Ok(Keycode::Calculator),
      fermium::SDLK_CANCEL => Ok(Keycode::Cancel),
      fermium::SDLK_CAPSLOCK => Ok(Keycode::CapsLock),
      fermium::SDLK_CARET => Ok(Keycode::Caret),
      fermium::SDLK_CLEAR => Ok(Keycode::Clear),
      fermium::SDLK_CLEARAGAIN => Ok(Keycode::ClearAgain),
      fermium::SDLK_COLON => Ok(Keycode::Colon),
      fermium::SDLK_COMMA => Ok(Keycode::Comma),
      fermium::SDLK_COMPUTER => Ok(Keycode::Computer),
      fermium::SDLK_COPY => Ok(Keycode::Copy),
      fermium::SDLK_CRSEL => Ok(Keycode::CrSel),
      fermium::SDLK_CURRENCYSUBUNIT => Ok(Keycode::CurrencySubUnit),
      fermium::SDLK_CURRENCYUNIT => Ok(Keycode::CurrencyUnit),
      fermium::SDLK_CUT => Ok(Keycode::Cut),
      fermium::SDLK_DECIMALSEPARATOR => Ok(Keycode::DecimalSeparator),
      fermium::SDLK_DELETE => Ok(Keycode::Delete),
      fermium::SDLK_DISPLAYSWITCH => Ok(Keycode::DisplaySwitch),
      fermium::SDLK_DOLLAR => Ok(Keycode::Dollar),
      fermium::SDLK_DOWN => Ok(Keycode::Down),
      fermium::SDLK_EJECT => Ok(Keycode::Eject),
      fermium::SDLK_END => Ok(Keycode::End),
      fermium::SDLK_EQUALS => Ok(Keycode::Equals),
      fermium::SDLK_ESCAPE => Ok(Keycode::Escape),
      fermium::SDLK_EXCLAIM => Ok(Keycode::Exclamation),
      fermium::SDLK_EXECUTE => Ok(Keycode::Execute),
      fermium::SDLK_EXSEL => Ok(Keycode::ExSel),
      fermium::SDLK_F1 => Ok(Keycode::F1),
      fermium::SDLK_F2 => Ok(Keycode::F2),
      fermium::SDLK_F3 => Ok(Keycode::F3),
      fermium::SDLK_F4 => Ok(Keycode::F4),
      fermium::SDLK_F5 => Ok(Keycode::F5),
      fermium::SDLK_F6 => Ok(Keycode::F6),
      fermium::SDLK_F7 => Ok(Keycode::F7),
      fermium::SDLK_F8 => Ok(Keycode::F8),
      fermium::SDLK_F9 => Ok(Keycode::F9),
      fermium::SDLK_F10 => Ok(Keycode::F10),
      fermium::SDLK_F11 => Ok(Keycode::F11),
      fermium::SDLK_F12 => Ok(Keycode::F12),
      fermium::SDLK_F13 => Ok(Keycode::F13),
      fermium::SDLK_F14 => Ok(Keycode::F14),
      fermium::SDLK_F15 => Ok(Keycode::F15),
      fermium::SDLK_F16 => Ok(Keycode::F16),
      fermium::SDLK_F17 => Ok(Keycode::F17),
      fermium::SDLK_F18 => Ok(Keycode::F18),
      fermium::SDLK_F19 => Ok(Keycode::F19),
      fermium::SDLK_F20 => Ok(Keycode::F20),
      fermium::SDLK_F21 => Ok(Keycode::F21),
      fermium::SDLK_F22 => Ok(Keycode::F22),
      fermium::SDLK_F23 => Ok(Keycode::F23),
      fermium::SDLK_F24 => Ok(Keycode::F24),
      fermium::SDLK_FIND => Ok(Keycode::Find),
      fermium::SDLK_GREATER => Ok(Keycode::Greater),
      fermium::SDLK_HASH => Ok(Keycode::Hash),
      fermium::SDLK_HELP => Ok(Keycode::Help),
      fermium::SDLK_HOME => Ok(Keycode::Home),
      fermium::SDLK_INSERT => Ok(Keycode::Insert),
      fermium::SDLK_KBDILLUMDOWN => Ok(Keycode::KbdIlluminationDown),
      fermium::SDLK_KBDILLUMTOGGLE => Ok(Keycode::KbdIlluminationToggle),
      fermium::SDLK_KBDILLUMUP => Ok(Keycode::KbdIlluminationUp),
      fermium::SDLK_KP_0 => Ok(Keycode::KP_0),
      fermium::SDLK_KP_00 => Ok(Keycode::KP_00),
      fermium::SDLK_KP_000 => Ok(Keycode::KP_000),
      fermium::SDLK_KP_1 => Ok(Keycode::KP_1),
      fermium::SDLK_KP_2 => Ok(Keycode::KP_2),
      fermium::SDLK_KP_3 => Ok(Keycode::KP_3),
      fermium::SDLK_KP_4 => Ok(Keycode::KP_4),
      fermium::SDLK_KP_5 => Ok(Keycode::KP_5),
      fermium::SDLK_KP_6 => Ok(Keycode::KP_6),
      fermium::SDLK_KP_7 => Ok(Keycode::KP_7),
      fermium::SDLK_KP_8 => Ok(Keycode::KP_8),
      fermium::SDLK_KP_9 => Ok(Keycode::KP_9),
      fermium::SDLK_KP_A => Ok(Keycode::KP_A),
      fermium::SDLK_KP_AMPERSAND => Ok(Keycode::KP_Ampersand),
      fermium::SDLK_KP_AT => Ok(Keycode::KP_At),
      fermium::SDLK_KP_B => Ok(Keycode::KP_B),
      fermium::SDLK_KP_BACKSPACE => Ok(Keycode::KP_Backspace),
      fermium::SDLK_KP_BINARY => Ok(Keycode::KP_Binary),
      fermium::SDLK_KP_C => Ok(Keycode::KP_C),
      fermium::SDLK_KP_CLEAR => Ok(Keycode::KP_Clear),
      fermium::SDLK_KP_CLEARENTRY => Ok(Keycode::KP_ClearEntry),
      fermium::SDLK_KP_COLON => Ok(Keycode::KP_Colon),
      fermium::SDLK_KP_COMMA => Ok(Keycode::KP_Comma),
      fermium::SDLK_KP_D => Ok(Keycode::KP_D),
      fermium::SDLK_KP_DBLAMPERSAND => Ok(Keycode::KP_DblAmpersand),
      fermium::SDLK_KP_DBLVERTICALBAR => Ok(Keycode::KP_DblVerticalBar),
      fermium::SDLK_KP_DECIMAL => Ok(Keycode::KP_Decimal),
      fermium::SDLK_KP_DIVIDE => Ok(Keycode::KP_Divide),
      fermium::SDLK_KP_E => Ok(Keycode::KP_E),
      fermium::SDLK_KP_ENTER => Ok(Keycode::KP_Enter),
      fermium::SDLK_KP_EQUALS => Ok(Keycode::KP_Equals),
      fermium::SDLK_KP_EQUALSAS400 => Ok(Keycode::KP_EqualsAs400),
      fermium::SDLK_KP_EXCLAM => Ok(Keycode::KP_Exclamation),
      fermium::SDLK_KP_F => Ok(Keycode::KP_F),
      fermium::SDLK_KP_GREATER => Ok(Keycode::KP_Greater),
      fermium::SDLK_KP_HASH => Ok(Keycode::KP_Hash),
      fermium::SDLK_KP_HEXADECIMAL => Ok(Keycode::KP_Hexadecimal),
      fermium::SDLK_KP_LEFTBRACE => Ok(Keycode::KP_LeftBrace),
      fermium::SDLK_KP_LEFTPAREN => Ok(Keycode::KP_LeftParen),
      fermium::SDLK_KP_LESS => Ok(Keycode::KP_Less),
      fermium::SDLK_KP_MEMADD => Ok(Keycode::KP_MemAdd),
      fermium::SDLK_KP_MEMCLEAR => Ok(Keycode::KP_MemClear),
      fermium::SDLK_KP_MEMDIVIDE => Ok(Keycode::KP_MemDivide),
      fermium::SDLK_KP_MEMMULTIPLY => Ok(Keycode::KP_MemMultiply),
      fermium::SDLK_KP_MEMRECALL => Ok(Keycode::KP_MemRecall),
      fermium::SDLK_KP_MEMSTORE => Ok(Keycode::KP_MemStore),
      fermium::SDLK_KP_MEMSUBTRACT => Ok(Keycode::KP_MemSubtract),
      fermium::SDLK_KP_MINUS => Ok(Keycode::KP_Minus),
      fermium::SDLK_KP_MULTIPLY => Ok(Keycode::KP_Multiply),
      fermium::SDLK_KP_OCTAL => Ok(Keycode::KP_Octal),
      fermium::SDLK_KP_PERCENT => Ok(Keycode::KP_Percent),
      fermium::SDLK_KP_PERIOD => Ok(Keycode::KP_Period),
      fermium::SDLK_KP_PLUS => Ok(Keycode::KP_Plus),
      fermium::SDLK_KP_PLUSMINUS => Ok(Keycode::KP_PlusMinus),
      fermium::SDLK_KP_POWER => Ok(Keycode::KP_Power),
      fermium::SDLK_KP_RIGHTBRACE => Ok(Keycode::KP_RightBrace),
      fermium::SDLK_KP_RIGHTPAREN => Ok(Keycode::KP_RightParen),
      fermium::SDLK_KP_SPACE => Ok(Keycode::KP_Space),
      fermium::SDLK_KP_TAB => Ok(Keycode::KP_Tab),
      fermium::SDLK_KP_VERTICALBAR => Ok(Keycode::KP_VerticalBar),
      fermium::SDLK_KP_XOR => Ok(Keycode::KP_Xor),
      fermium::SDLK_LALT => Ok(Keycode::LeftAlt),
      fermium::SDLK_LCTRL => Ok(Keycode::LeftCtrl),
      fermium::SDLK_LEFT => Ok(Keycode::Left),
      fermium::SDLK_LEFTBRACKET => Ok(Keycode::LeftBracket),
      fermium::SDLK_LEFTPAREN => Ok(Keycode::LeftParen),
      fermium::SDLK_LESS => Ok(Keycode::Less),
      fermium::SDLK_LGUI => Ok(Keycode::LeftGUI),
      fermium::SDLK_LSHIFT => Ok(Keycode::LeftShift),
      fermium::SDLK_MAIL => Ok(Keycode::Mail),
      fermium::SDLK_MEDIASELECT => Ok(Keycode::MediaSelect),
      fermium::SDLK_MENU => Ok(Keycode::Menu),
      fermium::SDLK_MINUS => Ok(Keycode::Minus),
      fermium::SDLK_MODE => Ok(Keycode::Mode),
      fermium::SDLK_MUTE => Ok(Keycode::Mute),
      fermium::SDLK_NUMLOCKCLEAR => Ok(Keycode::NumLockClear),
      fermium::SDLK_OPER => Ok(Keycode::Oper),
      fermium::SDLK_OUT => Ok(Keycode::Out),
      fermium::SDLK_PAGEDOWN => Ok(Keycode::PageDown),
      fermium::SDLK_PAGEUP => Ok(Keycode::PageUp),
      fermium::SDLK_PASTE => Ok(Keycode::Paste),
      fermium::SDLK_PAUSE => Ok(Keycode::Pause),
      fermium::SDLK_PERCENT => Ok(Keycode::Percent),
      fermium::SDLK_PERIOD => Ok(Keycode::Period),
      fermium::SDLK_PLUS => Ok(Keycode::Plus),
      fermium::SDLK_POWER => Ok(Keycode::Power),
      fermium::SDLK_PRINTSCREEN => Ok(Keycode::PrintScreen),
      fermium::SDLK_PRIOR => Ok(Keycode::Prior),
      fermium::SDLK_QUESTION => Ok(Keycode::Question),
      fermium::SDLK_QUOTE => Ok(Keycode::Quote),
      fermium::SDLK_QUOTEDBL => Ok(Keycode::DblQuote),
      fermium::SDLK_RALT => Ok(Keycode::RightAlt),
      fermium::SDLK_RCTRL => Ok(Keycode::RightCtrl),
      fermium::SDLK_RETURN => Ok(Keycode::Return),
      fermium::SDLK_RETURN2 => Ok(Keycode::Return2),
      fermium::SDLK_RGUI => Ok(Keycode::RightGUI),
      fermium::SDLK_RIGHT => Ok(Keycode::Right),
      fermium::SDLK_RIGHTBRACKET => Ok(Keycode::RightBracket),
      fermium::SDLK_RIGHTPAREN => Ok(Keycode::RightParen),
      fermium::SDLK_RSHIFT => Ok(Keycode::RightShift),
      fermium::SDLK_SCROLLLOCK => Ok(Keycode::ScrollLock),
      fermium::SDLK_SELECT => Ok(Keycode::Select),
      fermium::SDLK_SEMICOLON => Ok(Keycode::Semicolon),
      fermium::SDLK_SEPARATOR => Ok(Keycode::Separator),
      fermium::SDLK_SLASH => Ok(Keycode::Slash),
      fermium::SDLK_SLEEP => Ok(Keycode::Sleep),
      fermium::SDLK_SPACE => Ok(Keycode::Space),
      fermium::SDLK_STOP => Ok(Keycode::Stop),
      fermium::SDLK_SYSREQ => Ok(Keycode::SysReq),
      fermium::SDLK_TAB => Ok(Keycode::Tab),
      fermium::SDLK_THOUSANDSSEPARATOR => Ok(Keycode::ThousandsSeparator),
      fermium::SDLK_UNDERSCORE => Ok(Keycode::Underscore),
      fermium::SDLK_UNDO => Ok(Keycode::Undo),
      fermium::SDLK_UNKNOWN => Ok(Keycode::Unknown),
      fermium::SDLK_UP => Ok(Keycode::Up),
      fermium::SDLK_VOLUMEDOWN => Ok(Keycode::VolumeDown),
      fermium::SDLK_VOLUMEUP => Ok(Keycode::VolumeUp),
      fermium::SDLK_WWW => Ok(Keycode::WWW),
      fermium::SDLK_a => Ok(Keycode::A),
      fermium::SDLK_b => Ok(Keycode::B),
      fermium::SDLK_c => Ok(Keycode::C),
      fermium::SDLK_d => Ok(Keycode::D),
      fermium::SDLK_e => Ok(Keycode::E),
      fermium::SDLK_f => Ok(Keycode::F),
      fermium::SDLK_g => Ok(Keycode::G),
      fermium::SDLK_h => Ok(Keycode::H),
      fermium::SDLK_i => Ok(Keycode::I),
      fermium::SDLK_j => Ok(Keycode::J),
      fermium::SDLK_k => Ok(Keycode::K),
      fermium::SDLK_l => Ok(Keycode::L),
      fermium::SDLK_m => Ok(Keycode::M),
      fermium::SDLK_n => Ok(Keycode::N),
      fermium::SDLK_o => Ok(Keycode::O),
      fermium::SDLK_p => Ok(Keycode::P),
      fermium::SDLK_q => Ok(Keycode::Q),
      fermium::SDLK_r => Ok(Keycode::R),
      fermium::SDLK_s => Ok(Keycode::S),
      fermium::SDLK_t => Ok(Keycode::T),
      fermium::SDLK_u => Ok(Keycode::U),
      fermium::SDLK_v => Ok(Keycode::V),
      fermium::SDLK_w => Ok(Keycode::W),
      fermium::SDLK_x => Ok(Keycode::X),
      fermium::SDLK_y => Ok(Keycode::Y),
      fermium::SDLK_z => Ok(Keycode::Z),
      _ => Err(()),
    }
  }
}
#[test]
pub fn test_sdlk_a() {
  assert_eq!(
    Keycode::try_from(fermium::SDLK_a as i32),
    Ok(Keycode::A)
  );
}

/// Different "scancode" values that can come in.
///
/// A scancode is a "physical" key value, and which key on the keyboard counts
/// as which scancode is fixed for each particular keyboard model. Note that not
/// all keyboard models can produce all Scancode values given here.
///
/// Some names have abbreviations:
///
/// * AC = Application Control
/// * Kbd = Keyboard
/// * KP = Keypad
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
pub enum Scancode {
  _0 = fermium::SDL_SCANCODE_0,
  _1 = fermium::SDL_SCANCODE_1,
  _2 = fermium::SDL_SCANCODE_2,
  _3 = fermium::SDL_SCANCODE_3,
  _4 = fermium::SDL_SCANCODE_4,
  _5 = fermium::SDL_SCANCODE_5,
  _6 = fermium::SDL_SCANCODE_6,
  _7 = fermium::SDL_SCANCODE_7,
  _8 = fermium::SDL_SCANCODE_8,
  _9 = fermium::SDL_SCANCODE_9,
  A = fermium::SDL_SCANCODE_A,
  AC_Back = fermium::SDL_SCANCODE_AC_BACK,
  AC_Bookmarks = fermium::SDL_SCANCODE_AC_BOOKMARKS,
  AC_Forward = fermium::SDL_SCANCODE_AC_FORWARD,
  AC_Home = fermium::SDL_SCANCODE_AC_HOME,
  AC_Refresh = fermium::SDL_SCANCODE_AC_REFRESH,
  AC_Search = fermium::SDL_SCANCODE_AC_SEARCH,
  AC_Stop = fermium::SDL_SCANCODE_AC_STOP,
  Again = fermium::SDL_SCANCODE_AGAIN,
  AltErase = fermium::SDL_SCANCODE_ALTERASE,
  Apostrophe = fermium::SDL_SCANCODE_APOSTROPHE,
  App1 = fermium::SDL_SCANCODE_APP1,
  App2 = fermium::SDL_SCANCODE_APP2,
  Application = fermium::SDL_SCANCODE_APPLICATION,
  AudioFastForward = fermium::SDL_SCANCODE_AUDIOFASTFORWARD,
  AudioMute = fermium::SDL_SCANCODE_AUDIOMUTE,
  AudioNext = fermium::SDL_SCANCODE_AUDIONEXT,
  AudioPlay = fermium::SDL_SCANCODE_AUDIOPLAY,
  AudioPrev = fermium::SDL_SCANCODE_AUDIOPREV,
  AudioRewind = fermium::SDL_SCANCODE_AUDIOREWIND,
  AudioStop = fermium::SDL_SCANCODE_AUDIOSTOP,
  B = fermium::SDL_SCANCODE_B,
  Backslash = fermium::SDL_SCANCODE_BACKSLASH,
  Backspace = fermium::SDL_SCANCODE_BACKSPACE,
  BrightnessDown = fermium::SDL_SCANCODE_BRIGHTNESSDOWN,
  BrightnessUp = fermium::SDL_SCANCODE_BRIGHTNESSUP,
  C = fermium::SDL_SCANCODE_C,
  Calculator = fermium::SDL_SCANCODE_CALCULATOR,
  Cancel = fermium::SDL_SCANCODE_CANCEL,
  CapsLock = fermium::SDL_SCANCODE_CAPSLOCK,
  Clear = fermium::SDL_SCANCODE_CLEAR,
  ClearAgain = fermium::SDL_SCANCODE_CLEARAGAIN,
  Comma = fermium::SDL_SCANCODE_COMMA,
  Computer = fermium::SDL_SCANCODE_COMPUTER,
  Copy = fermium::SDL_SCANCODE_COPY,
  CrSel = fermium::SDL_SCANCODE_CRSEL,
  CurrencySubUnit = fermium::SDL_SCANCODE_CURRENCYSUBUNIT,
  CurrencyUnit = fermium::SDL_SCANCODE_CURRENCYUNIT,
  Cut = fermium::SDL_SCANCODE_CUT,
  D = fermium::SDL_SCANCODE_D,
  DecimalSeparator = fermium::SDL_SCANCODE_DECIMALSEPARATOR,
  Delete = fermium::SDL_SCANCODE_DELETE,
  DisplaySwitch = fermium::SDL_SCANCODE_DISPLAYSWITCH,
  Down = fermium::SDL_SCANCODE_DOWN,
  E = fermium::SDL_SCANCODE_E,
  Eject = fermium::SDL_SCANCODE_EJECT,
  End = fermium::SDL_SCANCODE_END,
  Equals = fermium::SDL_SCANCODE_EQUALS,
  Escape = fermium::SDL_SCANCODE_ESCAPE,
  Execute = fermium::SDL_SCANCODE_EXECUTE,
  ExSel = fermium::SDL_SCANCODE_EXSEL,
  F = fermium::SDL_SCANCODE_F,
  F1 = fermium::SDL_SCANCODE_F1,
  F2 = fermium::SDL_SCANCODE_F2,
  F3 = fermium::SDL_SCANCODE_F3,
  F4 = fermium::SDL_SCANCODE_F4,
  F5 = fermium::SDL_SCANCODE_F5,
  F6 = fermium::SDL_SCANCODE_F6,
  F7 = fermium::SDL_SCANCODE_F7,
  F8 = fermium::SDL_SCANCODE_F8,
  F9 = fermium::SDL_SCANCODE_F9,
  F10 = fermium::SDL_SCANCODE_F10,
  F11 = fermium::SDL_SCANCODE_F11,
  F12 = fermium::SDL_SCANCODE_F12,
  F13 = fermium::SDL_SCANCODE_F13,
  F14 = fermium::SDL_SCANCODE_F14,
  F15 = fermium::SDL_SCANCODE_F15,
  F16 = fermium::SDL_SCANCODE_F16,
  F17 = fermium::SDL_SCANCODE_F17,
  F18 = fermium::SDL_SCANCODE_F18,
  F19 = fermium::SDL_SCANCODE_F19,
  F20 = fermium::SDL_SCANCODE_F20,
  F21 = fermium::SDL_SCANCODE_F21,
  F22 = fermium::SDL_SCANCODE_F22,
  F23 = fermium::SDL_SCANCODE_F23,
  F24 = fermium::SDL_SCANCODE_F24,
  Find = fermium::SDL_SCANCODE_FIND,
  G = fermium::SDL_SCANCODE_G,
  Grave = fermium::SDL_SCANCODE_GRAVE,
  H = fermium::SDL_SCANCODE_H,
  Help = fermium::SDL_SCANCODE_HELP,
  Home = fermium::SDL_SCANCODE_HOME,
  I = fermium::SDL_SCANCODE_I,
  Insert = fermium::SDL_SCANCODE_INSERT,
  International1 = fermium::SDL_SCANCODE_INTERNATIONAL1,
  International2 = fermium::SDL_SCANCODE_INTERNATIONAL2,
  International3 = fermium::SDL_SCANCODE_INTERNATIONAL3,
  International4 = fermium::SDL_SCANCODE_INTERNATIONAL4,
  International5 = fermium::SDL_SCANCODE_INTERNATIONAL5,
  International6 = fermium::SDL_SCANCODE_INTERNATIONAL6,
  International7 = fermium::SDL_SCANCODE_INTERNATIONAL7,
  International8 = fermium::SDL_SCANCODE_INTERNATIONAL8,
  International9 = fermium::SDL_SCANCODE_INTERNATIONAL9,
  J = fermium::SDL_SCANCODE_J,
  K = fermium::SDL_SCANCODE_K,
  KbdIlluminationDown = fermium::SDL_SCANCODE_KBDILLUMDOWN,
  KbdIlluminationToggle = fermium::SDL_SCANCODE_KBDILLUMTOGGLE,
  KbdIlluminationUp = fermium::SDL_SCANCODE_KBDILLUMUP,
  KP_0 = fermium::SDL_SCANCODE_KP_0,
  KP_00 = fermium::SDL_SCANCODE_KP_00,
  KP_000 = fermium::SDL_SCANCODE_KP_000,
  KP_1 = fermium::SDL_SCANCODE_KP_1,
  KP_2 = fermium::SDL_SCANCODE_KP_2,
  KP_3 = fermium::SDL_SCANCODE_KP_3,
  KP_4 = fermium::SDL_SCANCODE_KP_4,
  KP_5 = fermium::SDL_SCANCODE_KP_5,
  KP_6 = fermium::SDL_SCANCODE_KP_6,
  KP_7 = fermium::SDL_SCANCODE_KP_7,
  KP_8 = fermium::SDL_SCANCODE_KP_8,
  KP_9 = fermium::SDL_SCANCODE_KP_9,
  KP_A = fermium::SDL_SCANCODE_KP_A,
  KP_Ampersand = fermium::SDL_SCANCODE_KP_AMPERSAND,
  KP_At = fermium::SDL_SCANCODE_KP_AT,
  KP_B = fermium::SDL_SCANCODE_KP_B,
  KP_Backspace = fermium::SDL_SCANCODE_KP_BACKSPACE,
  KP_Binary = fermium::SDL_SCANCODE_KP_BINARY,
  KP_C = fermium::SDL_SCANCODE_KP_C,
  KP_Clear = fermium::SDL_SCANCODE_KP_CLEAR,
  KP_ClearEntry = fermium::SDL_SCANCODE_KP_CLEARENTRY,
  KP_Colon = fermium::SDL_SCANCODE_KP_COLON,
  KP_Comma = fermium::SDL_SCANCODE_KP_COMMA,
  KP_D = fermium::SDL_SCANCODE_KP_D,
  KP_DblAmpersand = fermium::SDL_SCANCODE_KP_DBLAMPERSAND,
  KP_DblVerticalBar = fermium::SDL_SCANCODE_KP_DBLVERTICALBAR,
  KP_Decimal = fermium::SDL_SCANCODE_KP_DECIMAL,
  KP_Divide = fermium::SDL_SCANCODE_KP_DIVIDE,
  KP_E = fermium::SDL_SCANCODE_KP_E,
  KP_Enter = fermium::SDL_SCANCODE_KP_ENTER,
  KP_Equals = fermium::SDL_SCANCODE_KP_EQUALS,
  KP_EqualsAs400 = fermium::SDL_SCANCODE_KP_EQUALSAS400,
  KP_Exclamation = fermium::SDL_SCANCODE_KP_EXCLAM,
  KP_F = fermium::SDL_SCANCODE_KP_F,
  KP_Greater = fermium::SDL_SCANCODE_KP_GREATER,
  KP_Hash = fermium::SDL_SCANCODE_KP_HASH,
  KP_Hexadecimal = fermium::SDL_SCANCODE_KP_HEXADECIMAL,
  KP_LeftBrace = fermium::SDL_SCANCODE_KP_LEFTBRACE,
  KP_LeftParen = fermium::SDL_SCANCODE_KP_LEFTPAREN,
  KP_Less = fermium::SDL_SCANCODE_KP_LESS,
  KP_MemAdd = fermium::SDL_SCANCODE_KP_MEMADD,
  KP_MemClear = fermium::SDL_SCANCODE_KP_MEMCLEAR,
  KP_MemDivide = fermium::SDL_SCANCODE_KP_MEMDIVIDE,
  KP_MemMultiply = fermium::SDL_SCANCODE_KP_MEMMULTIPLY,
  KP_MemRecall = fermium::SDL_SCANCODE_KP_MEMRECALL,
  KP_MemStore = fermium::SDL_SCANCODE_KP_MEMSTORE,
  KP_MemSubtract = fermium::SDL_SCANCODE_KP_MEMSUBTRACT,
  KP_Minus = fermium::SDL_SCANCODE_KP_MINUS,
  KP_Multiply = fermium::SDL_SCANCODE_KP_MULTIPLY,
  KP_Octal = fermium::SDL_SCANCODE_KP_OCTAL,
  KP_Percent = fermium::SDL_SCANCODE_KP_PERCENT,
  KP_Period = fermium::SDL_SCANCODE_KP_PERIOD,
  KP_Plus = fermium::SDL_SCANCODE_KP_PLUS,
  KP_PlusMinus = fermium::SDL_SCANCODE_KP_PLUSMINUS,
  KP_Power = fermium::SDL_SCANCODE_KP_POWER,
  KP_RightBrace = fermium::SDL_SCANCODE_KP_RIGHTBRACE,
  KP_RightParen = fermium::SDL_SCANCODE_KP_RIGHTPAREN,
  KP_Space = fermium::SDL_SCANCODE_KP_SPACE,
  KP_Tab = fermium::SDL_SCANCODE_KP_TAB,
  KP_VerticalBar = fermium::SDL_SCANCODE_KP_VERTICALBAR,
  KP_Xor = fermium::SDL_SCANCODE_KP_XOR,
  L = fermium::SDL_SCANCODE_L,
  LeftAlt = fermium::SDL_SCANCODE_LALT,
  Lang1 = fermium::SDL_SCANCODE_LANG1,
  Lang2 = fermium::SDL_SCANCODE_LANG2,
  Lang3 = fermium::SDL_SCANCODE_LANG3,
  Lang4 = fermium::SDL_SCANCODE_LANG4,
  Lang5 = fermium::SDL_SCANCODE_LANG5,
  Lang6 = fermium::SDL_SCANCODE_LANG6,
  Lang7 = fermium::SDL_SCANCODE_LANG7,
  Lang8 = fermium::SDL_SCANCODE_LANG8,
  Lang9 = fermium::SDL_SCANCODE_LANG9,
  LeftCtrl = fermium::SDL_SCANCODE_LCTRL,
  Left = fermium::SDL_SCANCODE_LEFT,
  LeftBracket = fermium::SDL_SCANCODE_LEFTBRACKET,
  LeftGUI = fermium::SDL_SCANCODE_LGUI,
  LeftShift = fermium::SDL_SCANCODE_LSHIFT,
  M = fermium::SDL_SCANCODE_M,
  Mail = fermium::SDL_SCANCODE_MAIL,
  Mediaselect = fermium::SDL_SCANCODE_MEDIASELECT,
  Menu = fermium::SDL_SCANCODE_MENU,
  Minus = fermium::SDL_SCANCODE_MINUS,
  Mode = fermium::SDL_SCANCODE_MODE,
  Mute = fermium::SDL_SCANCODE_MUTE,
  N = fermium::SDL_SCANCODE_N,
  NonUSBackslash = fermium::SDL_SCANCODE_NONUSBACKSLASH,
  NonUSHash = fermium::SDL_SCANCODE_NONUSHASH,
  NumLockClear = fermium::SDL_SCANCODE_NUMLOCKCLEAR,
  O = fermium::SDL_SCANCODE_O,
  Oper = fermium::SDL_SCANCODE_OPER,
  Out = fermium::SDL_SCANCODE_OUT,
  P = fermium::SDL_SCANCODE_P,
  PageDown = fermium::SDL_SCANCODE_PAGEDOWN,
  PageUp = fermium::SDL_SCANCODE_PAGEUP,
  Paste = fermium::SDL_SCANCODE_PASTE,
  Pause = fermium::SDL_SCANCODE_PAUSE,
  Period = fermium::SDL_SCANCODE_PERIOD,
  Power = fermium::SDL_SCANCODE_POWER,
  PrintScreen = fermium::SDL_SCANCODE_PRINTSCREEN,
  Prior = fermium::SDL_SCANCODE_PRIOR,
  Q = fermium::SDL_SCANCODE_Q,
  R = fermium::SDL_SCANCODE_R,
  RightAlt = fermium::SDL_SCANCODE_RALT,
  RightCtrl = fermium::SDL_SCANCODE_RCTRL,
  Return = fermium::SDL_SCANCODE_RETURN,
  Return2 = fermium::SDL_SCANCODE_RETURN2,
  RightGUI = fermium::SDL_SCANCODE_RGUI,
  Right = fermium::SDL_SCANCODE_RIGHT,
  RightBracket = fermium::SDL_SCANCODE_RIGHTBRACKET,
  RightShift = fermium::SDL_SCANCODE_RSHIFT,
  S = fermium::SDL_SCANCODE_S,
  ScrollLock = fermium::SDL_SCANCODE_SCROLLLOCK,
  Select = fermium::SDL_SCANCODE_SELECT,
  Semicolon = fermium::SDL_SCANCODE_SEMICOLON,
  Separator = fermium::SDL_SCANCODE_SEPARATOR,
  Slash = fermium::SDL_SCANCODE_SLASH,
  Sleep = fermium::SDL_SCANCODE_SLEEP,
  Space = fermium::SDL_SCANCODE_SPACE,
  Stop = fermium::SDL_SCANCODE_STOP,
  SysReq = fermium::SDL_SCANCODE_SYSREQ,
  T = fermium::SDL_SCANCODE_T,
  Tab = fermium::SDL_SCANCODE_TAB,
  ThousandsSeparator = fermium::SDL_SCANCODE_THOUSANDSSEPARATOR,
  U = fermium::SDL_SCANCODE_U,
  Undo = fermium::SDL_SCANCODE_UNDO,
  Unknown = fermium::SDL_SCANCODE_UNKNOWN,
  Up = fermium::SDL_SCANCODE_UP,
  V = fermium::SDL_SCANCODE_V,
  VolumeDown = fermium::SDL_SCANCODE_VOLUMEDOWN,
  VolumeUp = fermium::SDL_SCANCODE_VOLUMEUP,
  W = fermium::SDL_SCANCODE_W,
  WWW = fermium::SDL_SCANCODE_WWW,
  X = fermium::SDL_SCANCODE_X,
  Y = fermium::SDL_SCANCODE_Y,
  Z = fermium::SDL_SCANCODE_Z,
}
impl TryFrom<fermium::SDL_Scancode> for Scancode {
  type Error = ();
  fn try_from(t: fermium::SDL_Scancode) -> Result<Self, Self::Error> {
    match t {
      SDL_SCANCODE_0 => Ok(Scancode::_0),
      SDL_SCANCODE_1 => Ok(Scancode::_1),
      SDL_SCANCODE_2 => Ok(Scancode::_2),
      SDL_SCANCODE_3 => Ok(Scancode::_3),
      SDL_SCANCODE_4 => Ok(Scancode::_4),
      SDL_SCANCODE_5 => Ok(Scancode::_5),
      SDL_SCANCODE_6 => Ok(Scancode::_6),
      SDL_SCANCODE_7 => Ok(Scancode::_7),
      SDL_SCANCODE_8 => Ok(Scancode::_8),
      SDL_SCANCODE_9 => Ok(Scancode::_9),
      SDL_SCANCODE_A => Ok(Scancode::A),
      SDL_SCANCODE_AC_BACK => Ok(Scancode::AC_Back),
      SDL_SCANCODE_AC_BOOKMARKS => Ok(Scancode::AC_Bookmarks),
      SDL_SCANCODE_AC_FORWARD => Ok(Scancode::AC_Forward),
      SDL_SCANCODE_AC_HOME => Ok(Scancode::AC_Home),
      SDL_SCANCODE_AC_REFRESH => Ok(Scancode::AC_Refresh),
      SDL_SCANCODE_AC_SEARCH => Ok(Scancode::AC_Search),
      SDL_SCANCODE_AC_STOP => Ok(Scancode::AC_Stop),
      SDL_SCANCODE_AGAIN => Ok(Scancode::Again),
      SDL_SCANCODE_ALTERASE => Ok(Scancode::AltErase),
      SDL_SCANCODE_APOSTROPHE => Ok(Scancode::Apostrophe),
      SDL_SCANCODE_APP1 => Ok(Scancode::App1),
      SDL_SCANCODE_APP2 => Ok(Scancode::App2),
      SDL_SCANCODE_APPLICATION => Ok(Scancode::Application),
      SDL_SCANCODE_AUDIOFASTFORWARD => Ok(Scancode::AudioFastForward),
      SDL_SCANCODE_AUDIOMUTE => Ok(Scancode::AudioMute),
      SDL_SCANCODE_AUDIONEXT => Ok(Scancode::AudioNext),
      SDL_SCANCODE_AUDIOPLAY => Ok(Scancode::AudioPlay),
      SDL_SCANCODE_AUDIOPREV => Ok(Scancode::AudioPrev),
      SDL_SCANCODE_AUDIOREWIND => Ok(Scancode::AudioRewind),
      SDL_SCANCODE_AUDIOSTOP => Ok(Scancode::AudioStop),
      SDL_SCANCODE_B => Ok(Scancode::B),
      SDL_SCANCODE_BACKSLASH => Ok(Scancode::Backslash),
      SDL_SCANCODE_BACKSPACE => Ok(Scancode::Backspace),
      SDL_SCANCODE_BRIGHTNESSDOWN => Ok(Scancode::BrightnessDown),
      SDL_SCANCODE_BRIGHTNESSUP => Ok(Scancode::BrightnessUp),
      SDL_SCANCODE_C => Ok(Scancode::C),
      SDL_SCANCODE_CALCULATOR => Ok(Scancode::Calculator),
      SDL_SCANCODE_CANCEL => Ok(Scancode::Cancel),
      SDL_SCANCODE_CAPSLOCK => Ok(Scancode::CapsLock),
      SDL_SCANCODE_CLEAR => Ok(Scancode::Clear),
      SDL_SCANCODE_CLEARAGAIN => Ok(Scancode::ClearAgain),
      SDL_SCANCODE_COMMA => Ok(Scancode::Comma),
      SDL_SCANCODE_COMPUTER => Ok(Scancode::Computer),
      SDL_SCANCODE_COPY => Ok(Scancode::Copy),
      SDL_SCANCODE_CRSEL => Ok(Scancode::CrSel),
      SDL_SCANCODE_CURRENCYSUBUNIT => Ok(Scancode::CurrencySubUnit),
      SDL_SCANCODE_CURRENCYUNIT => Ok(Scancode::CurrencyUnit),
      SDL_SCANCODE_CUT => Ok(Scancode::Cut),
      SDL_SCANCODE_D => Ok(Scancode::D),
      SDL_SCANCODE_DECIMALSEPARATOR => Ok(Scancode::DecimalSeparator),
      SDL_SCANCODE_DELETE => Ok(Scancode::Delete),
      SDL_SCANCODE_DISPLAYSWITCH => Ok(Scancode::DisplaySwitch),
      SDL_SCANCODE_DOWN => Ok(Scancode::Down),
      SDL_SCANCODE_E => Ok(Scancode::E),
      SDL_SCANCODE_EJECT => Ok(Scancode::Eject),
      SDL_SCANCODE_END => Ok(Scancode::End),
      SDL_SCANCODE_EQUALS => Ok(Scancode::Equals),
      SDL_SCANCODE_ESCAPE => Ok(Scancode::Escape),
      SDL_SCANCODE_EXECUTE => Ok(Scancode::Execute),
      SDL_SCANCODE_EXSEL => Ok(Scancode::ExSel),
      SDL_SCANCODE_F => Ok(Scancode::F),
      SDL_SCANCODE_F1 => Ok(Scancode::F1),
      SDL_SCANCODE_F2 => Ok(Scancode::F2),
      SDL_SCANCODE_F3 => Ok(Scancode::F3),
      SDL_SCANCODE_F4 => Ok(Scancode::F4),
      SDL_SCANCODE_F5 => Ok(Scancode::F5),
      SDL_SCANCODE_F6 => Ok(Scancode::F6),
      SDL_SCANCODE_F7 => Ok(Scancode::F7),
      SDL_SCANCODE_F8 => Ok(Scancode::F8),
      SDL_SCANCODE_F9 => Ok(Scancode::F9),
      SDL_SCANCODE_F10 => Ok(Scancode::F10),
      SDL_SCANCODE_F11 => Ok(Scancode::F11),
      SDL_SCANCODE_F12 => Ok(Scancode::F12),
      SDL_SCANCODE_F13 => Ok(Scancode::F13),
      SDL_SCANCODE_F14 => Ok(Scancode::F14),
      SDL_SCANCODE_F15 => Ok(Scancode::F15),
      SDL_SCANCODE_F16 => Ok(Scancode::F16),
      SDL_SCANCODE_F17 => Ok(Scancode::F17),
      SDL_SCANCODE_F18 => Ok(Scancode::F18),
      SDL_SCANCODE_F19 => Ok(Scancode::F19),
      SDL_SCANCODE_F20 => Ok(Scancode::F20),
      SDL_SCANCODE_F21 => Ok(Scancode::F21),
      SDL_SCANCODE_F22 => Ok(Scancode::F22),
      SDL_SCANCODE_F23 => Ok(Scancode::F23),
      SDL_SCANCODE_F24 => Ok(Scancode::F24),
      SDL_SCANCODE_FIND => Ok(Scancode::Find),
      SDL_SCANCODE_G => Ok(Scancode::G),
      SDL_SCANCODE_GRAVE => Ok(Scancode::Grave),
      SDL_SCANCODE_H => Ok(Scancode::H),
      SDL_SCANCODE_HELP => Ok(Scancode::Help),
      SDL_SCANCODE_HOME => Ok(Scancode::Home),
      SDL_SCANCODE_I => Ok(Scancode::I),
      SDL_SCANCODE_INSERT => Ok(Scancode::Insert),
      SDL_SCANCODE_INTERNATIONAL1 => Ok(Scancode::International1),
      SDL_SCANCODE_INTERNATIONAL2 => Ok(Scancode::International2),
      SDL_SCANCODE_INTERNATIONAL3 => Ok(Scancode::International3),
      SDL_SCANCODE_INTERNATIONAL4 => Ok(Scancode::International4),
      SDL_SCANCODE_INTERNATIONAL5 => Ok(Scancode::International5),
      SDL_SCANCODE_INTERNATIONAL6 => Ok(Scancode::International6),
      SDL_SCANCODE_INTERNATIONAL7 => Ok(Scancode::International7),
      SDL_SCANCODE_INTERNATIONAL8 => Ok(Scancode::International8),
      SDL_SCANCODE_INTERNATIONAL9 => Ok(Scancode::International9),
      SDL_SCANCODE_J => Ok(Scancode::J),
      SDL_SCANCODE_K => Ok(Scancode::K),
      SDL_SCANCODE_KBDILLUMDOWN => Ok(Scancode::KbdIlluminationDown),
      SDL_SCANCODE_KBDILLUMTOGGLE => Ok(Scancode::KbdIlluminationToggle),
      SDL_SCANCODE_KBDILLUMUP => Ok(Scancode::KbdIlluminationUp),
      SDL_SCANCODE_KP_0 => Ok(Scancode::KP_0),
      SDL_SCANCODE_KP_00 => Ok(Scancode::KP_00),
      SDL_SCANCODE_KP_000 => Ok(Scancode::KP_000),
      SDL_SCANCODE_KP_1 => Ok(Scancode::KP_1),
      SDL_SCANCODE_KP_2 => Ok(Scancode::KP_2),
      SDL_SCANCODE_KP_3 => Ok(Scancode::KP_3),
      SDL_SCANCODE_KP_4 => Ok(Scancode::KP_4),
      SDL_SCANCODE_KP_5 => Ok(Scancode::KP_5),
      SDL_SCANCODE_KP_6 => Ok(Scancode::KP_6),
      SDL_SCANCODE_KP_7 => Ok(Scancode::KP_7),
      SDL_SCANCODE_KP_8 => Ok(Scancode::KP_8),
      SDL_SCANCODE_KP_9 => Ok(Scancode::KP_9),
      SDL_SCANCODE_KP_A => Ok(Scancode::KP_A),
      SDL_SCANCODE_KP_AMPERSAND => Ok(Scancode::KP_Ampersand),
      SDL_SCANCODE_KP_AT => Ok(Scancode::KP_At),
      SDL_SCANCODE_KP_B => Ok(Scancode::KP_B),
      SDL_SCANCODE_KP_BACKSPACE => Ok(Scancode::KP_Backspace),
      SDL_SCANCODE_KP_BINARY => Ok(Scancode::KP_Binary),
      SDL_SCANCODE_KP_C => Ok(Scancode::KP_C),
      SDL_SCANCODE_KP_CLEAR => Ok(Scancode::KP_Clear),
      SDL_SCANCODE_KP_CLEARENTRY => Ok(Scancode::KP_ClearEntry),
      SDL_SCANCODE_KP_COLON => Ok(Scancode::KP_Colon),
      SDL_SCANCODE_KP_COMMA => Ok(Scancode::KP_Comma),
      SDL_SCANCODE_KP_D => Ok(Scancode::KP_D),
      SDL_SCANCODE_KP_DBLAMPERSAND => Ok(Scancode::KP_DblAmpersand),
      SDL_SCANCODE_KP_DBLVERTICALBAR => Ok(Scancode::KP_DblVerticalBar),
      SDL_SCANCODE_KP_DECIMAL => Ok(Scancode::KP_Decimal),
      SDL_SCANCODE_KP_DIVIDE => Ok(Scancode::KP_Divide),
      SDL_SCANCODE_KP_E => Ok(Scancode::KP_E),
      SDL_SCANCODE_KP_ENTER => Ok(Scancode::KP_Enter),
      SDL_SCANCODE_KP_EQUALS => Ok(Scancode::KP_Equals),
      SDL_SCANCODE_KP_EQUALSAS400 => Ok(Scancode::KP_EqualsAs400),
      SDL_SCANCODE_KP_EXCLAM => Ok(Scancode::KP_Exclamation),
      SDL_SCANCODE_KP_F => Ok(Scancode::KP_F),
      SDL_SCANCODE_KP_GREATER => Ok(Scancode::KP_Greater),
      SDL_SCANCODE_KP_HASH => Ok(Scancode::KP_Hash),
      SDL_SCANCODE_KP_HEXADECIMAL => Ok(Scancode::KP_Hexadecimal),
      SDL_SCANCODE_KP_LEFTBRACE => Ok(Scancode::KP_LeftBrace),
      SDL_SCANCODE_KP_LEFTPAREN => Ok(Scancode::KP_LeftParen),
      SDL_SCANCODE_KP_LESS => Ok(Scancode::KP_Less),
      SDL_SCANCODE_KP_MEMADD => Ok(Scancode::KP_MemAdd),
      SDL_SCANCODE_KP_MEMCLEAR => Ok(Scancode::KP_MemClear),
      SDL_SCANCODE_KP_MEMDIVIDE => Ok(Scancode::KP_MemDivide),
      SDL_SCANCODE_KP_MEMMULTIPLY => Ok(Scancode::KP_MemMultiply),
      SDL_SCANCODE_KP_MEMRECALL => Ok(Scancode::KP_MemRecall),
      SDL_SCANCODE_KP_MEMSTORE => Ok(Scancode::KP_MemStore),
      SDL_SCANCODE_KP_MEMSUBTRACT => Ok(Scancode::KP_MemSubtract),
      SDL_SCANCODE_KP_MINUS => Ok(Scancode::KP_Minus),
      SDL_SCANCODE_KP_MULTIPLY => Ok(Scancode::KP_Multiply),
      SDL_SCANCODE_KP_OCTAL => Ok(Scancode::KP_Octal),
      SDL_SCANCODE_KP_PERCENT => Ok(Scancode::KP_Percent),
      SDL_SCANCODE_KP_PERIOD => Ok(Scancode::KP_Period),
      SDL_SCANCODE_KP_PLUS => Ok(Scancode::KP_Plus),
      SDL_SCANCODE_KP_PLUSMINUS => Ok(Scancode::KP_PlusMinus),
      SDL_SCANCODE_KP_POWER => Ok(Scancode::KP_Power),
      SDL_SCANCODE_KP_RIGHTBRACE => Ok(Scancode::KP_RightBrace),
      SDL_SCANCODE_KP_RIGHTPAREN => Ok(Scancode::KP_RightParen),
      SDL_SCANCODE_KP_SPACE => Ok(Scancode::KP_Space),
      SDL_SCANCODE_KP_TAB => Ok(Scancode::KP_Tab),
      SDL_SCANCODE_KP_VERTICALBAR => Ok(Scancode::KP_VerticalBar),
      SDL_SCANCODE_KP_XOR => Ok(Scancode::KP_Xor),
      SDL_SCANCODE_L => Ok(Scancode::L),
      SDL_SCANCODE_LALT => Ok(Scancode::LeftAlt),
      SDL_SCANCODE_LANG1 => Ok(Scancode::Lang1),
      SDL_SCANCODE_LANG2 => Ok(Scancode::Lang2),
      SDL_SCANCODE_LANG3 => Ok(Scancode::Lang3),
      SDL_SCANCODE_LANG4 => Ok(Scancode::Lang4),
      SDL_SCANCODE_LANG5 => Ok(Scancode::Lang5),
      SDL_SCANCODE_LANG6 => Ok(Scancode::Lang6),
      SDL_SCANCODE_LANG7 => Ok(Scancode::Lang7),
      SDL_SCANCODE_LANG8 => Ok(Scancode::Lang8),
      SDL_SCANCODE_LANG9 => Ok(Scancode::Lang9),
      SDL_SCANCODE_LCTRL => Ok(Scancode::LeftCtrl),
      SDL_SCANCODE_LEFT => Ok(Scancode::Left),
      SDL_SCANCODE_LEFTBRACKET => Ok(Scancode::LeftBracket),
      SDL_SCANCODE_LGUI => Ok(Scancode::LeftGUI),
      SDL_SCANCODE_LSHIFT => Ok(Scancode::LeftShift),
      SDL_SCANCODE_M => Ok(Scancode::M),
      SDL_SCANCODE_MAIL => Ok(Scancode::Mail),
      SDL_SCANCODE_MEDIASELECT => Ok(Scancode::Mediaselect),
      SDL_SCANCODE_MENU => Ok(Scancode::Menu),
      SDL_SCANCODE_MINUS => Ok(Scancode::Minus),
      SDL_SCANCODE_MODE => Ok(Scancode::Mode),
      SDL_SCANCODE_MUTE => Ok(Scancode::Mute),
      SDL_SCANCODE_N => Ok(Scancode::N),
      SDL_SCANCODE_NONUSBACKSLASH => Ok(Scancode::NonUSBackslash),
      SDL_SCANCODE_NONUSHASH => Ok(Scancode::NonUSHash),
      SDL_SCANCODE_NUMLOCKCLEAR => Ok(Scancode::NumLockClear),
      SDL_SCANCODE_O => Ok(Scancode::O),
      SDL_SCANCODE_OPER => Ok(Scancode::Oper),
      SDL_SCANCODE_OUT => Ok(Scancode::Out),
      SDL_SCANCODE_P => Ok(Scancode::P),
      SDL_SCANCODE_PAGEDOWN => Ok(Scancode::PageDown),
      SDL_SCANCODE_PAGEUP => Ok(Scancode::PageUp),
      SDL_SCANCODE_PASTE => Ok(Scancode::Paste),
      SDL_SCANCODE_PAUSE => Ok(Scancode::Pause),
      SDL_SCANCODE_PERIOD => Ok(Scancode::Period),
      SDL_SCANCODE_POWER => Ok(Scancode::Power),
      SDL_SCANCODE_PRINTSCREEN => Ok(Scancode::PrintScreen),
      SDL_SCANCODE_PRIOR => Ok(Scancode::Prior),
      SDL_SCANCODE_Q => Ok(Scancode::Q),
      SDL_SCANCODE_R => Ok(Scancode::R),
      SDL_SCANCODE_RALT => Ok(Scancode::RightAlt),
      SDL_SCANCODE_RCTRL => Ok(Scancode::RightCtrl),
      SDL_SCANCODE_RETURN => Ok(Scancode::Return),
      SDL_SCANCODE_RETURN2 => Ok(Scancode::Return2),
      SDL_SCANCODE_RGUI => Ok(Scancode::RightGUI),
      SDL_SCANCODE_RIGHT => Ok(Scancode::Right),
      SDL_SCANCODE_RIGHTBRACKET => Ok(Scancode::RightBracket),
      SDL_SCANCODE_RSHIFT => Ok(Scancode::RightShift),
      SDL_SCANCODE_S => Ok(Scancode::S),
      SDL_SCANCODE_SCROLLLOCK => Ok(Scancode::ScrollLock),
      SDL_SCANCODE_SELECT => Ok(Scancode::Select),
      SDL_SCANCODE_SEMICOLON => Ok(Scancode::Semicolon),
      SDL_SCANCODE_SEPARATOR => Ok(Scancode::Separator),
      SDL_SCANCODE_SLASH => Ok(Scancode::Slash),
      SDL_SCANCODE_SLEEP => Ok(Scancode::Sleep),
      SDL_SCANCODE_SPACE => Ok(Scancode::Space),
      SDL_SCANCODE_STOP => Ok(Scancode::Stop),
      SDL_SCANCODE_SYSREQ => Ok(Scancode::SysReq),
      SDL_SCANCODE_T => Ok(Scancode::T),
      SDL_SCANCODE_TAB => Ok(Scancode::Tab),
      SDL_SCANCODE_THOUSANDSSEPARATOR => Ok(Scancode::ThousandsSeparator),
      SDL_SCANCODE_U => Ok(Scancode::U),
      SDL_SCANCODE_UNDO => Ok(Scancode::Undo),
      SDL_SCANCODE_UNKNOWN => Ok(Scancode::Unknown),
      SDL_SCANCODE_UP => Ok(Scancode::Up),
      SDL_SCANCODE_V => Ok(Scancode::V),
      SDL_SCANCODE_VOLUMEDOWN => Ok(Scancode::VolumeDown),
      SDL_SCANCODE_VOLUMEUP => Ok(Scancode::VolumeUp),
      SDL_SCANCODE_W => Ok(Scancode::W),
      SDL_SCANCODE_WWW => Ok(Scancode::WWW),
      SDL_SCANCODE_X => Ok(Scancode::X),
      SDL_SCANCODE_Y => Ok(Scancode::Y),
      SDL_SCANCODE_Z => Ok(Scancode::Z),
      _ => Err(()),
    }
  }
}

// Note(Lokathor): Rust is stupidly picky so we'll make casted consts here.
const KEY_MODIFIER_CAPS: u16 = KMOD_CAPS as u16;
const KEY_MODIFIER_LALT: u16 = KMOD_LALT as u16;
const KEY_MODIFIER_LCTRL: u16 = KMOD_LCTRL as u16;
const KEY_MODIFIER_LGUI: u16 = KMOD_LGUI as u16;
const KEY_MODIFIER_LSHIFT: u16 = KMOD_LSHIFT as u16;
const KEY_MODIFIER_MODE: u16 = KMOD_MODE as u16;
const KEY_MODIFIER_NUM: u16 = KMOD_NUM as u16;
const KEY_MODIFIER_RALT: u16 = KMOD_RALT as u16;
const KEY_MODIFIER_RCTRL: u16 = KMOD_RCTRL as u16;
const KEY_MODIFIER_RGUI: u16 = KMOD_RGUI as u16;
const KEY_MODIFIER_RSHIFT: u16 = KMOD_RSHIFT as u16;

/// Holds flags for the keyboard modifier keys being held.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KeyModifiers(u16);
impl KeyModifiers {
  phantom_fields! {
    self.0: u16,
    caps_lock: KEY_MODIFIER_CAPS,
    left_alt: KEY_MODIFIER_LALT,
    left_ctrl: KEY_MODIFIER_LCTRL,
    left_gui: KEY_MODIFIER_LGUI,
    left_shift: KEY_MODIFIER_LSHIFT,
    /// The `AltGr` key
    mode: KEY_MODIFIER_MODE,
    num_lock: KEY_MODIFIER_NUM,
    right_alt: KEY_MODIFIER_RALT,
    right_ctrl: KEY_MODIFIER_RCTRL,
    right_gui: KEY_MODIFIER_RGUI,
    right_shift: KEY_MODIFIER_RSHIFT,
  }
}
