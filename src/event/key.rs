use super::*;

/// A keyboard event.
#[derive(Debug, Clone, Copy)]
pub struct KeyboardEvent {
  /// When?
  pub timestamp: u32,
  /// Which window had focus?
  pub window_id: u32,
  /// Is the key pressed?
  pub is_pressed: bool,
  /// The repeat count.
  pub repeat: u8,
  /// Info about the key and any modifiers.
  pub key: KeyInfo,
}

impl From<fermium::SDL_KeyboardEvent> for KeyboardEvent {
  fn from(ev: fermium::SDL_KeyboardEvent) -> Self {
    Self {
      timestamp: ev.timestamp,
      window_id: ev.windowID,
      is_pressed: u32::from(ev.state) == fermium::SDL_PRESSED,
      repeat: ev.repeat,
      key: KeyInfo::from(ev.keysym),
    }
  }
}
impl From<KeyboardEvent> for fermium::SDL_KeyboardEvent {
  fn from(ev: KeyboardEvent) -> Self {
    Self {
      type_: (if ev.is_pressed {
        fermium::SDL_KEYDOWN
      } else {
        fermium::SDL_KEYUP
      }) as u32,
      timestamp: ev.timestamp,
      windowID: ev.window_id,
      state: (if ev.is_pressed {
        fermium::SDL_PRESSED
      } else {
        fermium::SDL_RELEASED
      }) as u8,
      repeat: ev.repeat,
      keysym: fermium::SDL_Keysym::from(ev.key),
      padding2: 0,
      padding3: 0,
    }
  }
}

/// Info about a key pressed, and any modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyInfo {
  /// Physical scancode.
  pub scancode: Scancode,
  /// Virtual keycode.
  pub keycode: Keycode,
  /// The modifiers
  pub modifiers: KeyModifiers,
}

impl From<fermium::SDL_Keysym> for KeyInfo {
  fn from(key: fermium::SDL_Keysym) -> Self {
    Self {
      scancode: Scancode(key.scancode as u32),
      keycode: Keycode(key.sym as u32),
      modifiers: KeyModifiers(key.mod_),
    }
  }
}
impl From<KeyInfo> for fermium::SDL_Keysym {
  fn from(info: KeyInfo) -> Self {
    Self {
      scancode: info.scancode.0 as fermium::SDL_Scancode,
      sym: info.keycode.0 as fermium::SDL_Keycode,
      mod_: info.modifiers.0,
      unused: 0,
    }
  }
}

/// A bit bag of modifier keys being held.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyModifiers(pub(crate) u16);
#[allow(non_upper_case_globals)]
impl KeyModifiers {
  /// Left shift
  pub const LeftShift: KeyModifiers = KeyModifiers(fermium::KMOD_LSHIFT as u16);

  /// Right shift
  pub const RightShift: KeyModifiers =
    KeyModifiers(fermium::KMOD_RSHIFT as u16);

  /// Left control
  pub const LeftCtrl: KeyModifiers = KeyModifiers(fermium::KMOD_LCTRL as u16);

  /// Right control
  pub const RightCtrl: KeyModifiers = KeyModifiers(fermium::KMOD_RCTRL as u16);

  /// Left alt
  pub const LeftAlt: KeyModifiers = KeyModifiers(fermium::KMOD_LALT as u16);

  /// Right alt
  pub const RightAlt: KeyModifiers = KeyModifiers(fermium::KMOD_RALT as u16);

  /// Left GUI key (usually the windows key)
  pub const LeftGUI: KeyModifiers = KeyModifiers(fermium::KMOD_LGUI as u16);

  /// Right GUI key (usually the windows key)
  pub const RightGUI: KeyModifiers = KeyModifiers(fermium::KMOD_RGUI as u16);

  /// Caps Lock key
  pub const CapsLock: KeyModifiers = KeyModifiers(fermium::KMOD_CAPS as u16);

  /// Num Lock key
  pub const NumLock: KeyModifiers = KeyModifiers(fermium::KMOD_NUM as u16);

  /// AltGr key
  pub const AltGr: KeyModifiers = KeyModifiers(fermium::KMOD_MODE as u16);
}
impl KeyModifiers {
  /// Does the modifiers value on the left contain the modifiers value on the
  /// right?
  pub fn contains(self, modifiers: KeyModifiers) -> bool {
    (self.0 & modifiers.0) == modifiers.0
  }

  /// No modifiers at all.
  pub fn is_empty(self) -> bool {
    self.0 == 0
  }

  /// Either Shift is pressed.
  pub fn shift(self) -> bool {
    self.contains(Self::LeftShift) || self.contains(Self::RightShift)
  }

  /// Either Alt is pressed.
  pub fn alt(self) -> bool {
    self.contains(Self::LeftAlt) || self.contains(Self::RightAlt)
  }

  /// Either Ctrl is pressed.
  pub fn control(self) -> bool {
    self.contains(Self::LeftCtrl) || self.contains(Self::RightCtrl)
  }

  /// Either GUI is pressed.
  pub fn gui(self) -> bool {
    self.contains(Self::LeftGUI) || self.contains(Self::RightGUI)
  }
}

