use super::*;

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
  pub fn get_axis(&self, axis: ControllerAxis) -> i16 {
    unsafe { SDL_GameControllerGetAxis(self.ptr, axis as fermium::SDL_GameControllerAxis::Type) }
  }

  /// Gives if the given button is pressed.
  ///
  /// Note that this returns `false` for not-pressed but _also_ if there's some
  /// sort of error. You can call [get_error](get_error) if you really want.
  pub fn get_button(&self, button: ControllerButton) -> bool {
    1 == unsafe {
      SDL_GameControllerGetButton(self.ptr, button as fermium::SDL_GameControllerButton::Type)
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
