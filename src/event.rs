use core::convert::{TryFrom, TryInto};

use fermium::{
  SDL_Event, SDL_EventType, SDL_CONTROLLERAXISMOTION, SDL_CONTROLLERBUTTONDOWN,
  SDL_CONTROLLERBUTTONUP, SDL_CONTROLLERDEVICEADDED,
  SDL_CONTROLLERDEVICEREMAPPED, SDL_CONTROLLERDEVICEREMOVED, SDL_JOYAXISMOTION,
  SDL_JOYBALLMOTION, SDL_JOYBUTTONDOWN, SDL_JOYBUTTONUP, SDL_JOYDEVICEADDED,
  SDL_JOYDEVICEREMOVED, SDL_JOYHATMOTION, SDL_KEYDOWN, SDL_KEYUP,
  SDL_MOUSEBUTTONDOWN, SDL_MOUSEBUTTONUP, SDL_MOUSEMOTION, SDL_MOUSEWHEEL,
  SDL_QUIT, SDL_WINDOWEVENT,
};

use crate::{JoystickID, MouseButtonState, MouseID, WindowID};

#[non_exhaustive]
pub enum Event {
  Quit,
  // TODO: DisplayEvent
  Window(WindowEvent),
  Keyboard(KeyboardEvent),
  /* TODO: TextEditing,
   * TODO: TextInput, */
  MouseMotion(MouseMotionEvent),
  MouseButton(MouseButtonEvent),
  MouseWheel(MouseWheelEvent),
  JoyAxis(JoyAxisEvent),
  JoyBall(JoyBallEvent),
  JoyHat(JoyHatEvent),
  JoyButton(JoyButtonEvent),
  JoyDevice(JoyDeviceEvent),
  ControllerAxis(ControllerAxisEvent),
  ControllerButton(ControllerButtonEvent),
  ControllerDevice(ControllerDeviceEvent),
}

impl TryFrom<SDL_Event> for Event {
  type Error = ();
  #[inline]
  #[must_use]
  fn try_from(sdl_event: SDL_Event) -> Result<Self, Self::Error> {
    // Safety: `sdl_event` is a union so there's all sorts of union access here
    unsafe {
      Ok(match sdl_event.type_ as SDL_EventType {
        SDL_QUIT => Event::Quit,
        SDL_WINDOWEVENT => Event::Window(sdl_event.window.try_into()?),
        SDL_KEYDOWN | SDL_KEYUP => Event::Keyboard(sdl_event.key.into()),
        SDL_MOUSEMOTION => Event::MouseMotion(sdl_event.motion.into()),
        SDL_MOUSEBUTTONDOWN | SDL_MOUSEBUTTONUP => {
          Event::MouseButton(sdl_event.button.into())
        }
        SDL_MOUSEWHEEL => Event::MouseWheel(sdl_event.wheel.into()),
        SDL_JOYAXISMOTION => Event::JoyAxis(sdl_event.jaxis.into()),
        SDL_JOYBALLMOTION => Event::JoyBall(sdl_event.jball.into()),
        SDL_JOYHATMOTION => Event::JoyHat(sdl_event.jhat.try_into()?),
        SDL_JOYBUTTONDOWN | SDL_JOYBUTTONUP => {
          Event::JoyButton(sdl_event.jbutton.into())
        }
        SDL_JOYDEVICEADDED | SDL_JOYDEVICEREMOVED => {
          Event::JoyDevice(sdl_event.jdevice.try_into()?)
        }
        SDL_CONTROLLERAXISMOTION => {
          Event::ControllerAxis(sdl_event.caxis.into())
        }
        SDL_CONTROLLERBUTTONDOWN | SDL_CONTROLLERBUTTONUP => {
          Event::ControllerButton(sdl_event.cbutton.into())
        }
        SDL_CONTROLLERDEVICEADDED
        | SDL_CONTROLLERDEVICEREMOVED
        | SDL_CONTROLLERDEVICEREMAPPED => {
          Event::ControllerDevice(sdl_event.cdevice.try_into()?)
        }
        _ => return Err(()),
      })
    }
  }
}