/// Physical scancode value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scancode(pub u32);
#[allow(missing_docs)]
impl Scancode {
  pub const UNKNOWN: Self = Self(fermium::SDL_SCANCODE_UNKNOWN as u32);
  pub const A: Self = Self(fermium::SDL_SCANCODE_A as u32);
  pub const B: Self = Self(fermium::SDL_SCANCODE_B as u32);
  pub const C: Self = Self(fermium::SDL_SCANCODE_C as u32);
  pub const D: Self = Self(fermium::SDL_SCANCODE_D as u32);
  pub const E: Self = Self(fermium::SDL_SCANCODE_E as u32);
  pub const F: Self = Self(fermium::SDL_SCANCODE_F as u32);
  pub const G: Self = Self(fermium::SDL_SCANCODE_G as u32);
  pub const H: Self = Self(fermium::SDL_SCANCODE_H as u32);
  pub const I: Self = Self(fermium::SDL_SCANCODE_I as u32);
  pub const J: Self = Self(fermium::SDL_SCANCODE_J as u32);
  pub const K: Self = Self(fermium::SDL_SCANCODE_K as u32);
  pub const L: Self = Self(fermium::SDL_SCANCODE_L as u32);
  pub const M: Self = Self(fermium::SDL_SCANCODE_M as u32);
  pub const N: Self = Self(fermium::SDL_SCANCODE_N as u32);
  pub const O: Self = Self(fermium::SDL_SCANCODE_O as u32);
  pub const P: Self = Self(fermium::SDL_SCANCODE_P as u32);
  pub const Q: Self = Self(fermium::SDL_SCANCODE_Q as u32);
  pub const R: Self = Self(fermium::SDL_SCANCODE_R as u32);
  pub const S: Self = Self(fermium::SDL_SCANCODE_S as u32);
  pub const T: Self = Self(fermium::SDL_SCANCODE_T as u32);
  pub const U: Self = Self(fermium::SDL_SCANCODE_U as u32);
  pub const V: Self = Self(fermium::SDL_SCANCODE_V as u32);
  pub const W: Self = Self(fermium::SDL_SCANCODE_W as u32);
  pub const X: Self = Self(fermium::SDL_SCANCODE_X as u32);
  pub const Y: Self = Self(fermium::SDL_SCANCODE_Y as u32);
  pub const Z: Self = Self(fermium::SDL_SCANCODE_Z as u32);
  pub const _1: Self = Self(fermium::SDL_SCANCODE_1 as u32);
  pub const _2: Self = Self(fermium::SDL_SCANCODE_2 as u32);
  pub const _3: Self = Self(fermium::SDL_SCANCODE_3 as u32);
  pub const _4: Self = Self(fermium::SDL_SCANCODE_4 as u32);
  pub const _5: Self = Self(fermium::SDL_SCANCODE_5 as u32);
  pub const _6: Self = Self(fermium::SDL_SCANCODE_6 as u32);
  pub const _7: Self = Self(fermium::SDL_SCANCODE_7 as u32);
  pub const _8: Self = Self(fermium::SDL_SCANCODE_8 as u32);
  pub const _9: Self = Self(fermium::SDL_SCANCODE_9 as u32);
  pub const _0: Self = Self(fermium::SDL_SCANCODE_0 as u32);
  pub const RETURN: Self = Self(fermium::SDL_SCANCODE_RETURN as u32);
  pub const ESCAPE: Self = Self(fermium::SDL_SCANCODE_ESCAPE as u32);
  pub const BACKSPACE: Self = Self(fermium::SDL_SCANCODE_BACKSPACE as u32);
  pub const TAB: Self = Self(fermium::SDL_SCANCODE_TAB as u32);
  pub const SPACE: Self = Self(fermium::SDL_SCANCODE_SPACE as u32);
  pub const MINUS: Self = Self(fermium::SDL_SCANCODE_MINUS as u32);
  pub const EQUALS: Self = Self(fermium::SDL_SCANCODE_EQUALS as u32);
  pub const LEFTBRACKET: Self = Self(fermium::SDL_SCANCODE_LEFTBRACKET as u32);
  pub const RIGHTBRACKET: Self = Self(fermium::SDL_SCANCODE_RIGHTBRACKET as u32);
  pub const BACKSLASH: Self = Self(fermium::SDL_SCANCODE_BACKSLASH as u32);
  pub const NONUSHASH: Self = Self(fermium::SDL_SCANCODE_NONUSHASH as u32);
  pub const SEMICOLON: Self = Self(fermium::SDL_SCANCODE_SEMICOLON as u32);
  pub const APOSTROPHE: Self = Self(fermium::SDL_SCANCODE_APOSTROPHE as u32);
  pub const GRAVE: Self = Self(fermium::SDL_SCANCODE_GRAVE as u32);
  pub const COMMA: Self = Self(fermium::SDL_SCANCODE_COMMA as u32);
  pub const PERIOD: Self = Self(fermium::SDL_SCANCODE_PERIOD as u32);
  pub const SLASH: Self = Self(fermium::SDL_SCANCODE_SLASH as u32);
  pub const CAPSLOCK: Self = Self(fermium::SDL_SCANCODE_CAPSLOCK as u32);
  pub const F1: Self = Self(fermium::SDL_SCANCODE_F1 as u32);
  pub const F2: Self = Self(fermium::SDL_SCANCODE_F2 as u32);
  pub const F3: Self = Self(fermium::SDL_SCANCODE_F3 as u32);
  pub const F4: Self = Self(fermium::SDL_SCANCODE_F4 as u32);
  pub const F5: Self = Self(fermium::SDL_SCANCODE_F5 as u32);
  pub const F6: Self = Self(fermium::SDL_SCANCODE_F6 as u32);
  pub const F7: Self = Self(fermium::SDL_SCANCODE_F7 as u32);
  pub const F8: Self = Self(fermium::SDL_SCANCODE_F8 as u32);
  pub const F9: Self = Self(fermium::SDL_SCANCODE_F9 as u32);
  pub const F10: Self = Self(fermium::SDL_SCANCODE_F10 as u32);
  pub const F11: Self = Self(fermium::SDL_SCANCODE_F11 as u32);
  pub const F12: Self = Self(fermium::SDL_SCANCODE_F12 as u32);
  pub const PRINTSCREEN: Self = Self(fermium::SDL_SCANCODE_PRINTSCREEN as u32);
  pub const SCROLLLOCK: Self = Self(fermium::SDL_SCANCODE_SCROLLLOCK as u32);
  pub const PAUSE: Self = Self(fermium::SDL_SCANCODE_PAUSE as u32);
  pub const INSERT: Self = Self(fermium::SDL_SCANCODE_INSERT as u32);
  pub const HOME: Self = Self(fermium::SDL_SCANCODE_HOME as u32);
  pub const PAGEUP: Self = Self(fermium::SDL_SCANCODE_PAGEUP as u32);
  pub const DELETE: Self = Self(fermium::SDL_SCANCODE_DELETE as u32);
  pub const END: Self = Self(fermium::SDL_SCANCODE_END as u32);
  pub const PAGEDOWN: Self = Self(fermium::SDL_SCANCODE_PAGEDOWN as u32);
  pub const RIGHT: Self = Self(fermium::SDL_SCANCODE_RIGHT as u32);
  pub const LEFT: Self = Self(fermium::SDL_SCANCODE_LEFT as u32);
  pub const DOWN: Self = Self(fermium::SDL_SCANCODE_DOWN as u32);
  pub const UP: Self = Self(fermium::SDL_SCANCODE_UP as u32);
  pub const NUMLOCKCLEAR: Self = Self(fermium::SDL_SCANCODE_NUMLOCKCLEAR as u32);
  pub const KP_DIVIDE: Self = Self(fermium::SDL_SCANCODE_KP_DIVIDE as u32);
  pub const KP_MULTIPLY: Self = Self(fermium::SDL_SCANCODE_KP_MULTIPLY as u32);
  pub const KP_MINUS: Self = Self(fermium::SDL_SCANCODE_KP_MINUS as u32);
  pub const KP_PLUS: Self = Self(fermium::SDL_SCANCODE_KP_PLUS as u32);
  pub const KP_ENTER: Self = Self(fermium::SDL_SCANCODE_KP_ENTER as u32);
  pub const KP_1: Self = Self(fermium::SDL_SCANCODE_KP_1 as u32);
  pub const KP_2: Self = Self(fermium::SDL_SCANCODE_KP_2 as u32);
  pub const KP_3: Self = Self(fermium::SDL_SCANCODE_KP_3 as u32);
  pub const KP_4: Self = Self(fermium::SDL_SCANCODE_KP_4 as u32);
  pub const KP_5: Self = Self(fermium::SDL_SCANCODE_KP_5 as u32);
  pub const KP_6: Self = Self(fermium::SDL_SCANCODE_KP_6 as u32);
  pub const KP_7: Self = Self(fermium::SDL_SCANCODE_KP_7 as u32);
  pub const KP_8: Self = Self(fermium::SDL_SCANCODE_KP_8 as u32);
  pub const KP_9: Self = Self(fermium::SDL_SCANCODE_KP_9 as u32);
  pub const KP_0: Self = Self(fermium::SDL_SCANCODE_KP_0 as u32);
  pub const KP_PERIOD: Self = Self(fermium::SDL_SCANCODE_KP_PERIOD as u32);
  pub const NONUSBACKSLASH: Self = Self(fermium::SDL_SCANCODE_NONUSBACKSLASH as u32);
  pub const APPLICATION: Self = Self(fermium::SDL_SCANCODE_APPLICATION as u32);
  pub const POWER: Self = Self(fermium::SDL_SCANCODE_POWER as u32);
  pub const KP_EQUALS: Self = Self(fermium::SDL_SCANCODE_KP_EQUALS as u32);
  pub const F13: Self = Self(fermium::SDL_SCANCODE_F13 as u32);
  pub const F14: Self = Self(fermium::SDL_SCANCODE_F14 as u32);
  pub const F15: Self = Self(fermium::SDL_SCANCODE_F15 as u32);
  pub const F16: Self = Self(fermium::SDL_SCANCODE_F16 as u32);
  pub const F17: Self = Self(fermium::SDL_SCANCODE_F17 as u32);
  pub const F18: Self = Self(fermium::SDL_SCANCODE_F18 as u32);
  pub const F19: Self = Self(fermium::SDL_SCANCODE_F19 as u32);
  pub const F20: Self = Self(fermium::SDL_SCANCODE_F20 as u32);
  pub const F21: Self = Self(fermium::SDL_SCANCODE_F21 as u32);
  pub const F22: Self = Self(fermium::SDL_SCANCODE_F22 as u32);
  pub const F23: Self = Self(fermium::SDL_SCANCODE_F23 as u32);
  pub const F24: Self = Self(fermium::SDL_SCANCODE_F24 as u32);
  pub const EXECUTE: Self = Self(fermium::SDL_SCANCODE_EXECUTE as u32);
  pub const HELP: Self = Self(fermium::SDL_SCANCODE_HELP as u32);
  pub const MENU: Self = Self(fermium::SDL_SCANCODE_MENU as u32);
  pub const SELECT: Self = Self(fermium::SDL_SCANCODE_SELECT as u32);
  pub const STOP: Self = Self(fermium::SDL_SCANCODE_STOP as u32);
  pub const AGAIN: Self = Self(fermium::SDL_SCANCODE_AGAIN as u32);
  pub const UNDO: Self = Self(fermium::SDL_SCANCODE_UNDO as u32);
  pub const CUT: Self = Self(fermium::SDL_SCANCODE_CUT as u32);
  pub const COPY: Self = Self(fermium::SDL_SCANCODE_COPY as u32);
  pub const PASTE: Self = Self(fermium::SDL_SCANCODE_PASTE as u32);
  pub const FIND: Self = Self(fermium::SDL_SCANCODE_FIND as u32);
  pub const MUTE: Self = Self(fermium::SDL_SCANCODE_MUTE as u32);
  pub const VOLUMEUP: Self = Self(fermium::SDL_SCANCODE_VOLUMEUP as u32);
  pub const VOLUMEDOWN: Self = Self(fermium::SDL_SCANCODE_VOLUMEDOWN as u32);
  pub const KP_COMMA: Self = Self(fermium::SDL_SCANCODE_KP_COMMA as u32);
  pub const KP_EQUALSAS400: Self = Self(fermium::SDL_SCANCODE_KP_EQUALSAS400 as u32);
  pub const INTERNATIONAL1: Self = Self(fermium::SDL_SCANCODE_INTERNATIONAL1 as u32);
  pub const INTERNATIONAL2: Self = Self(fermium::SDL_SCANCODE_INTERNATIONAL2 as u32);
  pub const INTERNATIONAL3: Self = Self(fermium::SDL_SCANCODE_INTERNATIONAL3 as u32);
  pub const INTERNATIONAL4: Self = Self(fermium::SDL_SCANCODE_INTERNATIONAL4 as u32);
  pub const INTERNATIONAL5: Self = Self(fermium::SDL_SCANCODE_INTERNATIONAL5 as u32);
  pub const INTERNATIONAL6: Self = Self(fermium::SDL_SCANCODE_INTERNATIONAL6 as u32);
  pub const INTERNATIONAL7: Self = Self(fermium::SDL_SCANCODE_INTERNATIONAL7 as u32);
  pub const INTERNATIONAL8: Self = Self(fermium::SDL_SCANCODE_INTERNATIONAL8 as u32);
  pub const INTERNATIONAL9: Self = Self(fermium::SDL_SCANCODE_INTERNATIONAL9 as u32);
  pub const LANG1: Self = Self(fermium::SDL_SCANCODE_LANG1 as u32);
  pub const LANG2: Self = Self(fermium::SDL_SCANCODE_LANG2 as u32);
  pub const LANG3: Self = Self(fermium::SDL_SCANCODE_LANG3 as u32);
  pub const LANG4: Self = Self(fermium::SDL_SCANCODE_LANG4 as u32);
  pub const LANG5: Self = Self(fermium::SDL_SCANCODE_LANG5 as u32);
  pub const LANG6: Self = Self(fermium::SDL_SCANCODE_LANG6 as u32);
  pub const LANG7: Self = Self(fermium::SDL_SCANCODE_LANG7 as u32);
  pub const LANG8: Self = Self(fermium::SDL_SCANCODE_LANG8 as u32);
  pub const LANG9: Self = Self(fermium::SDL_SCANCODE_LANG9 as u32);
  pub const ALTERASE: Self = Self(fermium::SDL_SCANCODE_ALTERASE as u32);
  pub const SYSREQ: Self = Self(fermium::SDL_SCANCODE_SYSREQ as u32);
  pub const CANCEL: Self = Self(fermium::SDL_SCANCODE_CANCEL as u32);
  pub const CLEAR: Self = Self(fermium::SDL_SCANCODE_CLEAR as u32);
  pub const PRIOR: Self = Self(fermium::SDL_SCANCODE_PRIOR as u32);
  pub const RETURN2: Self = Self(fermium::SDL_SCANCODE_RETURN2 as u32);
  pub const SEPARATOR: Self = Self(fermium::SDL_SCANCODE_SEPARATOR as u32);
  pub const OUT: Self = Self(fermium::SDL_SCANCODE_OUT as u32);
  pub const OPER: Self = Self(fermium::SDL_SCANCODE_OPER as u32);
  pub const CLEARAGAIN: Self = Self(fermium::SDL_SCANCODE_CLEARAGAIN as u32);
  pub const CRSEL: Self = Self(fermium::SDL_SCANCODE_CRSEL as u32);
  pub const EXSEL: Self = Self(fermium::SDL_SCANCODE_EXSEL as u32);
  pub const KP_00: Self = Self(fermium::SDL_SCANCODE_KP_00 as u32);
  pub const KP_000: Self = Self(fermium::SDL_SCANCODE_KP_000 as u32);
  pub const THOUSANDSSEPARATOR: Self = Self(fermium::SDL_SCANCODE_THOUSANDSSEPARATOR as u32);
  pub const DECIMALSEPARATOR: Self = Self(fermium::SDL_SCANCODE_DECIMALSEPARATOR as u32);
  pub const CURRENCYUNIT: Self = Self(fermium::SDL_SCANCODE_CURRENCYUNIT as u32);
  pub const CURRENCYSUBUNIT: Self = Self(fermium::SDL_SCANCODE_CURRENCYSUBUNIT as u32);
  pub const KP_LEFTPAREN: Self = Self(fermium::SDL_SCANCODE_KP_LEFTPAREN as u32);
  pub const KP_RIGHTPAREN: Self = Self(fermium::SDL_SCANCODE_KP_RIGHTPAREN as u32);
  pub const KP_LEFTBRACE: Self = Self(fermium::SDL_SCANCODE_KP_LEFTBRACE as u32);
  pub const KP_RIGHTBRACE: Self = Self(fermium::SDL_SCANCODE_KP_RIGHTBRACE as u32);
  pub const KP_TAB: Self = Self(fermium::SDL_SCANCODE_KP_TAB as u32);
  pub const KP_BACKSPACE: Self = Self(fermium::SDL_SCANCODE_KP_BACKSPACE as u32);
  pub const KP_A: Self = Self(fermium::SDL_SCANCODE_KP_A as u32);
  pub const KP_B: Self = Self(fermium::SDL_SCANCODE_KP_B as u32);
  pub const KP_C: Self = Self(fermium::SDL_SCANCODE_KP_C as u32);
  pub const KP_D: Self = Self(fermium::SDL_SCANCODE_KP_D as u32);
  pub const KP_E: Self = Self(fermium::SDL_SCANCODE_KP_E as u32);
  pub const KP_F: Self = Self(fermium::SDL_SCANCODE_KP_F as u32);
  pub const KP_XOR: Self = Self(fermium::SDL_SCANCODE_KP_XOR as u32);
  pub const KP_POWER: Self = Self(fermium::SDL_SCANCODE_KP_POWER as u32);
  pub const KP_PERCENT: Self = Self(fermium::SDL_SCANCODE_KP_PERCENT as u32);
  pub const KP_LESS: Self = Self(fermium::SDL_SCANCODE_KP_LESS as u32);
  pub const KP_GREATER: Self = Self(fermium::SDL_SCANCODE_KP_GREATER as u32);
  pub const KP_AMPERSAND: Self = Self(fermium::SDL_SCANCODE_KP_AMPERSAND as u32);
  pub const KP_DBLAMPERSAND: Self = Self(fermium::SDL_SCANCODE_KP_DBLAMPERSAND as u32);
  pub const KP_VERTICALBAR: Self = Self(fermium::SDL_SCANCODE_KP_VERTICALBAR as u32);
  pub const KP_DBLVERTICALBAR: Self = Self(fermium::SDL_SCANCODE_KP_DBLVERTICALBAR as u32);
  pub const KP_COLON: Self = Self(fermium::SDL_SCANCODE_KP_COLON as u32);
  pub const KP_HASH: Self = Self(fermium::SDL_SCANCODE_KP_HASH as u32);
  pub const KP_SPACE: Self = Self(fermium::SDL_SCANCODE_KP_SPACE as u32);
  pub const KP_AT: Self = Self(fermium::SDL_SCANCODE_KP_AT as u32);
  pub const KP_EXCLAM: Self = Self(fermium::SDL_SCANCODE_KP_EXCLAM as u32);
  pub const KP_MEMSTORE: Self = Self(fermium::SDL_SCANCODE_KP_MEMSTORE as u32);
  pub const KP_MEMRECALL: Self = Self(fermium::SDL_SCANCODE_KP_MEMRECALL as u32);
  pub const KP_MEMCLEAR: Self = Self(fermium::SDL_SCANCODE_KP_MEMCLEAR as u32);
  pub const KP_MEMADD: Self = Self(fermium::SDL_SCANCODE_KP_MEMADD as u32);
  pub const KP_MEMSUBTRACT: Self = Self(fermium::SDL_SCANCODE_KP_MEMSUBTRACT as u32);
  pub const KP_MEMMULTIPLY: Self = Self(fermium::SDL_SCANCODE_KP_MEMMULTIPLY as u32);
  pub const KP_MEMDIVIDE: Self = Self(fermium::SDL_SCANCODE_KP_MEMDIVIDE as u32);
  pub const KP_PLUSMINUS: Self = Self(fermium::SDL_SCANCODE_KP_PLUSMINUS as u32);
  pub const KP_CLEAR: Self = Self(fermium::SDL_SCANCODE_KP_CLEAR as u32);
  pub const KP_CLEARENTRY: Self = Self(fermium::SDL_SCANCODE_KP_CLEARENTRY as u32);
  pub const KP_BINARY: Self = Self(fermium::SDL_SCANCODE_KP_BINARY as u32);
  pub const KP_OCTAL: Self = Self(fermium::SDL_SCANCODE_KP_OCTAL as u32);
  pub const KP_DECIMAL: Self = Self(fermium::SDL_SCANCODE_KP_DECIMAL as u32);
  pub const KP_HEXADECIMAL: Self = Self(fermium::SDL_SCANCODE_KP_HEXADECIMAL as u32);
  pub const LCTRL: Self = Self(fermium::SDL_SCANCODE_LCTRL as u32);
  pub const LSHIFT: Self = Self(fermium::SDL_SCANCODE_LSHIFT as u32);
  pub const LALT: Self = Self(fermium::SDL_SCANCODE_LALT as u32);
  pub const LGUI: Self = Self(fermium::SDL_SCANCODE_LGUI as u32);
  pub const RCTRL: Self = Self(fermium::SDL_SCANCODE_RCTRL as u32);
  pub const RSHIFT: Self = Self(fermium::SDL_SCANCODE_RSHIFT as u32);
  pub const RALT: Self = Self(fermium::SDL_SCANCODE_RALT as u32);
  pub const RGUI: Self = Self(fermium::SDL_SCANCODE_RGUI as u32);
  pub const MODE: Self = Self(fermium::SDL_SCANCODE_MODE as u32);
  pub const AUDIONEXT: Self = Self(fermium::SDL_SCANCODE_AUDIONEXT as u32);
  pub const AUDIOPREV: Self = Self(fermium::SDL_SCANCODE_AUDIOPREV as u32);
  pub const AUDIOSTOP: Self = Self(fermium::SDL_SCANCODE_AUDIOSTOP as u32);
  pub const AUDIOPLAY: Self = Self(fermium::SDL_SCANCODE_AUDIOPLAY as u32);
  pub const AUDIOMUTE: Self = Self(fermium::SDL_SCANCODE_AUDIOMUTE as u32);
  pub const MEDIASELECT: Self = Self(fermium::SDL_SCANCODE_MEDIASELECT as u32);
  pub const WWW: Self = Self(fermium::SDL_SCANCODE_WWW as u32);
  pub const MAIL: Self = Self(fermium::SDL_SCANCODE_MAIL as u32);
  pub const CALCULATOR: Self = Self(fermium::SDL_SCANCODE_CALCULATOR as u32);
  pub const COMPUTER: Self = Self(fermium::SDL_SCANCODE_COMPUTER as u32);
  pub const AC_SEARCH: Self = Self(fermium::SDL_SCANCODE_AC_SEARCH as u32);
  pub const AC_HOME: Self = Self(fermium::SDL_SCANCODE_AC_HOME as u32);
  pub const AC_BACK: Self = Self(fermium::SDL_SCANCODE_AC_BACK as u32);
  pub const AC_FORWARD: Self = Self(fermium::SDL_SCANCODE_AC_FORWARD as u32);
  pub const AC_STOP: Self = Self(fermium::SDL_SCANCODE_AC_STOP as u32);
  pub const AC_REFRESH: Self = Self(fermium::SDL_SCANCODE_AC_REFRESH as u32);
  pub const AC_BOOKMARKS: Self = Self(fermium::SDL_SCANCODE_AC_BOOKMARKS as u32);
  pub const BRIGHTNESSDOWN: Self = Self(fermium::SDL_SCANCODE_BRIGHTNESSDOWN as u32);
  pub const BRIGHTNESSUP: Self = Self(fermium::SDL_SCANCODE_BRIGHTNESSUP as u32);
  pub const DISPLAYSWITCH: Self = Self(fermium::SDL_SCANCODE_DISPLAYSWITCH as u32);
  pub const KBDILLUMTOGGLE: Self = Self(fermium::SDL_SCANCODE_KBDILLUMTOGGLE as u32);
  pub const KBDILLUMDOWN: Self = Self(fermium::SDL_SCANCODE_KBDILLUMDOWN as u32);
  pub const KBDILLUMUP: Self = Self(fermium::SDL_SCANCODE_KBDILLUMUP as u32);
  pub const EJECT: Self = Self(fermium::SDL_SCANCODE_EJECT as u32);
  pub const SLEEP: Self = Self(fermium::SDL_SCANCODE_SLEEP as u32);
  pub const APP1: Self = Self(fermium::SDL_SCANCODE_APP1 as u32);
  pub const APP2: Self = Self(fermium::SDL_SCANCODE_APP2 as u32);
  pub const AUDIOREWIND: Self = Self(fermium::SDL_SCANCODE_AUDIOREWIND as u32);
  pub const AUDIOFASTFORWARD: Self = Self(fermium::SDL_SCANCODE_AUDIOFASTFORWARD as u32);
}

