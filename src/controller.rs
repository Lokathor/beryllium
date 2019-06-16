use super::*;

/// A joystick ID value is just a non-negative `i32` value.
///
/// There's no special safety involved here. Invalid ID values will cause the
/// API to return errors, but nothing will explode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct JoystickID(pub i32);

/// Handle to a window on the screen.
#[derive(Debug)]
#[repr(transparent)]
pub struct Controller<'sdl> {
  pub(crate) ptr: *mut SDL_GameController,
  pub(crate) _marker: PhantomData<&'sdl SDLToken>,
}
impl<'sdl> Drop for Controller<'sdl> {
  fn drop(&mut self) {
    unsafe { SDL_GameControllerClose(self.ptr) }
  }
}
impl<'sdl> Controller<'sdl> {
  /// Obtains the mapping string, if available.
  pub fn mapping_string(&self) -> Option<String> {
    let ptr = unsafe { SDL_GameControllerMapping(self.ptr) };
    if ptr.is_null() {
      None
    } else {
      unsafe {
        let out = Some(gather_string(ptr));
        SDL_free(ptr as *mut _);
        out
      }
    }
  }

  /// The implementation defined name for this controller.
  pub fn name(&self) -> Option<String> {
    let ptr = unsafe { SDL_GameControllerName(self.ptr) };
    if ptr.is_null() {
      None
    } else {
      unsafe { Some(gather_string(ptr)) }
    }
  }

  /// Checks that the controller is currently connected.
  pub fn is_attached(&self) -> bool {
    SDL_TRUE == unsafe { SDL_GameControllerGetAttached(self.ptr) }
  }

  /// Gives the axis value of the specified axis.
  ///
  /// * Sticks: negative to positive
  /// * Triggers: zero to positive
  ///
  /// Also gives 0 on failure. You could call [get_error](get_error) if you
  /// want.
  pub fn axis(&self, axis: ControllerAxis) -> i16 {
    unsafe { SDL_GameControllerGetAxis(self.ptr, axis as fermium::SDL_GameControllerAxis::Type) }
  }

  /// Gives if the given button is pressed.
  ///
  /// Note that this returns `false` for not-pressed but _also_ if there's some
  /// sort of error. You can call [get_error](get_error) if you really want.
  pub fn button(&self, button: ControllerButton) -> bool {
    1 == unsafe {
      SDL_GameControllerGetButton(self.ptr, button as fermium::SDL_GameControllerButton::Type)
    }
  }

  /// Attempts to get the joystick ID of this controller.
  pub fn joystick_id(&self) -> Result<JoystickID, String> {
    let joystick_ptr: *mut SDL_Joystick = unsafe { SDL_GameControllerGetJoystick(self.ptr) };
    if joystick_ptr.is_null() {
      Err(get_error())
    } else {
      let out = unsafe { SDL_JoystickInstanceID(joystick_ptr) };
      if out < 0 {
        Err(get_error())
      } else {
        Ok(JoystickID(out))
      }
    }
  }
}

/// The types of axises that a Controller has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
#[allow(missing_docs)]
pub enum ControllerAxis {
  Invalid = SDL_CONTROLLER_AXIS_INVALID,
  LeftX = SDL_CONTROLLER_AXIS_LEFTX,
  LeftY = SDL_CONTROLLER_AXIS_LEFTY,
  RightX = SDL_CONTROLLER_AXIS_RIGHTX,
  RightY = SDL_CONTROLLER_AXIS_RIGHTY,
  LeftTrigger = SDL_CONTROLLER_AXIS_TRIGGERLEFT,
  RightTrigger = SDL_CONTROLLER_AXIS_TRIGGERRIGHT,
}
impl From<u8> for ControllerAxis {
  fn from(axis: u8) -> Self {
    match fermium::SDL_GameControllerAxis::Type::from(axis) {
      SDL_CONTROLLER_AXIS_LEFTX => ControllerAxis::LeftX,
      SDL_CONTROLLER_AXIS_LEFTY => ControllerAxis::LeftY,
      SDL_CONTROLLER_AXIS_RIGHTX => ControllerAxis::RightX,
      SDL_CONTROLLER_AXIS_RIGHTY => ControllerAxis::RightY,
      SDL_CONTROLLER_AXIS_TRIGGERLEFT => ControllerAxis::LeftTrigger,
      SDL_CONTROLLER_AXIS_TRIGGERRIGHT => ControllerAxis::RightTrigger,
      _ => ControllerAxis::Invalid,
    }
  }
}

