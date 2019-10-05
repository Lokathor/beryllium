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
  pub(crate) ptr: *mut fermium::SDL_GameController,
  pub(crate) _marker: PhantomData<&'sdl SDLToken>,
}
impl<'sdl> Drop for Controller<'sdl> {
  fn drop(&mut self) {
    unsafe { fermium::SDL_GameControllerClose(self.ptr) }
  }
}
impl<'sdl> Controller<'sdl> {
  /// Obtains the mapping string, if available.
  pub fn mapping_string(&self) -> Option<String> {
    let ptr = unsafe { fermium::SDL_GameControllerMapping(self.ptr) };
    if ptr.is_null() {
      None
    } else {
      unsafe {
        let out = Some(gather_string(ptr));
        fermium::SDL_free(ptr as *mut _);
        out
      }
    }
  }

  /// The implementation defined name for this controller.
  pub fn name(&self) -> Option<String> {
    let ptr = unsafe { fermium::SDL_GameControllerName(self.ptr) };
    if ptr.is_null() {
      None
    } else {
      unsafe { Some(gather_string(ptr)) }
    }
  }

  /// Checks that the controller is currently connected.
  pub fn is_attached(&self) -> bool {
    fermium::SDL_TRUE == unsafe { fermium::SDL_GameControllerGetAttached(self.ptr) }
  }

  /// Gives the axis value of the specified axis.
  ///
  /// * Sticks: negative to positive
  /// * Triggers: zero to positive
  ///
  /// Also gives 0 on failure. You could call [get_error](get_error) if you
  /// want.
  pub fn axis(&self, axis: ControllerAxis) -> i16 {
    unsafe { fermium::SDL_GameControllerGetAxis(self.ptr, axis as fermium::SDL_GameControllerAxis) }
  }

  /// Gives if the given button is pressed.
  ///
  /// Note that this returns `false` for not-pressed but _also_ if there's some
  /// sort of error. You can call [get_error](get_error) if you really want.
  pub fn button(&self, button: ControllerButton) -> bool {
    1 == unsafe {
      fermium::SDL_GameControllerGetButton(self.ptr, button as fermium::SDL_GameControllerButton)
    }
  }

  /// Attempts to get the joystick ID of this controller.
  pub fn joystick_id(&self) -> Result<JoystickID, String> {
    let joystick_ptr: *mut fermium::SDL_Joystick = unsafe { fermium::SDL_GameControllerGetJoystick(self.ptr) };
    if joystick_ptr.is_null() {
      Err(get_error())
    } else {
      let out = unsafe { fermium::SDL_JoystickInstanceID(joystick_ptr) };
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
  Invalid = fermium::SDL_CONTROLLER_AXIS_INVALID,
  LeftX = fermium::SDL_CONTROLLER_AXIS_LEFTX,
  LeftY = fermium::SDL_CONTROLLER_AXIS_LEFTY,
  RightX = fermium::SDL_CONTROLLER_AXIS_RIGHTX,
  RightY = fermium::SDL_CONTROLLER_AXIS_RIGHTY,
  LeftTrigger = fermium::SDL_CONTROLLER_AXIS_TRIGGERLEFT,
  RightTrigger = fermium::SDL_CONTROLLER_AXIS_TRIGGERRIGHT,
}
impl From<u8> for ControllerAxis {
  fn from(axis: u8) -> Self {
    match fermium::SDL_GameControllerAxis::from(axis) {
      fermium::SDL_CONTROLLER_AXIS_LEFTX => ControllerAxis::LeftX,
      fermium::SDL_CONTROLLER_AXIS_LEFTY => ControllerAxis::LeftY,
      fermium::SDL_CONTROLLER_AXIS_RIGHTX => ControllerAxis::RightX,
      fermium::SDL_CONTROLLER_AXIS_RIGHTY => ControllerAxis::RightY,
      fermium::SDL_CONTROLLER_AXIS_TRIGGERLEFT => ControllerAxis::LeftTrigger,
      fermium::SDL_CONTROLLER_AXIS_TRIGGERRIGHT => ControllerAxis::RightTrigger,
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
  Invalid = fermium::SDL_CONTROLLER_BUTTON_INVALID,
  South = fermium::SDL_CONTROLLER_BUTTON_A,
  East = fermium::SDL_CONTROLLER_BUTTON_B,
  West = fermium::SDL_CONTROLLER_BUTTON_X,
  North = fermium::SDL_CONTROLLER_BUTTON_Y,
  Back = fermium::SDL_CONTROLLER_BUTTON_BACK,
  Guide = fermium::SDL_CONTROLLER_BUTTON_GUIDE,
  Start = fermium::SDL_CONTROLLER_BUTTON_START,
  LeftStick = fermium::SDL_CONTROLLER_BUTTON_LEFTSTICK,
  RightStick = fermium::SDL_CONTROLLER_BUTTON_RIGHTSTICK,
  L1 = fermium::SDL_CONTROLLER_BUTTON_LEFTSHOULDER,
  R1 = fermium::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER,
  Up = fermium::SDL_CONTROLLER_BUTTON_DPAD_UP,
  Down = fermium::SDL_CONTROLLER_BUTTON_DPAD_DOWN,
  Left = fermium::SDL_CONTROLLER_BUTTON_DPAD_LEFT,
  Right = fermium::SDL_CONTROLLER_BUTTON_DPAD_RIGHT,
}
impl From<u8> for ControllerButton {
  fn from(button: u8) -> Self {
    match fermium::SDL_GameControllerButton::from(button) {
      fermium::SDL_CONTROLLER_BUTTON_A => ControllerButton::South,
      fermium::SDL_CONTROLLER_BUTTON_B => ControllerButton::East,
      fermium::SDL_CONTROLLER_BUTTON_X => ControllerButton::West,
      fermium::SDL_CONTROLLER_BUTTON_Y => ControllerButton::North,
      fermium::SDL_CONTROLLER_BUTTON_BACK => ControllerButton::Back,
      fermium::SDL_CONTROLLER_BUTTON_GUIDE => ControllerButton::Guide,
      fermium::SDL_CONTROLLER_BUTTON_START => ControllerButton::Start,
      fermium::SDL_CONTROLLER_BUTTON_LEFTSTICK => ControllerButton::LeftStick,
      fermium::SDL_CONTROLLER_BUTTON_RIGHTSTICK => ControllerButton::RightStick,
      fermium::SDL_CONTROLLER_BUTTON_LEFTSHOULDER => ControllerButton::L1,
      fermium::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER => ControllerButton::R1,
      fermium::SDL_CONTROLLER_BUTTON_DPAD_UP => ControllerButton::Up,
      fermium::SDL_CONTROLLER_BUTTON_DPAD_DOWN => ControllerButton::Down,
      fermium::SDL_CONTROLLER_BUTTON_DPAD_LEFT => ControllerButton::Left,
      fermium::SDL_CONTROLLER_BUTTON_DPAD_RIGHT => ControllerButton::Right,
      _ => ControllerButton::Invalid,
    }
  }
}