pub use window_event::*;
mod window_event {
  use super::*;
  use fermium::{
    SDL_WindowEvent, SDL_WindowEventID, SDL_WINDOWEVENT_CLOSE,
    SDL_WINDOWEVENT_ENTER, SDL_WINDOWEVENT_EXPOSED,
    SDL_WINDOWEVENT_FOCUS_GAINED, SDL_WINDOWEVENT_FOCUS_LOST,
    SDL_WINDOWEVENT_HIDDEN, SDL_WINDOWEVENT_HIT_TEST, SDL_WINDOWEVENT_LEAVE,
    SDL_WINDOWEVENT_MAXIMIZED, SDL_WINDOWEVENT_MINIMIZED,
    SDL_WINDOWEVENT_MOVED, SDL_WINDOWEVENT_RESIZED, SDL_WINDOWEVENT_RESTORED,
    SDL_WINDOWEVENT_SHOWN, SDL_WINDOWEVENT_SIZE_CHANGED,
    SDL_WINDOWEVENT_TAKE_FOCUS,
  };

  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  #[non_exhaustive]
  pub enum WindowEvent {
    Shown { window_id: WindowID },
    Hidden { window_id: WindowID },
    Exposed { window_id: WindowID },
    Maximized { window_id: WindowID },
    Minimized { window_id: WindowID },
    Restored { window_id: WindowID },
    MouseEntered { window_id: WindowID },
    MouseLeft { window_id: WindowID },
    FocusGained { window_id: WindowID },
    FocusLost { window_id: WindowID },
    Close { window_id: WindowID },
    TakeFocus { window_id: WindowID },
    HitTest { window_id: WindowID },
    Moved { window_id: WindowID, x: i32, y: i32 },
    Resized { window_id: WindowID, width: u32, height: u32 },
    SizeChanged { window_id: WindowID, width: u32, height: u32 },
  }

  impl TryFrom<SDL_WindowEvent> for WindowEvent {
    type Error = ();
    #[inline]
    #[must_use]
    fn try_from(window_event: SDL_WindowEvent) -> Result<Self, Self::Error> {
      let window_id = WindowID(window_event.windowID);
      Ok(match window_event.event as SDL_WindowEventID {
        SDL_WINDOWEVENT_SHOWN => Self::Shown { window_id },
        SDL_WINDOWEVENT_HIDDEN => Self::Hidden { window_id },
        SDL_WINDOWEVENT_EXPOSED => Self::Exposed { window_id },
        SDL_WINDOWEVENT_MAXIMIZED => Self::Maximized { window_id },
        SDL_WINDOWEVENT_MINIMIZED => Self::Minimized { window_id },
        SDL_WINDOWEVENT_RESTORED => Self::Restored { window_id },
        SDL_WINDOWEVENT_ENTER => Self::MouseEntered { window_id },
        SDL_WINDOWEVENT_LEAVE => Self::MouseLeft { window_id },
        SDL_WINDOWEVENT_FOCUS_GAINED => Self::FocusGained { window_id },
        SDL_WINDOWEVENT_FOCUS_LOST => Self::FocusLost { window_id },
        SDL_WINDOWEVENT_CLOSE => Self::Close { window_id },
        SDL_WINDOWEVENT_TAKE_FOCUS => Self::TakeFocus { window_id },
        SDL_WINDOWEVENT_HIT_TEST => Self::HitTest { window_id },
        SDL_WINDOWEVENT_MOVED => {
          let x = window_event.data1 as i32;
          let y = window_event.data2 as i32;
          Self::Moved { window_id, x, y }
        }
        SDL_WINDOWEVENT_RESIZED => {
          let width = window_event.data1 as u32;
          let height = window_event.data2 as u32;
          Self::Resized { window_id, width, height }
        }
        SDL_WINDOWEVENT_SIZE_CHANGED => {
          let width = window_event.data1 as u32;
          let height = window_event.data2 as u32;
          Self::SizeChanged { window_id, width, height }
        }
        _ => return Err(()),
      })
    }
  }
}