/// The types of buttons that a Controller has.
///
/// Here's the deal:
///
/// * SDL2 follows the XBox button name convention.
/// * I personally follow the SNES button name convention.
///
/// These two layouts use the same names, but with inverted positions. Naturally
/// this leads to annoyance and confusion. The middle ground is that this enum
/// names the face buttons using _compass directions_ as the button names.
///
/// Of course, then the naming is kinda screwed up when using a GameCube
/// controller (which is, kinda, 90 degrees clockwise from an SNES controller),
/// but ultimately [controller layouts are
/// chaos](https://memestatic.fjcdn.com/pictures/Press+x_875402_6229280.jpg).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
#[allow(missing_docs)]
pub enum ControllerButton {
  Invalid = SDL_CONTROLLER_BUTTON_INVALID,
  South = SDL_CONTROLLER_BUTTON_A,
  East = SDL_CONTROLLER_BUTTON_B,
  West = SDL_CONTROLLER_BUTTON_X,
  North = SDL_CONTROLLER_BUTTON_Y,
  Back = SDL_CONTROLLER_BUTTON_BACK,
  Guide = SDL_CONTROLLER_BUTTON_GUIDE,
  Start = SDL_CONTROLLER_BUTTON_START,
  LeftStick = SDL_CONTROLLER_BUTTON_LEFTSTICK,
  RightStick = SDL_CONTROLLER_BUTTON_RIGHTSTICK,
  L1 = SDL_CONTROLLER_BUTTON_LEFTSHOULDER,
  R1 = SDL_CONTROLLER_BUTTON_RIGHTSHOULDER,
  Up = SDL_CONTROLLER_BUTTON_DPAD_UP,
  Down = SDL_CONTROLLER_BUTTON_DPAD_DOWN,
  Left = SDL_CONTROLLER_BUTTON_DPAD_LEFT,
  Right = SDL_CONTROLLER_BUTTON_DPAD_RIGHT,
}
impl From<u8> for ControllerButton {
  fn from(button: u8) -> Self {
    match fermium::SDL_GameControllerButton::Type::from(button) {
      SDL_CONTROLLER_BUTTON_A => ControllerButton::South,
      SDL_CONTROLLER_BUTTON_B => ControllerButton::East,
      SDL_CONTROLLER_BUTTON_X => ControllerButton::West,
      SDL_CONTROLLER_BUTTON_Y => ControllerButton::North,
      SDL_CONTROLLER_BUTTON_BACK => ControllerButton::Back,
      SDL_CONTROLLER_BUTTON_GUIDE => ControllerButton::Guide,
      SDL_CONTROLLER_BUTTON_START => ControllerButton::Start,
      SDL_CONTROLLER_BUTTON_LEFTSTICK => ControllerButton::LeftStick,
      SDL_CONTROLLER_BUTTON_RIGHTSTICK => ControllerButton::RightStick,
      SDL_CONTROLLER_BUTTON_LEFTSHOULDER => ControllerButton::L1,
      SDL_CONTROLLER_BUTTON_RIGHTSHOULDER => ControllerButton::R1,
      SDL_CONTROLLER_BUTTON_DPAD_UP => ControllerButton::Up,
      SDL_CONTROLLER_BUTTON_DPAD_DOWN => ControllerButton::Down,
      SDL_CONTROLLER_BUTTON_DPAD_LEFT => ControllerButton::Left,
      SDL_CONTROLLER_BUTTON_DPAD_RIGHT => ControllerButton::Right,
      _ => ControllerButton::Invalid,
    }
  }
}
