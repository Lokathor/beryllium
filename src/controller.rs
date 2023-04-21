use core::ptr::NonNull;

use alloc::{string::String, sync::Arc, vec::Vec};
use fermium::prelude::*;

use crate::{
  error::{get_error, SdlError},
  init::SdlInit,
  Sdl,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum ControllerAxis {
  Invalid = SDL_CONTROLLER_AXIS_INVALID.0,
  LeftX = SDL_CONTROLLER_AXIS_LEFTX.0,
  LeftY = SDL_CONTROLLER_AXIS_LEFTY.0,
  RightX = SDL_CONTROLLER_AXIS_RIGHTX.0,
  RightY = SDL_CONTROLLER_AXIS_RIGHTY.0,
  TriggerLeft = SDL_CONTROLLER_AXIS_TRIGGERLEFT.0,
  TriggerRight = SDL_CONTROLLER_AXIS_TRIGGERRIGHT.0,
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
impl ControllerAxis {
  pub(crate) fn as_sdl_game_controller_axis(self) -> SDL_GameControllerAxis {
    SDL_GameControllerAxis(self as i32)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum ControllerButton {
  Invalid = SDL_CONTROLLER_BUTTON_INVALID.0,
  Y = SDL_CONTROLLER_BUTTON_Y.0,
  A = SDL_CONTROLLER_BUTTON_A.0,
  B = SDL_CONTROLLER_BUTTON_B.0,
  X = SDL_CONTROLLER_BUTTON_X.0,
  Back = SDL_CONTROLLER_BUTTON_BACK.0,
  Guide = SDL_CONTROLLER_BUTTON_GUIDE.0,
  Start = SDL_CONTROLLER_BUTTON_START.0,
  LeftStick = SDL_CONTROLLER_BUTTON_LEFTSTICK.0,
  RightStick = SDL_CONTROLLER_BUTTON_RIGHTSTICK.0,
  LeftShoulder = SDL_CONTROLLER_BUTTON_LEFTSHOULDER.0,
  RightShoulder = SDL_CONTROLLER_BUTTON_RIGHTSHOULDER.0,
  DpadUp = SDL_CONTROLLER_BUTTON_DPAD_UP.0,
  DpadDown = SDL_CONTROLLER_BUTTON_DPAD_DOWN.0,
  DpadLeft = SDL_CONTROLLER_BUTTON_DPAD_LEFT.0,
  DpadRight = SDL_CONTROLLER_BUTTON_DPAD_RIGHT.0,
  Misc1 = SDL_CONTROLLER_BUTTON_MISC1.0,
  Paddle1 = SDL_CONTROLLER_BUTTON_PADDLE1.0,
  Paddle2 = SDL_CONTROLLER_BUTTON_PADDLE2.0,
  Paddle3 = SDL_CONTROLLER_BUTTON_PADDLE3.0,
  Paddle4 = SDL_CONTROLLER_BUTTON_PADDLE4.0,
  Touchpad = SDL_CONTROLLER_BUTTON_TOUCHPAD.0,
}
impl From<u8> for ControllerButton {
  #[inline]
  fn from(value: u8) -> Self {
    match SDL_GameControllerButton(i32::from(value)) {
      SDL_CONTROLLER_BUTTON_A => ControllerButton::A,
      SDL_CONTROLLER_BUTTON_B => ControllerButton::B,
      SDL_CONTROLLER_BUTTON_X => ControllerButton::X,
      SDL_CONTROLLER_BUTTON_Y => ControllerButton::Y,
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
impl ControllerButton {
  pub(crate) fn as_sdl_game_controller_button(self) -> SDL_GameControllerButton {
    SDL_GameControllerButton(self as i32)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum ControllerType {
  Unknown = SDL_CONTROLLER_TYPE_UNKNOWN.0,
  Xbox360 = SDL_CONTROLLER_TYPE_XBOX360.0,
  XboxOne = SDL_CONTROLLER_TYPE_XBOXONE.0,
  Ps3 = SDL_CONTROLLER_TYPE_PS3.0,
  Ps4 = SDL_CONTROLLER_TYPE_PS4.0,
  NintendoSwitchPro = SDL_CONTROLLER_TYPE_NINTENDO_SWITCH_PRO.0,
  Virtual = SDL_CONTROLLER_TYPE_VIRTUAL.0,
  Ps5 = SDL_CONTROLLER_TYPE_PS5.0,
  AmazonLuna = SDL_CONTROLLER_TYPE_AMAZON_LUNA.0,
  Stadia = SDL_CONTROLLER_TYPE_GOOGLE_STADIA.0,
}
impl From<SDL_GameControllerType> for ControllerType {
  #[inline]
  fn from(value: SDL_GameControllerType) -> Self {
    match value {
      SDL_CONTROLLER_TYPE_XBOX360 => Self::Xbox360,
      SDL_CONTROLLER_TYPE_XBOXONE => Self::XboxOne,
      SDL_CONTROLLER_TYPE_PS3 => Self::Ps3,
      SDL_CONTROLLER_TYPE_PS4 => Self::Ps4,
      SDL_CONTROLLER_TYPE_NINTENDO_SWITCH_PRO => Self::NintendoSwitchPro,
      SDL_CONTROLLER_TYPE_VIRTUAL => Self::Virtual,
      SDL_CONTROLLER_TYPE_PS5 => Self::Ps5,
      SDL_CONTROLLER_TYPE_AMAZON_LUNA => Self::AmazonLuna,
      SDL_CONTROLLER_TYPE_GOOGLE_STADIA => Self::Stadia,
      _ => Self::Unknown,
    }
  }
}

pub struct GameController {
  ctrl: NonNull<SDL_GameController>,
  /// Note(Lokathor): The init is always the LAST field!
  #[allow(dead_code)]
  init: Arc<SdlInit>,
}
impl Sdl {
  #[inline]
  pub fn open_game_controller(&self, index: i32) -> Result<GameController, SdlError> {
    let p = unsafe { SDL_GameControllerOpen(index) };
    match NonNull::new(p) {
      Some(ctrl) => Ok(GameController { ctrl, init: self.init.clone() }),
      None => Err(get_error()),
    }
  }
}
impl Drop for GameController {
  #[inline]
  fn drop(&mut self) {
    unsafe { SDL_GameControllerClose(self.ctrl.as_ptr()) }
  }
}

impl GameController {
  #[inline]
  pub fn is_attached(&self) -> bool {
    unsafe { SDL_GameControllerGetAttached(self.ctrl.as_ptr()) }.into()
  }

  /// Triggers are `0` to `i16::MAX`, Sticks are `i16::MIN` to `i16::MAX`
  #[inline]
  pub fn get_axis(&self, axis: ControllerAxis) -> i16 {
    unsafe { SDL_GameControllerGetAxis(self.ctrl.as_ptr(), axis.as_sdl_game_controller_axis()) }
  }

  #[inline]
  pub fn get_button(&self, button: ControllerButton) -> bool {
    0 != unsafe {
      SDL_GameControllerGetButton(self.ctrl.as_ptr(), button.as_sdl_game_controller_button())
    }
  }

  #[inline]
  pub fn get_name(&self) -> String {
    let mut p: *const u8 = unsafe { SDL_GameControllerName(self.ctrl.as_ptr()).cast() };
    if p.is_null() {
      String::new()
    } else {
      let mut vec: Vec<u8> = Vec::new();
      while unsafe { *p != 0 } {
        vec.push(unsafe { *p });
        p = unsafe { p.add(1) };
      }
      match String::from_utf8(vec) {
        Ok(s) => s,
        Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
      }
    }
  }

  #[inline]
  pub fn get_type(&self) -> ControllerType {
    ControllerType::from(unsafe { SDL_GameControllerGetType(self.ctrl.as_ptr()) })
  }

  #[inline]
  pub fn get_mapping_string(&self) -> String {
    let mut s = String::new();
    let mut p = unsafe { SDL_GameControllerMapping(self.ctrl.as_ptr()) } as *const u8;
    if !p.is_null() {
      while unsafe { *p } != 0 {
        s.push(unsafe { *p } as char);
        p = unsafe { p.add(1) };
      }
    }
    s
  }
}