pub use keyboard_event::*;
mod keyboard_event {
  use super::*;
  use fermium::{SDL_KeyboardEvent, SDL_Keysym, SDL_PRESSED};
  //
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  #[repr(transparent)]
  pub struct Scancode(u32);
  //
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  #[repr(transparent)]
  pub struct Keycode(u32);
  //
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  #[repr(transparent)]
  pub struct KeyModifiers(u16);
  //
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct KeyboardEvent {
    pub window_id: WindowID,
    pub scancode: Scancode,
    pub keycode: Keycode,
    pub modifiers: KeyModifiers,
    pub is_pressed: bool,
    pub repeat: u8,
  }
  impl From<SDL_KeyboardEvent> for KeyboardEvent {
    #[inline]
    #[must_use]
    fn from(keyboard_event: SDL_KeyboardEvent) -> Self {
      Self {
        window_id: WindowID(keyboard_event.windowID),
        scancode: Scancode(keyboard_event.keysym.scancode as u32),
        keycode: Keycode(keyboard_event.keysym.sym as u32),
        modifiers: KeyModifiers(keyboard_event.keysym.mod_),
        is_pressed: keyboard_event.state as u32 == SDL_PRESSED,
        repeat: keyboard_event.repeat,
      }
    }
  }
  // TODO: Key constants
}

pub use mouse_motion::*;
mod mouse_motion {
  use super::*;
  use fermium::SDL_MouseMotionEvent;
  //
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct MouseMotionEvent {
    pub window_id: WindowID,
    pub mouse_id: MouseID,
    pub button_state: MouseButtonState,
    pub x_pos: i32,
    pub y_pos: i32,
    pub dx: i32,
    pub dy: i32,
  }
  impl From<SDL_MouseMotionEvent> for MouseMotionEvent {
    #[inline]
    #[must_use]
    fn from(mouse_motion_event: SDL_MouseMotionEvent) -> Self {
      Self {
        window_id: WindowID(mouse_motion_event.windowID),
        mouse_id: MouseID(mouse_motion_event.which),
        button_state: MouseButtonState(mouse_motion_event.state),
        x_pos: mouse_motion_event.x,
        y_pos: mouse_motion_event.y,
        dx: mouse_motion_event.xrel,
        dy: mouse_motion_event.yrel,
      }
    }
  }
}

pub use mouse_button::*;
mod mouse_button {
  use super::*;
  use fermium::{SDL_MouseButtonEvent, SDL_PRESSED};
  //
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct MouseButtonEvent {
    pub window_id: WindowID,
    pub mouse_id: MouseID,
    pub button: MouseButtonState,
    pub is_pressed: bool,
    pub clicks: u8,
    pub x_pos: i32,
    pub y_pos: i32,
  }
  impl From<SDL_MouseButtonEvent> for MouseButtonEvent {
    #[inline]
    #[must_use]
    fn from(mouse_button_event: SDL_MouseButtonEvent) -> Self {
      Self {
        window_id: WindowID(mouse_button_event.windowID),
        mouse_id: MouseID(mouse_button_event.which),
        button: MouseButtonState(mouse_button_event.button as u32),
        is_pressed: mouse_button_event.state as u32 == SDL_PRESSED,
        clicks: mouse_button_event.clicks,
        x_pos: mouse_button_event.x,
        y_pos: mouse_button_event.y,
      }
    }
  }
}

pub use mouse_wheel::*;
mod mouse_wheel {
  use super::*;
  use fermium::{SDL_MouseWheelEvent, SDL_MOUSEWHEEL_FLIPPED};
  //
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct MouseWheelEvent {
    pub window_id: WindowID,
    pub mouse_id: MouseID,
    pub dx: i32,
    pub dy: i32,
  }
  impl From<SDL_MouseWheelEvent> for MouseWheelEvent {
    #[inline]
    #[must_use]
    fn from(mouse_wheel_event: SDL_MouseWheelEvent) -> Self {
      let mut out = Self {
        window_id: WindowID(mouse_wheel_event.windowID),
        mouse_id: MouseID(mouse_wheel_event.which),
        dx: mouse_wheel_event.x,
        dy: mouse_wheel_event.y,
      };
      if mouse_wheel_event.direction == SDL_MOUSEWHEEL_FLIPPED as u32 {
        out.dx = -out.dx;
        out.dy = -out.dy;
      }
      out
    }
  }
}

pub use joy_axis::*;
mod joy_axis {
  use super::*;
  use fermium::SDL_JoyAxisEvent;
  //
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct JoyAxisEvent {
    pub joystick_id: JoystickID,
    pub axis: u8,
    pub value: i16,
  }
  impl From<SDL_JoyAxisEvent> for JoyAxisEvent {
    #[inline]
    #[must_use]
    fn from(joy_axis_event: SDL_JoyAxisEvent) -> Self {
      Self {
        joystick_id: JoystickID(joy_axis_event.which),
        axis: joy_axis_event.axis,
        value: joy_axis_event.value,
      }
    }
  }
}

