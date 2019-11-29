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
  pub modifiers: u16,
}

impl From<fermium::SDL_Keysym> for KeyInfo {
  fn from(key: fermium::SDL_Keysym) -> Self {
    Self { scancode: key.scancode, keycode: key.sym, modifiers: key.mod_ }
  }
}
impl From<KeyInfo> for fermium::SDL_Keysym {
  fn from(info: KeyInfo) -> Self {
    Self {
      scancode: info.scancode,
      sym: info.keycode,
      mod_: info.modifiers,
      unused: 0,
    }
  }
}
