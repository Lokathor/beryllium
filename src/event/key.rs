use super::*;

#[derive(Debug, Clone, Copy)]
pub struct KeyboardEvent {
  timestamp: u32,
  window_id: u32,
  is_pressed: bool,
  repeat: u8,
  key: KeyInfo,
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

#[derive(Debug, Clone, Copy)]
pub struct KeyInfo {
  scancode: fermium::SDL_Scancode,
  keycode: fermium::SDL_Keycode,
  modifiers: u16,
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