pub use joy_ball::*;
mod joy_ball {
  use super::*;
  use fermium::SDL_JoyBallEvent;
  //
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct JoyBallEvent {
    pub joystick_id: JoystickID,
    pub ball: u8,
    pub dx: i16,
    pub dy: i16,
  }
  impl From<SDL_JoyBallEvent> for JoyBallEvent {
    #[inline]
    #[must_use]
    fn from(joy_ball_event: SDL_JoyBallEvent) -> Self {
      Self {
        joystick_id: JoystickID(joy_ball_event.which),
        ball: joy_ball_event.ball,
        dx: joy_ball_event.xrel,
        dy: joy_ball_event.yrel,
      }
    }
  }
}

pub use joy_hat::*;
mod joy_hat {
  use super::*;
  use fermium::{
    SDL_JoyHatEvent, SDL_HAT_CENTERED, SDL_HAT_DOWN, SDL_HAT_LEFT,
    SDL_HAT_LEFTDOWN, SDL_HAT_LEFTUP, SDL_HAT_RIGHT, SDL_HAT_RIGHTDOWN,
    SDL_HAT_RIGHTUP, SDL_HAT_UP,
  };
  //
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub enum HatValue {
    Centered = SDL_HAT_CENTERED as _,
    LeftUp = SDL_HAT_LEFTUP as _,
    Up = SDL_HAT_UP as _,
    RightUp = SDL_HAT_RIGHTUP as _,
    Left = SDL_HAT_LEFT as _,
    Right = SDL_HAT_RIGHT as _,
    LeftDown = SDL_HAT_LEFTDOWN as _,
    Down = SDL_HAT_DOWN as _,
    RightDown = SDL_HAT_RIGHTDOWN as _,
  }
  impl TryFrom<u8> for HatValue {
    type Error = ();
    #[inline]
    #[must_use]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
      Ok(match value as u32 {
        SDL_HAT_CENTERED => Self::Centered,
        SDL_HAT_LEFTUP => Self::LeftUp,
        SDL_HAT_UP => Self::Up,
        SDL_HAT_RIGHTUP => Self::RightUp,
        SDL_HAT_LEFT => Self::Left,
        SDL_HAT_RIGHT => Self::Right,
        SDL_HAT_LEFTDOWN => Self::LeftDown,
        SDL_HAT_DOWN => Self::Down,
        SDL_HAT_RIGHTDOWN => Self::RightDown,
        _ => return Err(()),
      })
    }
  }
  //
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct JoyHatEvent {
    pub joystick_id: JoystickID,
    pub hat: u8,
    pub value: HatValue,
  }
  impl TryFrom<SDL_JoyHatEvent> for JoyHatEvent {
    type Error = ();
    #[inline]
    #[must_use]
    fn try_from(joy_hat_event: SDL_JoyHatEvent) -> Result<Self, Self::Error> {
      Ok(Self {
        joystick_id: JoystickID(joy_hat_event.which),
        hat: joy_hat_event.hat,
        value: joy_hat_event.value.try_into()?,
      })
    }
  }
}

pub use joy_button::*;
mod joy_button {
  use super::*;
  use fermium::{SDL_JoyButtonEvent, SDL_PRESSED};
  //
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct JoyButtonEvent {
    pub joystick_id: JoystickID,
    pub button: u8,
    pub is_pressed: bool,
  }
  impl From<SDL_JoyButtonEvent> for JoyButtonEvent {
    #[inline]
    #[must_use]
    fn from(joy_button_event: SDL_JoyButtonEvent) -> Self {
      Self {
        joystick_id: JoystickID(joy_button_event.which),
        button: joy_button_event.button,
        is_pressed: joy_button_event.state as u32 == SDL_PRESSED,
      }
    }
  }
}

pub use joy_device::*;
mod joy_device {
  use super::*;
  use fermium::SDL_JoyDeviceEvent;

  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  #[non_exhaustive]
  pub enum JoyDeviceEvent {
    Added { device_index: i32 },
    Removed { joystick_id: JoystickID },
  }

