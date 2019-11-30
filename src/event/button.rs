use super::*;

/// A mouse button was pressed or released.
#[derive(Debug, Clone, Copy)]
pub struct MouseButtonEvent {
  /// When
  pub timestamp: u32,
  /// Which window was involved?
  pub window_id: u32,
  /// Which mouse was involved?
  pub mouse_id: u32,
  /// The button ID.
  pub button: MouseButton,
  /// The button was pressed.
  pub is_pressed: bool,
  /// The number of repeated clicks.
  pub clicks: u8,
  /// The mouse's x position (relative to the window).
  pub x_pos: i32,
  /// The mouse's y position (relative to the window).
  pub y_pos: i32,
}
impl From<fermium::SDL_MouseButtonEvent> for MouseButtonEvent {
  fn from(ev: fermium::SDL_MouseButtonEvent) -> Self {
    Self {
      timestamp: ev.timestamp,
      window_id: ev.windowID,
      mouse_id: ev.which,
      button: MouseButton(u32::from(ev.button)),
      is_pressed: u32::from(ev.state) == fermium::SDL_PRESSED,
      clicks: ev.clicks,
      x_pos: ev.x,
      y_pos: ev.y,
    }
  }
}
impl From<MouseButtonEvent> for fermium::SDL_MouseButtonEvent {
  fn from(ev: MouseButtonEvent) -> Self {
    Self {
      type_: (if ev.is_pressed {
        fermium::SDL_MOUSEBUTTONDOWN
      } else {
        fermium::SDL_MOUSEBUTTONUP
      }) as u32,
      timestamp: ev.timestamp,
      windowID: ev.window_id,
      which: ev.mouse_id,
      button: ev.button.0 as u8,
      state: (if ev.is_pressed {
        fermium::SDL_PRESSED
      } else {
        fermium::SDL_RELEASED
      }) as u8,
      clicks: ev.clicks,
      padding1: 0,
      x: ev.x_pos,
      y: ev.y_pos,
    }
  }
}

// Note: Fermium is accidentally incomplete, so we have to define this ourselves.
const fn button(input: u32) -> u32 {
  1 << (input - 1)
}

/// A bit bag of mouse buttons being pressed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseButton(pub(crate) u32);
#[allow(non_upper_case_globals)]
impl MouseButton {
  /// Left button.
  pub const Left: MouseButton = MouseButton(button(1));
  /// Middle button (mouse wheel).
  pub const Middle: MouseButton = MouseButton(button(2));
  /// Right button.
  pub const Right: MouseButton = MouseButton(button(3));
  /// Extra button 1
  pub const X1: MouseButton = MouseButton(button(4));
  /// Extra button 2
  pub const X2: MouseButton = MouseButton(button(5));
}
impl MouseButton {
  /// Does the button value on the left contain the button value on the right?
  pub fn has_all(self, buttons: MouseButton) -> bool {
    (self.0 & buttons.0) == buttons.0
  }
}
