use fermium::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ControllerAxis {
  Invalid,
  LeftX,
  LeftY,
  RightX,
  RightY,
  TriggerLeft,
  TriggerRight,
}
impl From<u8> for ControllerAxis {
  #[inline]
  fn from(value: u8) -> Self {
    match SDL_GameControllerAxis(i32::from(value)) {
      SDL_CONTROLLER_AXIS_LEFTX => ControllerAxis::LeftX,
      SDL_CONTROLLER_AXIS_LEFTY => ControllerAxis::LeftY,
      SDL_CONTROLLER_AXIS_RIGHTX => ControllerAxis::RightX,
      SDL_CONTROLLER_AXIS_RIGHTY => ControllerAxis::RightY,
      SDL_CONTROLLER_AXIS_TRIGGERLEFT => ControllerAxis::TriggerLeft,
      SDL_CONTROLLER_AXIS_TRIGGERRIGHT => ControllerAxis::TriggerRight,
      _ => ControllerAxis::Invalid,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ControllerButton {
  Invalid,
  North,
  South,
  East,
  West,
  Back,
  Guide,
  Start,
  LeftStick,
  RightStick,
  LeftShoulder,
  RightShoulder,
  DpadUp,
  DpadDown,
  DpadLeft,
  DpadRight,
  Misc1,
  Paddle1,
  Paddle2,
  Paddle3,
  Paddle4,
  Touchpad,
}
impl From<u8> for ControllerButton {
  #[inline]
  fn from(value: u8) -> Self {
    match SDL_GameControllerButton(i32::from(value)) {
      SDL_CONTROLLER_BUTTON_A => ControllerButton::South,
      SDL_CONTROLLER_BUTTON_B => ControllerButton::East,
      SDL_CONTROLLER_BUTTON_X => ControllerButton::West,
      SDL_CONTROLLER_BUTTON_Y => ControllerButton::North,
      SDL_CONTROLLER_BUTTON_BACK => ControllerButton::Back,
      SDL_CONTROLLER_BUTTON_GUIDE => ControllerButton::Guide,
      SDL_CONTROLLER_BUTTON_START => ControllerButton::Start,
      SDL_CONTROLLER_BUTTON_LEFTSTICK => ControllerButton::LeftStick,
      SDL_CONTROLLER_BUTTON_RIGHTSTICK => ControllerButton::RightStick,
      SDL_CONTROLLER_BUTTON_LEFTSHOULDER => ControllerButton::LeftShoulder,
      SDL_CONTROLLER_BUTTON_RIGHTSHOULDER => ControllerButton::RightShoulder,
      SDL_CONTROLLER_BUTTON_DPAD_UP => ControllerButton::DpadUp,
      SDL_CONTROLLER_BUTTON_DPAD_DOWN => ControllerButton::DpadDown,
      SDL_CONTROLLER_BUTTON_DPAD_LEFT => ControllerButton::DpadLeft,
      SDL_CONTROLLER_BUTTON_DPAD_RIGHT => ControllerButton::DpadRight,
      SDL_CONTROLLER_BUTTON_MISC1 => ControllerButton::Misc1,
      SDL_CONTROLLER_BUTTON_PADDLE1 => ControllerButton::Paddle1,
      SDL_CONTROLLER_BUTTON_PADDLE2 => ControllerButton::Paddle2,
      SDL_CONTROLLER_BUTTON_PADDLE3 => ControllerButton::Paddle3,
      SDL_CONTROLLER_BUTTON_PADDLE4 => ControllerButton::Paddle4,
      SDL_CONTROLLER_BUTTON_TOUCHPAD => ControllerButton::Touchpad,
      _ => ControllerButton::Invalid,
    }
  }
}