  impl TryFrom<SDL_JoyDeviceEvent> for JoyDeviceEvent {
    type Error = ();
    #[inline]
    #[must_use]
    fn try_from(
      joy_device_event: SDL_JoyDeviceEvent,
    ) -> Result<Self, Self::Error> {
      Ok(match joy_device_event.type_ as SDL_EventType {
        SDL_JOYDEVICEADDED => {
          Self::Added { device_index: joy_device_event.which }
        }
        SDL_JOYDEVICEREMOVED => {
          Self::Removed { joystick_id: JoystickID(joy_device_event.which) }
        }
        _ => return Err(()),
      })
    }
  }
}

pub use controller_axis::*;
mod controller_axis {
  use super::*;
  use fermium::{
    SDL_ControllerAxisEvent, SDL_CONTROLLER_AXIS_INVALID,
    SDL_CONTROLLER_AXIS_LEFTX, SDL_CONTROLLER_AXIS_LEFTY,
    SDL_CONTROLLER_AXIS_RIGHTX, SDL_CONTROLLER_AXIS_RIGHTY,
    SDL_CONTROLLER_AXIS_TRIGGERLEFT, SDL_CONTROLLER_AXIS_TRIGGERRIGHT,
  };

  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub enum ControllerAxis {
    Invalid = SDL_CONTROLLER_AXIS_INVALID as _,
    LeftX = SDL_CONTROLLER_AXIS_LEFTX as _,
    LeftY = SDL_CONTROLLER_AXIS_LEFTY as _,
    RightX = SDL_CONTROLLER_AXIS_RIGHTX as _,
    RightY = SDL_CONTROLLER_AXIS_RIGHTY as _,
    LeftTrigger = SDL_CONTROLLER_AXIS_TRIGGERLEFT as _,
    RightTrigger = SDL_CONTROLLER_AXIS_TRIGGERRIGHT as _,
  }
  impl From<u8> for ControllerAxis {
    #[inline]
    #[must_use]
    fn from(axis: u8) -> Self {
      match axis as i32 {
        SDL_CONTROLLER_AXIS_LEFTX => Self::LeftX,
        SDL_CONTROLLER_AXIS_LEFTY => Self::LeftY,
        SDL_CONTROLLER_AXIS_RIGHTX => Self::RightX,
        SDL_CONTROLLER_AXIS_RIGHTY => Self::RightY,
        SDL_CONTROLLER_AXIS_TRIGGERLEFT => Self::LeftTrigger,
        SDL_CONTROLLER_AXIS_TRIGGERRIGHT => Self::RightTrigger,
        _ => Self::Invalid,
      }
    }
  }

  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct ControllerAxisEvent {
    joystick_id: JoystickID,
    axis: ControllerAxis,
    value: i16,
  }

  impl From<SDL_ControllerAxisEvent> for ControllerAxisEvent {
    #[inline]
    #[must_use]
    fn from(controller_axis_event: SDL_ControllerAxisEvent) -> Self {
      Self {
        joystick_id: JoystickID(controller_axis_event.which),
        axis: controller_axis_event.axis.into(),
        value: controller_axis_event.value,
      }
    }
  }
}