/// Virtual keycode value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Keycode(pub u32);
#[allow(missing_docs)]
impl Keycode {
  pub const UNKNOWN: Self = Self(fermium::SDLK_UNKNOWN as u32);
  pub const RETURN: Self = Self(fermium::SDLK_RETURN as u32);
  pub const ESCAPE: Self = Self(fermium::SDLK_ESCAPE as u32);
  pub const BACKSPACE: Self = Self(fermium::SDLK_BACKSPACE as u32);
  pub const TAB: Self = Self(fermium::SDLK_TAB as u32);
  pub const SPACE: Self = Self(fermium::SDLK_SPACE as u32);
  pub const EXCLAIM: Self = Self(fermium::SDLK_EXCLAIM as u32);
  pub const QUOTEDBL: Self = Self(fermium::SDLK_QUOTEDBL as u32);
  pub const HASH: Self = Self(fermium::SDLK_HASH as u32);
  pub const PERCENT: Self = Self(fermium::SDLK_PERCENT as u32);
  pub const DOLLAR: Self = Self(fermium::SDLK_DOLLAR as u32);
  pub const AMPERSAND: Self = Self(fermium::SDLK_AMPERSAND as u32);
  pub const QUOTE: Self = Self(fermium::SDLK_QUOTE as u32);
  pub const LEFTPAREN: Self = Self(fermium::SDLK_LEFTPAREN as u32);
  pub const RIGHTPAREN: Self = Self(fermium::SDLK_RIGHTPAREN as u32);
  pub const ASTERISK: Self = Self(fermium::SDLK_ASTERISK as u32);
  pub const PLUS: Self = Self(fermium::SDLK_PLUS as u32);
  pub const COMMA: Self = Self(fermium::SDLK_COMMA as u32);
  pub const MINUS: Self = Self(fermium::SDLK_MINUS as u32);
  pub const PERIOD: Self = Self(fermium::SDLK_PERIOD as u32);
  pub const SLASH: Self = Self(fermium::SDLK_SLASH as u32);
  pub const _0: Self = Self(fermium::SDLK_0 as u32);
  pub const _1: Self = Self(fermium::SDLK_1 as u32);
  pub const _2: Self = Self(fermium::SDLK_2 as u32);
  pub const _3: Self = Self(fermium::SDLK_3 as u32);
  pub const _4: Self = Self(fermium::SDLK_4 as u32);
  pub const _5: Self = Self(fermium::SDLK_5 as u32);
  pub const _6: Self = Self(fermium::SDLK_6 as u32);
  pub const _7: Self = Self(fermium::SDLK_7 as u32);
  pub const _8: Self = Self(fermium::SDLK_8 as u32);
  pub const _9: Self = Self(fermium::SDLK_9 as u32);
  pub const COLON: Self = Self(fermium::SDLK_COLON as u32);
  pub const SEMICOLON: Self = Self(fermium::SDLK_SEMICOLON as u32);
  pub const LESS: Self = Self(fermium::SDLK_LESS as u32);
  pub const EQUALS: Self = Self(fermium::SDLK_EQUALS as u32);
  pub const GREATER: Self = Self(fermium::SDLK_GREATER as u32);
  pub const QUESTION: Self = Self(fermium::SDLK_QUESTION as u32);
  pub const AT: Self = Self(fermium::SDLK_AT as u32);
  pub const LEFTBRACKET: Self = Self(fermium::SDLK_LEFTBRACKET as u32);
  pub const BACKSLASH: Self = Self(fermium::SDLK_BACKSLASH as u32);
  pub const RIGHTBRACKET: Self = Self(fermium::SDLK_RIGHTBRACKET as u32);
  pub const CARET: Self = Self(fermium::SDLK_CARET as u32);
  pub const UNDERSCORE: Self = Self(fermium::SDLK_UNDERSCORE as u32);
  pub const BACKQUOTE: Self = Self(fermium::SDLK_BACKQUOTE as u32);
  pub const A: Self = Self(fermium::SDLK_a as u32);
  pub const B: Self = Self(fermium::SDLK_b as u32);
  pub const C: Self = Self(fermium::SDLK_c as u32);
  pub const D: Self = Self(fermium::SDLK_d as u32);
  pub const E: Self = Self(fermium::SDLK_e as u32);
  pub const F: Self = Self(fermium::SDLK_f as u32);
  pub const G: Self = Self(fermium::SDLK_g as u32);
  pub const H: Self = Self(fermium::SDLK_h as u32);
  pub const I: Self = Self(fermium::SDLK_i as u32);
  pub const J: Self = Self(fermium::SDLK_j as u32);
  pub const K: Self = Self(fermium::SDLK_k as u32);
  pub const L: Self = Self(fermium::SDLK_l as u32);
  pub const M: Self = Self(fermium::SDLK_m as u32);
  pub const N: Self = Self(fermium::SDLK_n as u32);
  pub const O: Self = Self(fermium::SDLK_o as u32);
  pub const P: Self = Self(fermium::SDLK_p as u32);
  pub const Q: Self = Self(fermium::SDLK_q as u32);
  pub const R: Self = Self(fermium::SDLK_r as u32);
  pub const S: Self = Self(fermium::SDLK_s as u32);
  pub const T: Self = Self(fermium::SDLK_t as u32);
  pub const U: Self = Self(fermium::SDLK_u as u32);
  pub const V: Self = Self(fermium::SDLK_v as u32);
  pub const W: Self = Self(fermium::SDLK_w as u32);
  pub const X: Self = Self(fermium::SDLK_x as u32);
  pub const Y: Self = Self(fermium::SDLK_y as u32);
  pub const Z: Self = Self(fermium::SDLK_z as u32);
  pub const CAPSLOCK: Self = Self(fermium::SDLK_CAPSLOCK as u32);
  pub const F1: Self = Self(fermium::SDLK_F1 as u32);
  pub const F2: Self = Self(fermium::SDLK_F2 as u32);
  pub const F3: Self = Self(fermium::SDLK_F3 as u32);
  pub const F4: Self = Self(fermium::SDLK_F4 as u32);
  pub const F5: Self = Self(fermium::SDLK_F5 as u32);
  pub const F6: Self = Self(fermium::SDLK_F6 as u32);
  pub const F7: Self = Self(fermium::SDLK_F7 as u32);
  pub const F8: Self = Self(fermium::SDLK_F8 as u32);
  pub const F9: Self = Self(fermium::SDLK_F9 as u32);
  pub const F10: Self = Self(fermium::SDLK_F10 as u32);
  pub const F11: Self = Self(fermium::SDLK_F11 as u32);
  pub const F12: Self = Self(fermium::SDLK_F12 as u32);
  pub const PRINTSCREEN: Self = Self(fermium::SDLK_PRINTSCREEN as u32);
  pub const SCROLLLOCK: Self = Self(fermium::SDLK_SCROLLLOCK as u32);
  pub const PAUSE: Self = Self(fermium::SDLK_PAUSE as u32);
  pub const INSERT: Self = Self(fermium::SDLK_INSERT as u32);
  pub const HOME: Self = Self(fermium::SDLK_HOME as u32);
  pub const PAGEUP: Self = Self(fermium::SDLK_PAGEUP as u32);
  pub const DELETE: Self = Self(fermium::SDLK_DELETE as u32);
  pub const END: Self = Self(fermium::SDLK_END as u32);
  pub const PAGEDOWN: Self = Self(fermium::SDLK_PAGEDOWN as u32);
  pub const RIGHT: Self = Self(fermium::SDLK_RIGHT as u32);
  pub const LEFT: Self = Self(fermium::SDLK_LEFT as u32);
  pub const DOWN: Self = Self(fermium::SDLK_DOWN as u32);
  pub const UP: Self = Self(fermium::SDLK_UP as u32);
  pub const NUMLOCKCLEAR: Self = Self(fermium::SDLK_NUMLOCKCLEAR as u32);
  pub const KP_DIVIDE: Self = Self(fermium::SDLK_KP_DIVIDE as u32);
  pub const KP_MULTIPLY: Self = Self(fermium::SDLK_KP_MULTIPLY as u32);
  pub const KP_MINUS: Self = Self(fermium::SDLK_KP_MINUS as u32);
  pub const KP_PLUS: Self = Self(fermium::SDLK_KP_PLUS as u32);
  pub const KP_ENTER: Self = Self(fermium::SDLK_KP_ENTER as u32);
  pub const KP_1: Self = Self(fermium::SDLK_KP_1 as u32);
  pub const KP_2: Self = Self(fermium::SDLK_KP_2 as u32);
  pub const KP_3: Self = Self(fermium::SDLK_KP_3 as u32);
  pub const KP_4: Self = Self(fermium::SDLK_KP_4 as u32);
  pub const KP_5: Self = Self(fermium::SDLK_KP_5 as u32);
  pub const KP_6: Self = Self(fermium::SDLK_KP_6 as u32);
  pub const KP_7: Self = Self(fermium::SDLK_KP_7 as u32);
  pub const KP_8: Self = Self(fermium::SDLK_KP_8 as u32);
  pub const KP_9: Self = Self(fermium::SDLK_KP_9 as u32);
  pub const KP_0: Self = Self(fermium::SDLK_KP_0 as u32);
  pub const KP_PERIOD: Self = Self(fermium::SDLK_KP_PERIOD as u32);
  pub const APPLICATION: Self = Self(fermium::SDLK_APPLICATION as u32);
  pub const POWER: Self = Self(fermium::SDLK_POWER as u32);
  pub const KP_EQUALS: Self = Self(fermium::SDLK_KP_EQUALS as u32);
  pub const F13: Self = Self(fermium::SDLK_F13 as u32);
  pub const F14: Self = Self(fermium::SDLK_F14 as u32);
  pub const F15: Self = Self(fermium::SDLK_F15 as u32);
  pub const F16: Self = Self(fermium::SDLK_F16 as u32);
  pub const F17: Self = Self(fermium::SDLK_F17 as u32);
  pub const F18: Self = Self(fermium::SDLK_F18 as u32);
  pub const F19: Self = Self(fermium::SDLK_F19 as u32);
  pub const F20: Self = Self(fermium::SDLK_F20 as u32);
  pub const F21: Self = Self(fermium::SDLK_F21 as u32);
  pub const F22: Self = Self(fermium::SDLK_F22 as u32);
  pub const F23: Self = Self(fermium::SDLK_F23 as u32);
  pub const F24: Self = Self(fermium::SDLK_F24 as u32);
  pub const EXECUTE: Self = Self(fermium::SDLK_EXECUTE as u32);
  pub const HELP: Self = Self(fermium::SDLK_HELP as u32);
  pub const MENU: Self = Self(fermium::SDLK_MENU as u32);
  pub const SELECT: Self = Self(fermium::SDLK_SELECT as u32);
  pub const STOP: Self = Self(fermium::SDLK_STOP as u32);
  pub const AGAIN: Self = Self(fermium::SDLK_AGAIN as u32);
  pub const UNDO: Self = Self(fermium::SDLK_UNDO as u32);
  pub const CUT: Self = Self(fermium::SDLK_CUT as u32);
  pub const COPY: Self = Self(fermium::SDLK_COPY as u32);
  pub const PASTE: Self = Self(fermium::SDLK_PASTE as u32);
  pub const FIND: Self = Self(fermium::SDLK_FIND as u32);
  pub const MUTE: Self = Self(fermium::SDLK_MUTE as u32);
  pub const VOLUMEUP: Self = Self(fermium::SDLK_VOLUMEUP as u32);
  pub const VOLUMEDOWN: Self = Self(fermium::SDLK_VOLUMEDOWN as u32);
  pub const KP_COMMA: Self = Self(fermium::SDLK_KP_COMMA as u32);
  pub const KP_EQUALSAS400: Self = Self(fermium::SDLK_KP_EQUALSAS400 as u32);
  pub const ALTERASE: Self = Self(fermium::SDLK_ALTERASE as u32);
  pub const SYSREQ: Self = Self(fermium::SDLK_SYSREQ as u32);
  pub const CANCEL: Self = Self(fermium::SDLK_CANCEL as u32);
  pub const CLEAR: Self = Self(fermium::SDLK_CLEAR as u32);
  pub const PRIOR: Self = Self(fermium::SDLK_PRIOR as u32);
  pub const RETURN2: Self = Self(fermium::SDLK_RETURN2 as u32);
  pub const SEPARATOR: Self = Self(fermium::SDLK_SEPARATOR as u32);
  pub const OUT: Self = Self(fermium::SDLK_OUT as u32);
  pub const OPER: Self = Self(fermium::SDLK_OPER as u32);
  pub const CLEARAGAIN: Self = Self(fermium::SDLK_CLEARAGAIN as u32);
  pub const CRSEL: Self = Self(fermium::SDLK_CRSEL as u32);
  pub const EXSEL: Self = Self(fermium::SDLK_EXSEL as u32);
  pub const KP_00: Self = Self(fermium::SDLK_KP_00 as u32);
  pub const KP_000: Self = Self(fermium::SDLK_KP_000 as u32);
  pub const THOUSANDSSEPARATOR: Self = Self(fermium::SDLK_THOUSANDSSEPARATOR as u32);
  pub const DECIMALSEPARATOR: Self = Self(fermium::SDLK_DECIMALSEPARATOR as u32);
  pub const CURRENCYUNIT: Self = Self(fermium::SDLK_CURRENCYUNIT as u32);
  pub const CURRENCYSUBUNIT: Self = Self(fermium::SDLK_CURRENCYSUBUNIT as u32);
  pub const KP_LEFTPAREN: Self = Self(fermium::SDLK_KP_LEFTPAREN as u32);
  pub const KP_RIGHTPAREN: Self = Self(fermium::SDLK_KP_RIGHTPAREN as u32);
  pub const KP_LEFTBRACE: Self = Self(fermium::SDLK_KP_LEFTBRACE as u32);
  pub const KP_RIGHTBRACE: Self = Self(fermium::SDLK_KP_RIGHTBRACE as u32);
  pub const KP_TAB: Self = Self(fermium::SDLK_KP_TAB as u32);
  pub const KP_BACKSPACE: Self = Self(fermium::SDLK_KP_BACKSPACE as u32);
  pub const KP_A: Self = Self(fermium::SDLK_KP_A as u32);
  pub const KP_B: Self = Self(fermium::SDLK_KP_B as u32);
  pub const KP_C: Self = Self(fermium::SDLK_KP_C as u32);
  pub const KP_D: Self = Self(fermium::SDLK_KP_D as u32);
  pub const KP_E: Self = Self(fermium::SDLK_KP_E as u32);
  pub const KP_F: Self = Self(fermium::SDLK_KP_F as u32);
  pub const KP_XOR: Self = Self(fermium::SDLK_KP_XOR as u32);
  pub const KP_POWER: Self = Self(fermium::SDLK_KP_POWER as u32);
  pub const KP_PERCENT: Self = Self(fermium::SDLK_KP_PERCENT as u32);
  pub const KP_LESS: Self = Self(fermium::SDLK_KP_LESS as u32);
  pub const KP_GREATER: Self = Self(fermium::SDLK_KP_GREATER as u32);
  pub const KP_AMPERSAND: Self = Self(fermium::SDLK_KP_AMPERSAND as u32);
  pub const KP_DBLAMPERSAND: Self = Self(fermium::SDLK_KP_DBLAMPERSAND as u32);
  pub const KP_VERTICALBAR: Self = Self(fermium::SDLK_KP_VERTICALBAR as u32);
  pub const KP_DBLVERTICALBAR: Self = Self(fermium::SDLK_KP_DBLVERTICALBAR as u32);
  pub const KP_COLON: Self = Self(fermium::SDLK_KP_COLON as u32);
  pub const KP_HASH: Self = Self(fermium::SDLK_KP_HASH as u32);
  pub const KP_SPACE: Self = Self(fermium::SDLK_KP_SPACE as u32);
  pub const KP_AT: Self = Self(fermium::SDLK_KP_AT as u32);
  pub const KP_EXCLAM: Self = Self(fermium::SDLK_KP_EXCLAM as u32);
  pub const KP_MEMSTORE: Self = Self(fermium::SDLK_KP_MEMSTORE as u32);
  pub const KP_MEMRECALL: Self = Self(fermium::SDLK_KP_MEMRECALL as u32);
  pub const KP_MEMCLEAR: Self = Self(fermium::SDLK_KP_MEMCLEAR as u32);
  pub const KP_MEMADD: Self = Self(fermium::SDLK_KP_MEMADD as u32);
  pub const KP_MEMSUBTRACT: Self = Self(fermium::SDLK_KP_MEMSUBTRACT as u32);
  pub const KP_MEMMULTIPLY: Self = Self(fermium::SDLK_KP_MEMMULTIPLY as u32);
  pub const KP_MEMDIVIDE: Self = Self(fermium::SDLK_KP_MEMDIVIDE as u32);
  pub const KP_PLUSMINUS: Self = Self(fermium::SDLK_KP_PLUSMINUS as u32);
  pub const KP_CLEAR: Self = Self(fermium::SDLK_KP_CLEAR as u32);
  pub const KP_CLEARENTRY: Self = Self(fermium::SDLK_KP_CLEARENTRY as u32);
  pub const KP_BINARY: Self = Self(fermium::SDLK_KP_BINARY as u32);
  pub const KP_OCTAL: Self = Self(fermium::SDLK_KP_OCTAL as u32);
  pub const KP_DECIMAL: Self = Self(fermium::SDLK_KP_DECIMAL as u32);
  pub const KP_HEXADECIMAL: Self = Self(fermium::SDLK_KP_HEXADECIMAL as u32);
  pub const LCTRL: Self = Self(fermium::SDLK_LCTRL as u32);
  pub const LSHIFT: Self = Self(fermium::SDLK_LSHIFT as u32);
  pub const LALT: Self = Self(fermium::SDLK_LALT as u32);
  pub const LGUI: Self = Self(fermium::SDLK_LGUI as u32);
  pub const RCTRL: Self = Self(fermium::SDLK_RCTRL as u32);
  pub const RSHIFT: Self = Self(fermium::SDLK_RSHIFT as u32);
  pub const RALT: Self = Self(fermium::SDLK_RALT as u32);
  pub const RGUI: Self = Self(fermium::SDLK_RGUI as u32);
  pub const MODE: Self = Self(fermium::SDLK_MODE as u32);
  pub const AUDIONEXT: Self = Self(fermium::SDLK_AUDIONEXT as u32);
  pub const AUDIOPREV: Self = Self(fermium::SDLK_AUDIOPREV as u32);
  pub const AUDIOSTOP: Self = Self(fermium::SDLK_AUDIOSTOP as u32);
  pub const AUDIOPLAY: Self = Self(fermium::SDLK_AUDIOPLAY as u32);
  pub const AUDIOMUTE: Self = Self(fermium::SDLK_AUDIOMUTE as u32);
  pub const MEDIASELECT: Self = Self(fermium::SDLK_MEDIASELECT as u32);
  pub const WWW: Self = Self(fermium::SDLK_WWW as u32);
  pub const MAIL: Self = Self(fermium::SDLK_MAIL as u32);
  pub const CALCULATOR: Self = Self(fermium::SDLK_CALCULATOR as u32);
  pub const COMPUTER: Self = Self(fermium::SDLK_COMPUTER as u32);
  pub const AC_SEARCH: Self = Self(fermium::SDLK_AC_SEARCH as u32);
  pub const AC_HOME: Self = Self(fermium::SDLK_AC_HOME as u32);
  pub const AC_BACK: Self = Self(fermium::SDLK_AC_BACK as u32);
  pub const AC_FORWARD: Self = Self(fermium::SDLK_AC_FORWARD as u32);
  pub const AC_STOP: Self = Self(fermium::SDLK_AC_STOP as u32);
  pub const AC_REFRESH: Self = Self(fermium::SDLK_AC_REFRESH as u32);
  pub const AC_BOOKMARKS: Self = Self(fermium::SDLK_AC_BOOKMARKS as u32);
  pub const BRIGHTNESSDOWN: Self = Self(fermium::SDLK_BRIGHTNESSDOWN as u32);
  pub const BRIGHTNESSUP: Self = Self(fermium::SDLK_BRIGHTNESSUP as u32);
  pub const DISPLAYSWITCH: Self = Self(fermium::SDLK_DISPLAYSWITCH as u32);
  pub const KBDILLUMTOGGLE: Self = Self(fermium::SDLK_KBDILLUMTOGGLE as u32);
  pub const KBDILLUMDOWN: Self = Self(fermium::SDLK_KBDILLUMDOWN as u32);
  pub const KBDILLUMUP: Self = Self(fermium::SDLK_KBDILLUMUP as u32);
  pub const EJECT: Self = Self(fermium::SDLK_EJECT as u32);
  pub const SLEEP: Self = Self(fermium::SDLK_SLEEP as u32);
  pub const APP1: Self = Self(fermium::SDLK_APP1 as u32);
  pub const APP2: Self = Self(fermium::SDLK_APP2 as u32);
  pub const AUDIOREWIND: Self = Self(fermium::SDLK_AUDIOREWIND as u32);
  pub const AUDIOFASTFORWARD: Self = Self(fermium::SDLK_AUDIOFASTFORWARD as u32);
}