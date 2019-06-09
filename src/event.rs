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
    key: KeyInfo,
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
  /// The size of the window has been changed externally.
  ///
  /// This event is always preceeded by a [`Event::WindowSizeChanged`], however
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
  /// The window has been hidden.
  ///
  /// Inverse of [`Event::WindowShown`].
  WindowHidden {
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
  /// The window has been minimized.
  WindowMinimized {
    /// When the event happened
    timestamp: u32,
    /// The window which has been minimized.
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
        SDL_KEYDOWN | SDL_KEYUP => Event::Keyboard {
          timestamp: event.key.timestamp,
          window_id: event.key.windowID,
          is_key_down: u32::from(event.key.state) == SDL_PRESSED,
          repeat_count: event.key.repeat,
          key: KeyInfo::from(event.key.keysym),
        },
        SDL_WINDOWEVENT => match SDL_WindowEventID::Type::from(event.window.event) {
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
          _ => Event::UnknownEventType,
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
impl From<SDL_Keysym> for KeyInfo {
  fn from(keysym: SDL_Keysym) -> Self {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
pub enum Keycode {
  _0 = SDLK_0,
  _1 = SDLK_1,
  _2 = SDLK_2,
  _3 = SDLK_3,
  _4 = SDLK_4,
  _5 = SDLK_5,
  _6 = SDLK_6,
  _7 = SDLK_7,
  _8 = SDLK_8,
  _9 = SDLK_9,
  AC_Back = SDLK_AC_BACK,
  AC_Bookmarks = SDLK_AC_BOOKMARKS,
  AC_Forward = SDLK_AC_FORWARD,
  AC_Home = SDLK_AC_HOME,
  AC_Refresh = SDLK_AC_REFRESH,
  AC_Search = SDLK_AC_SEARCH,
  AC_Stop = SDLK_AC_STOP,
  Again = SDLK_AGAIN,
  AltErase = SDLK_ALTERASE,
  Ampersand = SDLK_AMPERSAND,
  App1 = SDLK_APP1,
  App2 = SDLK_APP2,
  Application = SDLK_APPLICATION,
  Asterisk = SDLK_ASTERISK,
  At = SDLK_AT,
  AudioFastForward = SDLK_AUDIOFASTFORWARD,
  AudioMute = SDLK_AUDIOMUTE,
  AudioNext = SDLK_AUDIONEXT,
  AudioPlay = SDLK_AUDIOPLAY,
  AudioPrev = SDLK_AUDIOPREV,
  AudioRewind = SDLK_AUDIOREWIND,
  AudioStop = SDLK_AUDIOSTOP,
  Backquote = SDLK_BACKQUOTE,
  Backslash = SDLK_BACKSLASH,
  Backspace = SDLK_BACKSPACE,
  BrightnessDown = SDLK_BRIGHTNESSDOWN,
  BrightnessUp = SDLK_BRIGHTNESSUP,
  Calculator = SDLK_CALCULATOR,
  Cancel = SDLK_CANCEL,
  CapsLock = SDLK_CAPSLOCK,
  Caret = SDLK_CARET,
  Clear = SDLK_CLEAR,
  ClearAgain = SDLK_CLEARAGAIN,
  Colon = SDLK_COLON,
  Comma = SDLK_COMMA,
  Computer = SDLK_COMPUTER,
  Copy = SDLK_COPY,
  CrSel = SDLK_CRSEL,
  CurrencySubUnit = SDLK_CURRENCYSUBUNIT,
  CurrencyUnit = SDLK_CURRENCYUNIT,
  Cut = SDLK_CUT,
  DecimalSeparator = SDLK_DECIMALSEPARATOR,
  Delete = SDLK_DELETE,
  DisplaySwitch = SDLK_DISPLAYSWITCH,
  Dollar = SDLK_DOLLAR,
  Down = SDLK_DOWN,
  Eject = SDLK_EJECT,
  End = SDLK_END,
  Equals = SDLK_EQUALS,
  Escape = SDLK_ESCAPE,
  Exclamation = SDLK_EXCLAIM,
  Execute = SDLK_EXECUTE,
  ExSel = SDLK_EXSEL,
  F1 = SDLK_F1,
  F2 = SDLK_F2,
  F3 = SDLK_F3,
  F4 = SDLK_F4,
  F5 = SDLK_F5,
  F6 = SDLK_F6,
  F7 = SDLK_F7,
  F8 = SDLK_F8,
  F9 = SDLK_F9,
  F10 = SDLK_F10,
  F11 = SDLK_F11,
  F12 = SDLK_F12,
  F13 = SDLK_F13,
  F14 = SDLK_F14,
  F15 = SDLK_F15,
  F16 = SDLK_F16,
  F17 = SDLK_F17,
  F18 = SDLK_F18,
  F19 = SDLK_F19,
  F20 = SDLK_F20,
  F21 = SDLK_F21,
  F22 = SDLK_F22,
  F23 = SDLK_F23,
  F24 = SDLK_F24,
  Find = SDLK_FIND,
  Greater = SDLK_GREATER,
  Hash = SDLK_HASH,
  Help = SDLK_HELP,
  Home = SDLK_HOME,
  Insert = SDLK_INSERT,
  KbdIlluminationDown = SDLK_KBDILLUMDOWN,
  KbdIlluminationToggle = SDLK_KBDILLUMTOGGLE,
  KbdIlluminationUp = SDLK_KBDILLUMUP,
  KP_0 = SDLK_KP_0,
  KP_00 = SDLK_KP_00,
  KP_000 = SDLK_KP_000,
  KP_1 = SDLK_KP_1,
  KP_2 = SDLK_KP_2,
  KP_3 = SDLK_KP_3,
  KP_4 = SDLK_KP_4,
  KP_5 = SDLK_KP_5,
  KP_6 = SDLK_KP_6,
  KP_7 = SDLK_KP_7,
  KP_8 = SDLK_KP_8,
  KP_9 = SDLK_KP_9,
  KP_A = SDLK_KP_A,
  KP_Ampersand = SDLK_KP_AMPERSAND,
  KP_At = SDLK_KP_AT,
  KP_B = SDLK_KP_B,
  KP_Backspace = SDLK_KP_BACKSPACE,
  KP_Binary = SDLK_KP_BINARY,
  KP_C = SDLK_KP_C,
  KP_Clear = SDLK_KP_CLEAR,
  KP_ClearEntry = SDLK_KP_CLEARENTRY,
  KP_Colon = SDLK_KP_COLON,
  KP_Comma = SDLK_KP_COMMA,
  KP_D = SDLK_KP_D,
  KP_DblAmpersand = SDLK_KP_DBLAMPERSAND,
  KP_DblVerticalBar = SDLK_KP_DBLVERTICALBAR,
  KP_Decimal = SDLK_KP_DECIMAL,
  KP_Divide = SDLK_KP_DIVIDE,
  KP_E = SDLK_KP_E,
  KP_Enter = SDLK_KP_ENTER,
  KP_Equals = SDLK_KP_EQUALS,
  KP_EqualsAs400 = SDLK_KP_EQUALSAS400,
  KP_Exclamation = SDLK_KP_EXCLAM,
  KP_F = SDLK_KP_F,
  KP_Greater = SDLK_KP_GREATER,
  KP_Hash = SDLK_KP_HASH,
  KP_Hexadecimal = SDLK_KP_HEXADECIMAL,
  KP_LeftBrace = SDLK_KP_LEFTBRACE,
  KP_LeftParen = SDLK_KP_LEFTPAREN,
  KP_Less = SDLK_KP_LESS,
  KP_MemAdd = SDLK_KP_MEMADD,
  KP_MemClear = SDLK_KP_MEMCLEAR,
  KP_MemDivide = SDLK_KP_MEMDIVIDE,
  KP_MemMultiply = SDLK_KP_MEMMULTIPLY,
  KP_MemRecall = SDLK_KP_MEMRECALL,
  KP_MemStore = SDLK_KP_MEMSTORE,
  KP_MemSubtract = SDLK_KP_MEMSUBTRACT,
  KP_Minus = SDLK_KP_MINUS,
  KP_Multiply = SDLK_KP_MULTIPLY,
  KP_Octal = SDLK_KP_OCTAL,
  KP_Percent = SDLK_KP_PERCENT,
  KP_Period = SDLK_KP_PERIOD,
  KP_Plus = SDLK_KP_PLUS,
  KP_PlusMinus = SDLK_KP_PLUSMINUS,
  KP_Power = SDLK_KP_POWER,
  KP_RightBrace = SDLK_KP_RIGHTBRACE,
  KP_RightParen = SDLK_KP_RIGHTPAREN,
  KP_Space = SDLK_KP_SPACE,
  KP_Tab = SDLK_KP_TAB,
  KP_VerticalBar = SDLK_KP_VERTICALBAR,
  KP_Xor = SDLK_KP_XOR,
  LeftAlt = SDLK_LALT,
  LeftCtrl = SDLK_LCTRL,
  Left = SDLK_LEFT,
  LeftBracket = SDLK_LEFTBRACKET,
  LeftParen = SDLK_LEFTPAREN,
  Less = SDLK_LESS,
  LeftGUI = SDLK_LGUI,
  LeftShift = SDLK_LSHIFT,
  Mail = SDLK_MAIL,
  MediaSelect = SDLK_MEDIASELECT,
  Menu = SDLK_MENU,
  Minus = SDLK_MINUS,
  Mode = SDLK_MODE,
  Mute = SDLK_MUTE,
  NumLockClear = SDLK_NUMLOCKCLEAR,
  Oper = SDLK_OPER,
  Out = SDLK_OUT,
  PageDown = SDLK_PAGEDOWN,
  PageUp = SDLK_PAGEUP,
  Paste = SDLK_PASTE,
  Pause = SDLK_PAUSE,
  Percent = SDLK_PERCENT,
  Period = SDLK_PERIOD,
  Plus = SDLK_PLUS,
  Power = SDLK_POWER,
  PrintScreen = SDLK_PRINTSCREEN,
  Prior = SDLK_PRIOR,
  Question = SDLK_QUESTION,
  Quote = SDLK_QUOTE,
  DblQuote = SDLK_QUOTEDBL,
  RightAlt = SDLK_RALT,
  RightCtrl = SDLK_RCTRL,
  Return = SDLK_RETURN,
  Return2 = SDLK_RETURN2,
  RightGUI = SDLK_RGUI,
  Right = SDLK_RIGHT,
  RightBracket = SDLK_RIGHTBRACKET,
  RightParen = SDLK_RIGHTPAREN,
  RightShift = SDLK_RSHIFT,
  ScrollLock = SDLK_SCROLLLOCK,
  Select = SDLK_SELECT,
  Semicolon = SDLK_SEMICOLON,
  Separator = SDLK_SEPARATOR,
  Slash = SDLK_SLASH,
  Sleep = SDLK_SLEEP,
  Space = SDLK_SPACE,
  Stop = SDLK_STOP,
  SysReq = SDLK_SYSREQ,
  Tab = SDLK_TAB,
  ThousandsSeparator = SDLK_THOUSANDSSEPARATOR,
  Underscore = SDLK_UNDERSCORE,
  Undo = SDLK_UNDO,
  Unknown = SDLK_UNKNOWN,
  Up = SDLK_UP,
  VolumeDown = SDLK_VOLUMEDOWN,
  VolumeUp = SDLK_VOLUMEUP,
  WWW = SDLK_WWW,
  A = SDLK_a,
  B = SDLK_b,
  C = SDLK_c,
  D = SDLK_d,
  E = SDLK_e,
  F = SDLK_f,
  G = SDLK_g,
  H = SDLK_h,
  I = SDLK_i,
  J = SDLK_j,
  K = SDLK_k,
  L = SDLK_l,
  M = SDLK_m,
  N = SDLK_n,
  O = SDLK_o,
  P = SDLK_p,
  Q = SDLK_q,
  R = SDLK_r,
  S = SDLK_s,
  T = SDLK_t,
  U = SDLK_u,
  V = SDLK_v,
  W = SDLK_w,
  X = SDLK_x,
  Y = SDLK_y,
  Z = SDLK_z,
}
impl TryFrom<i32> for Keycode {
  type Error = ();
  fn try_from(t: i32) -> Result<Self, Self::Error> {
    #[allow(non_upper_case_globals)]
    match t as fermium::_bindgen_ty_7::Type {
      SDLK_0 => Ok(Keycode::_0),
      SDLK_1 => Ok(Keycode::_1),
      SDLK_2 => Ok(Keycode::_2),
      SDLK_3 => Ok(Keycode::_3),
      SDLK_4 => Ok(Keycode::_4),
      SDLK_5 => Ok(Keycode::_5),
      SDLK_6 => Ok(Keycode::_6),
      SDLK_7 => Ok(Keycode::_7),
      SDLK_8 => Ok(Keycode::_8),
      SDLK_9 => Ok(Keycode::_9),
      SDLK_AC_BACK => Ok(Keycode::AC_Back),
      SDLK_AC_BOOKMARKS => Ok(Keycode::AC_Bookmarks),
      SDLK_AC_FORWARD => Ok(Keycode::AC_Forward),
      SDLK_AC_HOME => Ok(Keycode::AC_Home),
      SDLK_AC_REFRESH => Ok(Keycode::AC_Refresh),
      SDLK_AC_SEARCH => Ok(Keycode::AC_Search),
      SDLK_AC_STOP => Ok(Keycode::AC_Stop),
      SDLK_AGAIN => Ok(Keycode::Again),
      SDLK_ALTERASE => Ok(Keycode::AltErase),
      SDLK_AMPERSAND => Ok(Keycode::Ampersand),
      SDLK_APP1 => Ok(Keycode::App1),
      SDLK_APP2 => Ok(Keycode::App2),
      SDLK_APPLICATION => Ok(Keycode::Application),
      SDLK_ASTERISK => Ok(Keycode::Asterisk),
      SDLK_AT => Ok(Keycode::At),
      SDLK_AUDIOFASTFORWARD => Ok(Keycode::AudioFastForward),
      SDLK_AUDIOMUTE => Ok(Keycode::AudioMute),
      SDLK_AUDIONEXT => Ok(Keycode::AudioNext),
      SDLK_AUDIOPLAY => Ok(Keycode::AudioPlay),
      SDLK_AUDIOPREV => Ok(Keycode::AudioPrev),
      SDLK_AUDIOREWIND => Ok(Keycode::AudioRewind),
      SDLK_AUDIOSTOP => Ok(Keycode::AudioStop),
      SDLK_BACKQUOTE => Ok(Keycode::Backquote),
      SDLK_BACKSLASH => Ok(Keycode::Backslash),
      SDLK_BACKSPACE => Ok(Keycode::Backspace),
      SDLK_BRIGHTNESSDOWN => Ok(Keycode::BrightnessDown),
      SDLK_BRIGHTNESSUP => Ok(Keycode::BrightnessUp),
      SDLK_CALCULATOR => Ok(Keycode::Calculator),
      SDLK_CANCEL => Ok(Keycode::Cancel),
      SDLK_CAPSLOCK => Ok(Keycode::CapsLock),
      SDLK_CARET => Ok(Keycode::Caret),
      SDLK_CLEAR => Ok(Keycode::Clear),
      SDLK_CLEARAGAIN => Ok(Keycode::ClearAgain),
      SDLK_COLON => Ok(Keycode::Colon),
      SDLK_COMMA => Ok(Keycode::Comma),
      SDLK_COMPUTER => Ok(Keycode::Computer),
      SDLK_COPY => Ok(Keycode::Copy),
      SDLK_CRSEL => Ok(Keycode::CrSel),
      SDLK_CURRENCYSUBUNIT => Ok(Keycode::CurrencySubUnit),
      SDLK_CURRENCYUNIT => Ok(Keycode::CurrencyUnit),
      SDLK_CUT => Ok(Keycode::Cut),
      SDLK_DECIMALSEPARATOR => Ok(Keycode::DecimalSeparator),
      SDLK_DELETE => Ok(Keycode::Delete),
      SDLK_DISPLAYSWITCH => Ok(Keycode::DisplaySwitch),
      SDLK_DOLLAR => Ok(Keycode::Dollar),
      SDLK_DOWN => Ok(Keycode::Down),
      SDLK_EJECT => Ok(Keycode::Eject),
      SDLK_END => Ok(Keycode::End),
      SDLK_EQUALS => Ok(Keycode::Equals),
      SDLK_ESCAPE => Ok(Keycode::Escape),
      SDLK_EXCLAIM => Ok(Keycode::Exclamation),
      SDLK_EXECUTE => Ok(Keycode::Execute),
      SDLK_EXSEL => Ok(Keycode::ExSel),
      SDLK_F1 => Ok(Keycode::F1),
      SDLK_F2 => Ok(Keycode::F2),
      SDLK_F3 => Ok(Keycode::F3),
      SDLK_F4 => Ok(Keycode::F4),
      SDLK_F5 => Ok(Keycode::F5),
      SDLK_F6 => Ok(Keycode::F6),
      SDLK_F7 => Ok(Keycode::F7),
      SDLK_F8 => Ok(Keycode::F8),
      SDLK_F9 => Ok(Keycode::F9),
      SDLK_F10 => Ok(Keycode::F10),
      SDLK_F11 => Ok(Keycode::F11),
      SDLK_F12 => Ok(Keycode::F12),
      SDLK_F13 => Ok(Keycode::F13),
      SDLK_F14 => Ok(Keycode::F14),
      SDLK_F15 => Ok(Keycode::F15),
      SDLK_F16 => Ok(Keycode::F16),
      SDLK_F17 => Ok(Keycode::F17),
      SDLK_F18 => Ok(Keycode::F18),
      SDLK_F19 => Ok(Keycode::F19),
      SDLK_F20 => Ok(Keycode::F20),
      SDLK_F21 => Ok(Keycode::F21),
      SDLK_F22 => Ok(Keycode::F22),
      SDLK_F23 => Ok(Keycode::F23),
      SDLK_F24 => Ok(Keycode::F24),
      SDLK_FIND => Ok(Keycode::Find),
      SDLK_GREATER => Ok(Keycode::Greater),
      SDLK_HASH => Ok(Keycode::Hash),
      SDLK_HELP => Ok(Keycode::Help),
      SDLK_HOME => Ok(Keycode::Home),
      SDLK_INSERT => Ok(Keycode::Insert),
      SDLK_KBDILLUMDOWN => Ok(Keycode::KbdIlluminationDown),
      SDLK_KBDILLUMTOGGLE => Ok(Keycode::KbdIlluminationToggle),
      SDLK_KBDILLUMUP => Ok(Keycode::KbdIlluminationUp),
      SDLK_KP_0 => Ok(Keycode::KP_0),
      SDLK_KP_00 => Ok(Keycode::KP_00),
      SDLK_KP_000 => Ok(Keycode::KP_000),
      SDLK_KP_1 => Ok(Keycode::KP_1),
      SDLK_KP_2 => Ok(Keycode::KP_2),
      SDLK_KP_3 => Ok(Keycode::KP_3),
      SDLK_KP_4 => Ok(Keycode::KP_4),
      SDLK_KP_5 => Ok(Keycode::KP_5),
      SDLK_KP_6 => Ok(Keycode::KP_6),
      SDLK_KP_7 => Ok(Keycode::KP_7),
      SDLK_KP_8 => Ok(Keycode::KP_8),
      SDLK_KP_9 => Ok(Keycode::KP_9),
      SDLK_KP_A => Ok(Keycode::KP_A),
      SDLK_KP_AMPERSAND => Ok(Keycode::KP_Ampersand),
      SDLK_KP_AT => Ok(Keycode::KP_At),
      SDLK_KP_B => Ok(Keycode::KP_B),
      SDLK_KP_BACKSPACE => Ok(Keycode::KP_Backspace),
      SDLK_KP_BINARY => Ok(Keycode::KP_Binary),
      SDLK_KP_C => Ok(Keycode::KP_C),
      SDLK_KP_CLEAR => Ok(Keycode::KP_Clear),
      SDLK_KP_CLEARENTRY => Ok(Keycode::KP_ClearEntry),
      SDLK_KP_COLON => Ok(Keycode::KP_Colon),
      SDLK_KP_COMMA => Ok(Keycode::KP_Comma),
      SDLK_KP_D => Ok(Keycode::KP_D),
      SDLK_KP_DBLAMPERSAND => Ok(Keycode::KP_DblAmpersand),
      SDLK_KP_DBLVERTICALBAR => Ok(Keycode::KP_DblVerticalBar),
      SDLK_KP_DECIMAL => Ok(Keycode::KP_Decimal),
      SDLK_KP_DIVIDE => Ok(Keycode::KP_Divide),
      SDLK_KP_E => Ok(Keycode::KP_E),
      SDLK_KP_ENTER => Ok(Keycode::KP_Enter),
      SDLK_KP_EQUALS => Ok(Keycode::KP_Equals),
      SDLK_KP_EQUALSAS400 => Ok(Keycode::KP_EqualsAs400),
      SDLK_KP_EXCLAM => Ok(Keycode::KP_Exclamation),
      SDLK_KP_F => Ok(Keycode::KP_F),
      SDLK_KP_GREATER => Ok(Keycode::KP_Greater),
      SDLK_KP_HASH => Ok(Keycode::KP_Hash),
      SDLK_KP_HEXADECIMAL => Ok(Keycode::KP_Hexadecimal),
      SDLK_KP_LEFTBRACE => Ok(Keycode::KP_LeftBrace),
      SDLK_KP_LEFTPAREN => Ok(Keycode::KP_LeftParen),
      SDLK_KP_LESS => Ok(Keycode::KP_Less),
      SDLK_KP_MEMADD => Ok(Keycode::KP_MemAdd),
      SDLK_KP_MEMCLEAR => Ok(Keycode::KP_MemClear),
      SDLK_KP_MEMDIVIDE => Ok(Keycode::KP_MemDivide),
      SDLK_KP_MEMMULTIPLY => Ok(Keycode::KP_MemMultiply),
      SDLK_KP_MEMRECALL => Ok(Keycode::KP_MemRecall),
      SDLK_KP_MEMSTORE => Ok(Keycode::KP_MemStore),
      SDLK_KP_MEMSUBTRACT => Ok(Keycode::KP_MemSubtract),
      SDLK_KP_MINUS => Ok(Keycode::KP_Minus),
      SDLK_KP_MULTIPLY => Ok(Keycode::KP_Multiply),
      SDLK_KP_OCTAL => Ok(Keycode::KP_Octal),
      SDLK_KP_PERCENT => Ok(Keycode::KP_Percent),
      SDLK_KP_PERIOD => Ok(Keycode::KP_Period),
      SDLK_KP_PLUS => Ok(Keycode::KP_Plus),
      SDLK_KP_PLUSMINUS => Ok(Keycode::KP_PlusMinus),
      SDLK_KP_POWER => Ok(Keycode::KP_Power),
      SDLK_KP_RIGHTBRACE => Ok(Keycode::KP_RightBrace),
      SDLK_KP_RIGHTPAREN => Ok(Keycode::KP_RightParen),
      SDLK_KP_SPACE => Ok(Keycode::KP_Space),
      SDLK_KP_TAB => Ok(Keycode::KP_Tab),
      SDLK_KP_VERTICALBAR => Ok(Keycode::KP_VerticalBar),
      SDLK_KP_XOR => Ok(Keycode::KP_Xor),
      SDLK_LALT => Ok(Keycode::LeftAlt),
      SDLK_LCTRL => Ok(Keycode::LeftCtrl),
      SDLK_LEFT => Ok(Keycode::Left),
      SDLK_LEFTBRACKET => Ok(Keycode::LeftBracket),
      SDLK_LEFTPAREN => Ok(Keycode::LeftParen),
      SDLK_LESS => Ok(Keycode::Less),
      SDLK_LGUI => Ok(Keycode::LeftGUI),
      SDLK_LSHIFT => Ok(Keycode::LeftShift),
      SDLK_MAIL => Ok(Keycode::Mail),
      SDLK_MEDIASELECT => Ok(Keycode::MediaSelect),
      SDLK_MENU => Ok(Keycode::Menu),
      SDLK_MINUS => Ok(Keycode::Minus),
      SDLK_MODE => Ok(Keycode::Mode),
      SDLK_MUTE => Ok(Keycode::Mute),
      SDLK_NUMLOCKCLEAR => Ok(Keycode::NumLockClear),
      SDLK_OPER => Ok(Keycode::Oper),
      SDLK_OUT => Ok(Keycode::Out),
      SDLK_PAGEDOWN => Ok(Keycode::PageDown),
      SDLK_PAGEUP => Ok(Keycode::PageUp),
      SDLK_PASTE => Ok(Keycode::Paste),
      SDLK_PAUSE => Ok(Keycode::Pause),
      SDLK_PERCENT => Ok(Keycode::Percent),
      SDLK_PERIOD => Ok(Keycode::Period),
      SDLK_PLUS => Ok(Keycode::Plus),
      SDLK_POWER => Ok(Keycode::Power),
      SDLK_PRINTSCREEN => Ok(Keycode::PrintScreen),
      SDLK_PRIOR => Ok(Keycode::Prior),
      SDLK_QUESTION => Ok(Keycode::Question),
      SDLK_QUOTE => Ok(Keycode::Quote),
      SDLK_QUOTEDBL => Ok(Keycode::DblQuote),
      SDLK_RALT => Ok(Keycode::RightAlt),
      SDLK_RCTRL => Ok(Keycode::RightCtrl),
      SDLK_RETURN => Ok(Keycode::Return),
      SDLK_RETURN2 => Ok(Keycode::Return2),
      SDLK_RGUI => Ok(Keycode::RightGUI),
      SDLK_RIGHT => Ok(Keycode::Right),
      SDLK_RIGHTBRACKET => Ok(Keycode::RightBracket),
      SDLK_RIGHTPAREN => Ok(Keycode::RightParen),
      SDLK_RSHIFT => Ok(Keycode::RightShift),
      SDLK_SCROLLLOCK => Ok(Keycode::ScrollLock),
      SDLK_SELECT => Ok(Keycode::Select),
      SDLK_SEMICOLON => Ok(Keycode::Semicolon),
      SDLK_SEPARATOR => Ok(Keycode::Separator),
      SDLK_SLASH => Ok(Keycode::Slash),
      SDLK_SLEEP => Ok(Keycode::Sleep),
      SDLK_SPACE => Ok(Keycode::Space),
      SDLK_STOP => Ok(Keycode::Stop),
      SDLK_SYSREQ => Ok(Keycode::SysReq),
      SDLK_TAB => Ok(Keycode::Tab),
      SDLK_THOUSANDSSEPARATOR => Ok(Keycode::ThousandsSeparator),
      SDLK_UNDERSCORE => Ok(Keycode::Underscore),
      SDLK_UNDO => Ok(Keycode::Undo),
      SDLK_UNKNOWN => Ok(Keycode::Unknown),
      SDLK_UP => Ok(Keycode::Up),
      SDLK_VOLUMEDOWN => Ok(Keycode::VolumeDown),
      SDLK_VOLUMEUP => Ok(Keycode::VolumeUp),
      SDLK_WWW => Ok(Keycode::WWW),
      SDLK_a => Ok(Keycode::A),
      SDLK_b => Ok(Keycode::B),
      SDLK_c => Ok(Keycode::C),
      SDLK_d => Ok(Keycode::D),
      SDLK_e => Ok(Keycode::E),
      SDLK_f => Ok(Keycode::F),
      SDLK_g => Ok(Keycode::G),
      SDLK_h => Ok(Keycode::H),
      SDLK_i => Ok(Keycode::I),
      SDLK_j => Ok(Keycode::J),
      SDLK_k => Ok(Keycode::K),
      SDLK_l => Ok(Keycode::L),
      SDLK_m => Ok(Keycode::M),
      SDLK_n => Ok(Keycode::N),
      SDLK_o => Ok(Keycode::O),
      SDLK_p => Ok(Keycode::P),
      SDLK_q => Ok(Keycode::Q),
      SDLK_r => Ok(Keycode::R),
      SDLK_s => Ok(Keycode::S),
      SDLK_t => Ok(Keycode::T),
      SDLK_u => Ok(Keycode::U),
      SDLK_v => Ok(Keycode::V),
      SDLK_w => Ok(Keycode::W),
      SDLK_x => Ok(Keycode::X),
      SDLK_y => Ok(Keycode::Y),
      SDLK_z => Ok(Keycode::Z),
      _ => Err(()),
    }
  }
}
#[test]
pub fn test_sdlk_a() {
  assert_eq!(
    Keycode::try_from(fermium::_bindgen_ty_7::SDLK_a as i32),
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
  _0 = SDL_SCANCODE_0,
  _1 = SDL_SCANCODE_1,
  _2 = SDL_SCANCODE_2,
  _3 = SDL_SCANCODE_3,
  _4 = SDL_SCANCODE_4,
  _5 = SDL_SCANCODE_5,
  _6 = SDL_SCANCODE_6,
  _7 = SDL_SCANCODE_7,
  _8 = SDL_SCANCODE_8,
  _9 = SDL_SCANCODE_9,
  A = SDL_SCANCODE_A,
  AC_Back = SDL_SCANCODE_AC_BACK,
  AC_Bookmarks = SDL_SCANCODE_AC_BOOKMARKS,
  AC_Forward = SDL_SCANCODE_AC_FORWARD,
  AC_Home = SDL_SCANCODE_AC_HOME,
  AC_Refresh = SDL_SCANCODE_AC_REFRESH,
  AC_Search = SDL_SCANCODE_AC_SEARCH,
  AC_Stop = SDL_SCANCODE_AC_STOP,
  Again = SDL_SCANCODE_AGAIN,
  AltErase = SDL_SCANCODE_ALTERASE,
  Apostrophe = SDL_SCANCODE_APOSTROPHE,
  App1 = SDL_SCANCODE_APP1,
  App2 = SDL_SCANCODE_APP2,
  Application = SDL_SCANCODE_APPLICATION,
  AudioFastForward = SDL_SCANCODE_AUDIOFASTFORWARD,
  AudioMute = SDL_SCANCODE_AUDIOMUTE,
  AudioNext = SDL_SCANCODE_AUDIONEXT,
  AudioPlay = SDL_SCANCODE_AUDIOPLAY,
  AudioPrev = SDL_SCANCODE_AUDIOPREV,
  AudioRewind = SDL_SCANCODE_AUDIOREWIND,
  AudioStop = SDL_SCANCODE_AUDIOSTOP,
  B = SDL_SCANCODE_B,
  Backslash = SDL_SCANCODE_BACKSLASH,
  Backspace = SDL_SCANCODE_BACKSPACE,
  BrightnessDown = SDL_SCANCODE_BRIGHTNESSDOWN,
  BrightnessUp = SDL_SCANCODE_BRIGHTNESSUP,
  C = SDL_SCANCODE_C,
  Calculator = SDL_SCANCODE_CALCULATOR,
  Cancel = SDL_SCANCODE_CANCEL,
  CapsLock = SDL_SCANCODE_CAPSLOCK,
  Clear = SDL_SCANCODE_CLEAR,
  ClearAgain = SDL_SCANCODE_CLEARAGAIN,
  Comma = SDL_SCANCODE_COMMA,
  Computer = SDL_SCANCODE_COMPUTER,
  Copy = SDL_SCANCODE_COPY,
  CrSel = SDL_SCANCODE_CRSEL,
  CurrencySubUnit = SDL_SCANCODE_CURRENCYSUBUNIT,
  CurrencyUnit = SDL_SCANCODE_CURRENCYUNIT,
  Cut = SDL_SCANCODE_CUT,
  D = SDL_SCANCODE_D,
  DecimalSeparator = SDL_SCANCODE_DECIMALSEPARATOR,
  Delete = SDL_SCANCODE_DELETE,
  DisplaySwitch = SDL_SCANCODE_DISPLAYSWITCH,
  Down = SDL_SCANCODE_DOWN,
  E = SDL_SCANCODE_E,
  Eject = SDL_SCANCODE_EJECT,
  End = SDL_SCANCODE_END,
  Equals = SDL_SCANCODE_EQUALS,
  Escape = SDL_SCANCODE_ESCAPE,
  Execute = SDL_SCANCODE_EXECUTE,
  ExSel = SDL_SCANCODE_EXSEL,
  F = SDL_SCANCODE_F,
  F1 = SDL_SCANCODE_F1,
  F2 = SDL_SCANCODE_F2,
  F3 = SDL_SCANCODE_F3,
  F4 = SDL_SCANCODE_F4,
  F5 = SDL_SCANCODE_F5,
  F6 = SDL_SCANCODE_F6,
  F7 = SDL_SCANCODE_F7,
  F8 = SDL_SCANCODE_F8,
  F9 = SDL_SCANCODE_F9,
  F10 = SDL_SCANCODE_F10,
  F11 = SDL_SCANCODE_F11,
  F12 = SDL_SCANCODE_F12,
  F13 = SDL_SCANCODE_F13,
  F14 = SDL_SCANCODE_F14,
  F15 = SDL_SCANCODE_F15,
  F16 = SDL_SCANCODE_F16,
  F17 = SDL_SCANCODE_F17,
  F18 = SDL_SCANCODE_F18,
  F19 = SDL_SCANCODE_F19,
  F20 = SDL_SCANCODE_F20,
  F21 = SDL_SCANCODE_F21,
  F22 = SDL_SCANCODE_F22,
  F23 = SDL_SCANCODE_F23,
  F24 = SDL_SCANCODE_F24,
  Find = SDL_SCANCODE_FIND,
  G = SDL_SCANCODE_G,
  Grave = SDL_SCANCODE_GRAVE,
  H = SDL_SCANCODE_H,
  Help = SDL_SCANCODE_HELP,
  Home = SDL_SCANCODE_HOME,
  I = SDL_SCANCODE_I,
  Insert = SDL_SCANCODE_INSERT,
  International1 = SDL_SCANCODE_INTERNATIONAL1,
  International2 = SDL_SCANCODE_INTERNATIONAL2,
  International3 = SDL_SCANCODE_INTERNATIONAL3,
  International4 = SDL_SCANCODE_INTERNATIONAL4,
  International5 = SDL_SCANCODE_INTERNATIONAL5,
  International6 = SDL_SCANCODE_INTERNATIONAL6,
  International7 = SDL_SCANCODE_INTERNATIONAL7,
  International8 = SDL_SCANCODE_INTERNATIONAL8,
  International9 = SDL_SCANCODE_INTERNATIONAL9,
  J = SDL_SCANCODE_J,
  K = SDL_SCANCODE_K,
  KbdIlluminationDown = SDL_SCANCODE_KBDILLUMDOWN,
  KbdIlluminationToggle = SDL_SCANCODE_KBDILLUMTOGGLE,
  KbdIlluminationUp = SDL_SCANCODE_KBDILLUMUP,
  KP_0 = SDL_SCANCODE_KP_0,
  KP_00 = SDL_SCANCODE_KP_00,
  KP_000 = SDL_SCANCODE_KP_000,
  KP_1 = SDL_SCANCODE_KP_1,
  KP_2 = SDL_SCANCODE_KP_2,
  KP_3 = SDL_SCANCODE_KP_3,
  KP_4 = SDL_SCANCODE_KP_4,
  KP_5 = SDL_SCANCODE_KP_5,
  KP_6 = SDL_SCANCODE_KP_6,
  KP_7 = SDL_SCANCODE_KP_7,
  KP_8 = SDL_SCANCODE_KP_8,
  KP_9 = SDL_SCANCODE_KP_9,
  KP_A = SDL_SCANCODE_KP_A,
  KP_Ampersand = SDL_SCANCODE_KP_AMPERSAND,
  KP_At = SDL_SCANCODE_KP_AT,
  KP_B = SDL_SCANCODE_KP_B,
  KP_Backspace = SDL_SCANCODE_KP_BACKSPACE,
  KP_Binary = SDL_SCANCODE_KP_BINARY,
  KP_C = SDL_SCANCODE_KP_C,
  KP_Clear = SDL_SCANCODE_KP_CLEAR,
  KP_ClearEntry = SDL_SCANCODE_KP_CLEARENTRY,
  KP_Colon = SDL_SCANCODE_KP_COLON,
  KP_Comma = SDL_SCANCODE_KP_COMMA,
  KP_D = SDL_SCANCODE_KP_D,
  KP_DblAmpersand = SDL_SCANCODE_KP_DBLAMPERSAND,
  KP_DblVerticalBar = SDL_SCANCODE_KP_DBLVERTICALBAR,
  KP_Decimal = SDL_SCANCODE_KP_DECIMAL,
  KP_Divide = SDL_SCANCODE_KP_DIVIDE,
  KP_E = SDL_SCANCODE_KP_E,
  KP_Enter = SDL_SCANCODE_KP_ENTER,
  KP_Equals = SDL_SCANCODE_KP_EQUALS,
  KP_EqualsAs400 = SDL_SCANCODE_KP_EQUALSAS400,
  KP_Exclamation = SDL_SCANCODE_KP_EXCLAM,
  KP_F = SDL_SCANCODE_KP_F,
  KP_Greater = SDL_SCANCODE_KP_GREATER,
  KP_Hash = SDL_SCANCODE_KP_HASH,
  KP_Hexadecimal = SDL_SCANCODE_KP_HEXADECIMAL,
  KP_LeftBrace = SDL_SCANCODE_KP_LEFTBRACE,
  KP_LeftParen = SDL_SCANCODE_KP_LEFTPAREN,
  KP_Less = SDL_SCANCODE_KP_LESS,
  KP_MemAdd = SDL_SCANCODE_KP_MEMADD,
  KP_MemClear = SDL_SCANCODE_KP_MEMCLEAR,
  KP_MemDivide = SDL_SCANCODE_KP_MEMDIVIDE,
  KP_MemMultiply = SDL_SCANCODE_KP_MEMMULTIPLY,
  KP_MemRecall = SDL_SCANCODE_KP_MEMRECALL,
  KP_MemStore = SDL_SCANCODE_KP_MEMSTORE,
  KP_MemSubtract = SDL_SCANCODE_KP_MEMSUBTRACT,
  KP_Minus = SDL_SCANCODE_KP_MINUS,
  KP_Multiply = SDL_SCANCODE_KP_MULTIPLY,
  KP_Octal = SDL_SCANCODE_KP_OCTAL,
  KP_Percent = SDL_SCANCODE_KP_PERCENT,
  KP_Period = SDL_SCANCODE_KP_PERIOD,
  KP_Plus = SDL_SCANCODE_KP_PLUS,
  KP_PlusMinus = SDL_SCANCODE_KP_PLUSMINUS,
  KP_Power = SDL_SCANCODE_KP_POWER,
  KP_RightBrace = SDL_SCANCODE_KP_RIGHTBRACE,
  KP_RightParen = SDL_SCANCODE_KP_RIGHTPAREN,
  KP_Space = SDL_SCANCODE_KP_SPACE,
  KP_Tab = SDL_SCANCODE_KP_TAB,
  KP_VerticalBar = SDL_SCANCODE_KP_VERTICALBAR,
  KP_Xor = SDL_SCANCODE_KP_XOR,
  L = SDL_SCANCODE_L,
  LeftAlt = SDL_SCANCODE_LALT,
  Lang1 = SDL_SCANCODE_LANG1,
  Lang2 = SDL_SCANCODE_LANG2,
  Lang3 = SDL_SCANCODE_LANG3,
  Lang4 = SDL_SCANCODE_LANG4,
  Lang5 = SDL_SCANCODE_LANG5,
  Lang6 = SDL_SCANCODE_LANG6,
  Lang7 = SDL_SCANCODE_LANG7,
  Lang8 = SDL_SCANCODE_LANG8,
  Lang9 = SDL_SCANCODE_LANG9,
  LeftCtrl = SDL_SCANCODE_LCTRL,
  Left = SDL_SCANCODE_LEFT,
  LeftBracket = SDL_SCANCODE_LEFTBRACKET,
  LeftGUI = SDL_SCANCODE_LGUI,
  LeftShift = SDL_SCANCODE_LSHIFT,
  M = SDL_SCANCODE_M,
  Mail = SDL_SCANCODE_MAIL,
  Mediaselect = SDL_SCANCODE_MEDIASELECT,
  Menu = SDL_SCANCODE_MENU,
  Minus = SDL_SCANCODE_MINUS,
  Mode = SDL_SCANCODE_MODE,
  Mute = SDL_SCANCODE_MUTE,
  N = SDL_SCANCODE_N,
  NonUSBackslash = SDL_SCANCODE_NONUSBACKSLASH,
  NonUSHash = SDL_SCANCODE_NONUSHASH,
  NumLockClear = SDL_SCANCODE_NUMLOCKCLEAR,
  O = SDL_SCANCODE_O,
  Oper = SDL_SCANCODE_OPER,
  Out = SDL_SCANCODE_OUT,
  P = SDL_SCANCODE_P,
  PageDown = SDL_SCANCODE_PAGEDOWN,
  PageUp = SDL_SCANCODE_PAGEUP,
  Paste = SDL_SCANCODE_PASTE,
  Pause = SDL_SCANCODE_PAUSE,
  Period = SDL_SCANCODE_PERIOD,
  Power = SDL_SCANCODE_POWER,
  PrintScreen = SDL_SCANCODE_PRINTSCREEN,
  Prior = SDL_SCANCODE_PRIOR,
  Q = SDL_SCANCODE_Q,
  R = SDL_SCANCODE_R,
  RightAlt = SDL_SCANCODE_RALT,
  RightCtrl = SDL_SCANCODE_RCTRL,
  Return = SDL_SCANCODE_RETURN,
  Return2 = SDL_SCANCODE_RETURN2,
  RightGUI = SDL_SCANCODE_RGUI,
  Right = SDL_SCANCODE_RIGHT,
  RightBracket = SDL_SCANCODE_RIGHTBRACKET,
  RightShift = SDL_SCANCODE_RSHIFT,
  S = SDL_SCANCODE_S,
  ScrollLock = SDL_SCANCODE_SCROLLLOCK,
  Select = SDL_SCANCODE_SELECT,
  Semicolon = SDL_SCANCODE_SEMICOLON,
  Separator = SDL_SCANCODE_SEPARATOR,
  Slash = SDL_SCANCODE_SLASH,
  Sleep = SDL_SCANCODE_SLEEP,
  Space = SDL_SCANCODE_SPACE,
  Stop = SDL_SCANCODE_STOP,
  SysReq = SDL_SCANCODE_SYSREQ,
  T = SDL_SCANCODE_T,
  Tab = SDL_SCANCODE_TAB,
  ThousandsSeparator = SDL_SCANCODE_THOUSANDSSEPARATOR,
  U = SDL_SCANCODE_U,
  Undo = SDL_SCANCODE_UNDO,
  Unknown = SDL_SCANCODE_UNKNOWN,
  Up = SDL_SCANCODE_UP,
  V = SDL_SCANCODE_V,
  VolumeDown = SDL_SCANCODE_VOLUMEDOWN,
  VolumeUp = SDL_SCANCODE_VOLUMEUP,
  W = SDL_SCANCODE_W,
  WWW = SDL_SCANCODE_WWW,
  X = SDL_SCANCODE_X,
  Y = SDL_SCANCODE_Y,
  Z = SDL_SCANCODE_Z,
}
impl TryFrom<fermium::SDL_Scancode::Type> for Scancode {
  type Error = ();
  fn try_from(t: fermium::SDL_Scancode::Type) -> Result<Self, Self::Error> {
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