pub use controller_button::*;
mod controller_button {
  use super::*;
  use fermium::{
    SDL_ControllerButtonEvent, SDL_CONTROLLER_BUTTON_A,
    SDL_CONTROLLER_BUTTON_B, SDL_CONTROLLER_BUTTON_BACK,
    SDL_CONTROLLER_BUTTON_DPAD_DOWN, SDL_CONTROLLER_BUTTON_DPAD_LEFT,
    SDL_CONTROLLER_BUTTON_DPAD_RIGHT, SDL_CONTROLLER_BUTTON_DPAD_UP,
    SDL_CONTROLLER_BUTTON_GUIDE, SDL_CONTROLLER_BUTTON_INVALID,
    SDL_CONTROLLER_BUTTON_LEFTSHOULDER, SDL_CONTROLLER_BUTTON_LEFTSTICK,
    SDL_CONTROLLER_BUTTON_RIGHTSHOULDER, SDL_CONTROLLER_BUTTON_RIGHTSTICK,
    SDL_CONTROLLER_BUTTON_START, SDL_CONTROLLER_BUTTON_X,
    SDL_CONTROLLER_BUTTON_Y, SDL_PRESSED,
  };

  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub enum ControllerButton {
    Invalid = SDL_CONTROLLER_BUTTON_INVALID as _,
    North = SDL_CONTROLLER_BUTTON_Y as _,
    South = SDL_CONTROLLER_BUTTON_A as _,
    East = SDL_CONTROLLER_BUTTON_B as _,
    West = SDL_CONTROLLER_BUTTON_X as _,
    Back = SDL_CONTROLLER_BUTTON_BACK as _,
    Guide = SDL_CONTROLLER_BUTTON_GUIDE as _,
    Start = SDL_CONTROLLER_BUTTON_START as _,
    LeftStick = SDL_CONTROLLER_BUTTON_LEFTSTICK as _,
    RightStick = SDL_CONTROLLER_BUTTON_RIGHTSTICK as _,
    LeftShoulder = SDL_CONTROLLER_BUTTON_LEFTSHOULDER as _,
    RightShoulder = SDL_CONTROLLER_BUTTON_RIGHTSHOULDER as _,
    Up = SDL_CONTROLLER_BUTTON_DPAD_UP as _,
    Down = SDL_CONTROLLER_BUTTON_DPAD_DOWN as _,
    Left = SDL_CONTROLLER_BUTTON_DPAD_LEFT as _,
    Right = SDL_CONTROLLER_BUTTON_DPAD_RIGHT as _,
  }
  impl From<u8> for ControllerButton {
    #[inline]
    #[must_use]
    fn from(axis: u8) -> Self {
      match axis as i32 {
        SDL_CONTROLLER_BUTTON_Y => Self::North,
        SDL_CONTROLLER_BUTTON_A => Self::South,
        SDL_CONTROLLER_BUTTON_B => Self::East,
        SDL_CONTROLLER_BUTTON_X => Self::West,
        SDL_CONTROLLER_BUTTON_BACK => Self::Back,
        SDL_CONTROLLER_BUTTON_GUIDE => Self::Guide,
        SDL_CONTROLLER_BUTTON_START => Self::Start,
        SDL_CONTROLLER_BUTTON_LEFTSTICK => Self::LeftStick,
        SDL_CONTROLLER_BUTTON_RIGHTSTICK => Self::RightStick,
        SDL_CONTROLLER_BUTTON_LEFTSHOULDER => Self::LeftShoulder,
        SDL_CONTROLLER_BUTTON_RIGHTSHOULDER => Self::RightShoulder,
        SDL_CONTROLLER_BUTTON_DPAD_UP => Self::Up,
        SDL_CONTROLLER_BUTTON_DPAD_DOWN => Self::Down,
        SDL_CONTROLLER_BUTTON_DPAD_LEFT => Self::Left,
        SDL_CONTROLLER_BUTTON_DPAD_RIGHT => Self::Right,
        _ => Self::Invalid,
      }
    }
  }

  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct ControllerButtonEvent {
    joystick_id: JoystickID,
    button: ControllerButton,
    is_pressed: bool,
  }

  impl From<SDL_ControllerButtonEvent> for ControllerButtonEvent {
    #[inline]
    #[must_use]
    fn from(controller_button_event: SDL_ControllerButtonEvent) -> Self {
      Self {
        joystick_id: JoystickID(controller_button_event.which),
        button: controller_button_event.button.into(),
        is_pressed: controller_button_event.state as u32 == SDL_PRESSED,
      }
    }
  }
}

pub use controller_device::*;
mod controller_device {
  use super::*;
  use fermium::SDL_ControllerDeviceEvent;

  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  #[non_exhaustive]
  pub enum ControllerDeviceEvent {
    Added { device_index: i32 },
    Removed { joystick_id: JoystickID },
    Remapped { joystick_id: JoystickID },
  }

  impl TryFrom<SDL_ControllerDeviceEvent> for ControllerDeviceEvent {
    type Error = ();
    #[inline]
    #[must_use]
    fn try_from(
      controller_device_event: SDL_ControllerDeviceEvent,
    ) -> Result<Self, Self::Error> {
      Ok(match controller_device_event.type_ as SDL_EventType {
        SDL_CONTROLLERDEVICEADDED => {
          Self::Added { device_index: controller_device_event.which }
        }
        SDL_CONTROLLERDEVICEREMOVED => Self::Removed {
          joystick_id: JoystickID(controller_device_event.which),
        },
        SDL_CONTROLLERDEVICEREMAPPED => Self::Remapped {
          joystick_id: JoystickID(controller_device_event.which),
        },
        _ => return Err(()),
      })
    }
  }
}

//
