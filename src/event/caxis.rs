use super::*;

#[derive(Debug, Clone, Copy)]
pub struct ControllerAxisEvent {
  timestamp: u32,
  joystick_id: fermium::SDL_JoystickID,
  axis: ControllerAxis,
  value: i16,
}
impl From<fermium::SDL_ControllerAxisEvent> for ControllerAxisEvent {
  fn from(ev: fermium::SDL_ControllerAxisEvent) -> Self {
    Self {
      timestamp: ev.timestamp,
      joystick_id: ev.which,
      axis: ControllerAxis::from(fermium::SDL_GameControllerAxis::from(ev.axis)),
      value: ev.value,
    }
  }
}
impl From<ControllerAxisEvent> for fermium::SDL_ControllerAxisEvent {
  fn from(ev: ControllerAxisEvent) -> Self {
    Self {
      type_: fermium::SDL_CONTROLLERAXISMOTION as u32,
      timestamp: ev.timestamp,
      which: ev.joystick_id,
      axis: ev.axis as u8,
      value: ev.value,
      padding1: 0,
      padding2: 0,
      padding3: 0,
      padding4: 0,
    }
  }
}

/// An axis of input for a game controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ControllerAxis {
  /// Invalid axis values get mapped to here.
  Invalid = fermium::SDL_CONTROLLER_AXIS_INVALID as u8,
  /// Left stick X (horizontal)
  LeftX = fermium::SDL_CONTROLLER_AXIS_LEFTX as u8,
  /// Left stick Y (vertical)
  LeftY = fermium::SDL_CONTROLLER_AXIS_LEFTY as u8,
  /// Right stick X (horizontal)
  RightX = fermium::SDL_CONTROLLER_AXIS_RIGHTX as u8,
  /// Right stick Y (vertical)
  RightY = fermium::SDL_CONTROLLER_AXIS_RIGHTY as u8,
  /// Left trigger, LT / L2
  TriggerLeft = fermium::SDL_CONTROLLER_AXIS_TRIGGERLEFT as u8,
  /// Right trigger, RT / R2
  TriggerRight = fermium::SDL_CONTROLLER_AXIS_TRIGGERRIGHT as u8,
}

impl From<fermium::SDL_GameControllerAxis> for ControllerAxis {
  fn from(gca: fermium::SDL_GameControllerAxis) -> Self {
    match gca {
      fermium::SDL_CONTROLLER_AXIS_LEFTX => ControllerAxis::LeftX,
      fermium::SDL_CONTROLLER_AXIS_LEFTY => ControllerAxis::LeftY,
      fermium::SDL_CONTROLLER_AXIS_RIGHTX => ControllerAxis::RightX,
      fermium::SDL_CONTROLLER_AXIS_RIGHTY => ControllerAxis::RightY,
      fermium::SDL_CONTROLLER_AXIS_TRIGGERLEFT => ControllerAxis::TriggerLeft,
      fermium::SDL_CONTROLLER_AXIS_TRIGGERRIGHT => ControllerAxis::TriggerRight,
      _ => ControllerAxis::Invalid,
    }
  }
}