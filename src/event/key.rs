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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyInfo {
  /// Physical scancode.
  pub scancode: fermium::SDL_Scancode,
  /// Virtual keycode.
  pub keycode: fermium::SDL_Keycode,
  /// The modifiers
  pub modifiers: KeyModifiers,
}

impl From<fermium::SDL_Keysym> for KeyInfo {
  fn from(key: fermium::SDL_Keysym) -> Self {
    Self {
      scancode: key.scancode,
      keycode: key.sym,
      modifiers: KeyModifiers(key.mod_),
    }
  }
}
impl From<KeyInfo> for fermium::SDL_Keysym {
  fn from(info: KeyInfo) -> Self {
    Self {
      scancode: info.scancode,
      sym: info.keycode,
      mod_: info.modifiers.0,
      unused: 0,
    }
  }
}

/// A bit bag of modifier keys being held.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
