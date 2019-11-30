use super::*;

/// A controller button was pressed or released.
///
/// Note: On some controllers I've seen (iBuffalo SNES) you can get events for
/// buttons being released even when the button was _already_ released. Seems
/// harmless, but it might happen to you too.
#[derive(Debug, Clone, Copy)]
pub struct ControllerButtonEvent {
  /// When?
  pub timestamp: u32,
  /// Which controller?
  pub joystick_id: fermium::SDL_JoystickID,
  /// Which button?
  pub button: ControllerButton,
  /// Is it pressed now?
  pub is_pressed: bool,
}
impl From<fermium::SDL_ControllerButtonEvent> for ControllerButtonEvent {
  fn from(ev: fermium::SDL_ControllerButtonEvent) -> Self {
    Self {
      timestamp: ev.timestamp,
      joystick_id: ev.which,
      button: ControllerButton::from(fermium::SDL_GameControllerButton::from(
        ev.button,
      )),
      is_pressed: u32::from(ev.state) == fermium::SDL_PRESSED,
    }
  }
}
impl From<ControllerButtonEvent> for fermium::SDL_ControllerButtonEvent {
  fn from(ev: ControllerButtonEvent) -> Self {
    Self {
      type_: if ev.is_pressed {
        fermium::SDL_CONTROLLERBUTTONDOWN
      } else {
        fermium::SDL_CONTROLLERBUTTONUP
      } as u32,
      timestamp: ev.timestamp,
      which: ev.joystick_id,
      button: ev.button as u8,
      state: if ev.is_pressed {
        fermium::SDL_PRESSED
      } else {
        fermium::SDL_RELEASED
      } as u8,
      padding1: 0,
      padding2: 0,
    }
  }
}

/// A controller button.
///
/// Note: SDL uses the heretical XBox button naming convention, whereas I use
/// the glorious SNES button naming convention, which is inverted. As a
/// compromise, the face buttons are named after the compass directions, which
/// everyone can agree on (except the Dwarves).
///
/// Note: Not all controllers have all buttons, but SDL will do its best.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ControllerButton {
  /// All invalid button IDs get collapsed to here.
  Invalid = fermium::SDL_CONTROLLER_BUTTON_INVALID as u8,
  /// SNES B
  South = fermium::SDL_CONTROLLER_BUTTON_A as u8,
  /// SNES A
  East = fermium::SDL_CONTROLLER_BUTTON_B as u8,
  /// SNES Y
  West = fermium::SDL_CONTROLLER_BUTTON_X as u8,
  /// SNES X
  North = fermium::SDL_CONTROLLER_BUTTON_Y as u8,
  /// aka Select
  Back = fermium::SDL_CONTROLLER_BUTTON_BACK as u8,
  /// Logo / system button
  Guide = fermium::SDL_CONTROLLER_BUTTON_GUIDE as u8,
  /// Start / Pause / etc
  Start = fermium::SDL_CONTROLLER_BUTTON_START as u8,
  /// Left stick clicking in, PlayStation "L3"
  LeftStick = fermium::SDL_CONTROLLER_BUTTON_LEFTSTICK as u8,
  /// Right stick click, PlayStation "R3"
  RightStick = fermium::SDL_CONTROLLER_BUTTON_RIGHTSTICK as u8,
  /// Upper left shoulder button, L1 / LB
  LeftShoulder = fermium::SDL_CONTROLLER_BUTTON_LEFTSHOULDER as u8,
  /// Upper right shoulder button, R1 / RB
  RightShoulder = fermium::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER as u8,
  /// Plus Pad Up
  Up = fermium::SDL_CONTROLLER_BUTTON_DPAD_UP as u8,
  /// Plus Pad Down
  Down = fermium::SDL_CONTROLLER_BUTTON_DPAD_DOWN as u8,
  /// Plus Pad Left
  Left = fermium::SDL_CONTROLLER_BUTTON_DPAD_LEFT as u8,
  /// Plus Pad Right
  Right = fermium::SDL_CONTROLLER_BUTTON_DPAD_RIGHT as u8,
}
impl From<fermium::SDL_GameControllerButton> for ControllerButton {
  fn from(gcb: fermium::SDL_GameControllerButton) -> Self {
    match gcb {
      fermium::SDL_CONTROLLER_BUTTON_A => ControllerButton::South,
      fermium::SDL_CONTROLLER_BUTTON_B => ControllerButton::East,
      fermium::SDL_CONTROLLER_BUTTON_X => ControllerButton::West,
      fermium::SDL_CONTROLLER_BUTTON_Y => ControllerButton::North,
      fermium::SDL_CONTROLLER_BUTTON_BACK => ControllerButton::Back,
      fermium::SDL_CONTROLLER_BUTTON_GUIDE => ControllerButton::Guide,
      fermium::SDL_CONTROLLER_BUTTON_START => ControllerButton::Start,
      fermium::SDL_CONTROLLER_BUTTON_LEFTSTICK => ControllerButton::LeftStick,
      fermium::SDL_CONTROLLER_BUTTON_RIGHTSTICK => ControllerButton::RightStick,
      fermium::SDL_CONTROLLER_BUTTON_LEFTSHOULDER => {
        ControllerButton::LeftShoulder
      }
      fermium::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER => {
        ControllerButton::RightShoulder
      }
      fermium::SDL_CONTROLLER_BUTTON_DPAD_UP => ControllerButton::Up,
      fermium::SDL_CONTROLLER_BUTTON_DPAD_DOWN => ControllerButton::Down,
      fermium::SDL_CONTROLLER_BUTTON_DPAD_LEFT => ControllerButton::Left,
      fermium::SDL_CONTROLLER_BUTTON_DPAD_RIGHT => ControllerButton::Right,
      _ => ControllerButton::Invalid,
    }
  }
}
