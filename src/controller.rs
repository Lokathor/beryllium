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

  /// The dead-zone constant used by default when `Controller::get_axis` is
  /// called with `LeftX` or `LeftY`. This is 7849, which corresponds to the
  /// left thumbstick deadzone recommended by XInput.
  ///
  /// Note that typically it's not necessary to use this, and you should just
  /// use `Controller::get_axis` instead.
  pub const LEFT_THUMBSTICK_DEADZONE: i16 = 7849;

  /// The dead-zone constant used by default when `Controller::get_axis` is
  /// called with `RightX` or `RightY`. This is 8689, which corresponds to the
  /// right thumbstick deadzone recommended by XInput.
  pub const RIGHT_THUMBSTICK_DEADZONE: i16 = 8689;

  /// The dead-zone constant used by default when `Controller::get_axis` is
  /// called with `LeftTrigger` or `RightTrigger`. This is 3855, which
  /// corresponds to the trigger threshold recommended by XInput (30), rescaled
  /// from the XInput range of 0-255 to the range of 0 to `i16::MAX` that SDL
  /// uses.
  pub const TRIGGER_THRESHOLD: i16 = 3855;

  /// Gives the axis value of the specified axis as a floating point number,
  /// after adjusting for the appropriate deadzone.
  ///
  /// * Sticks: range from -1.0 to 1.0
  /// * Triggers: range from 0.0 to 1.0
  ///
  /// Note that this is not generally what you want for thumb-sticks, as it will
  /// only perform axis-independent (e.g. rectangular) deadzone handling. For
  /// the thumb-sticks, using [`Controller::get_stick`] is likely to result in
  /// better handling much of the time.
  pub fn get_axis(&self, axis: ControllerAxis) -> f32 {
    let scaled_value = f32::from(self.get_raw_axis(axis)) * INVERSE_AXIS_RANGE;
    match axis {
      ControllerAxis::Invalid => {
        0.0
      }
      ControllerAxis::LeftTrigger | ControllerAxis::RightTrigger => {
        clamp(adjust_axis_deadzone(scaled_value, f32::from(Self::TRIGGER_THRESHOLD) * INVERSE_AXIS_RANGE), 0.0, 1.0)
      }
      ControllerAxis::LeftX | ControllerAxis::LeftY => {
        clamp(adjust_axis_deadzone(scaled_value, f32::from(Self::LEFT_THUMBSTICK_DEADZONE) * INVERSE_AXIS_RANGE), -1.0, 1.0)
      }
      ControllerAxis::RightX | ControllerAxis::RightY => {
        clamp(adjust_axis_deadzone(scaled_value, f32::from(Self::RIGHT_THUMBSTICK_DEADZONE) * INVERSE_AXIS_RANGE), -1.0, 1.0)
      }
    }
  }

  /// Gives the orientation of one of this controller's analog sticks as a pair
  /// of floating point numbers.
  ///
  /// The result of this is after handling circular deadzone (unlike `get_axis`).
  pub fn get_stick(&self, stick: ControllerStick) -> (f32, f32) {
    let (x, y, dz) = match stick {
      ControllerStick::Left => {
        (
          f32::from(self.get_raw_axis(ControllerAxis::LeftX)) * INVERSE_AXIS_RANGE,
          f32::from(self.get_raw_axis(ControllerAxis::LeftY)) * INVERSE_AXIS_RANGE,
          f32::from(Self::LEFT_THUMBSTICK_DEADZONE) * INVERSE_AXIS_RANGE,
        )
      }
      ControllerStick::Right => {
        (
          f32::from(self.get_raw_axis(ControllerAxis::RightX)) * INVERSE_AXIS_RANGE,
          f32::from(self.get_raw_axis(ControllerAxis::RightY)) * INVERSE_AXIS_RANGE,
          f32::from(Self::RIGHT_THUMBSTICK_DEADZONE) * INVERSE_AXIS_RANGE,
        )
      }
    };
    let len_sq = x * x + y * y;
    if len_sq <= dz * dz {
      (0.0, 0.0)
    } else {
      let len = len_sq.sqrt();
      let new_len = (len - dz) / (1.0 - dz);
      let len_scale = new_len / len;
      (x * len_scale, y * len_scale)
    }
  }

  /// Gives the "raw" axis value of the specified axis. This applies no
  /// dead-zone handling, and just returns the value directly from SDL.
  ///
  /// * Sticks: range from `i16::MIN` to `i16::MAX`
  /// * Triggers: range from `0` to `i16::MAX`.
  ///
  /// Also gives 0 on failure. You could call [get_error](get_error) if you
  /// want.
  pub fn get_raw_axis(&self, axis: ControllerAxis) -> i16 {
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

const INVERSE_AXIS_RANGE: f32 = 1.0 / (std::i16::MAX as f32);

#[inline]
fn adjust_axis_deadzone(value: f32, dead_zone: f32) -> f32 {
  debug_assert!(dead_zone < 1.0);
  let adjusted = if value < -dead_zone {
    value + dead_zone
  } else if value > dead_zone {
    value - dead_zone
  } else {
    0.0
  };
  adjusted / (1.0 - dead_zone)
}

#[inline]
fn clamp(v: f32, lo: f32, hi: f32) -> f32 {
  if v < lo { lo } else if v > hi { hi } else { v }
}

/// One of the two analog sticks on the controller. Mainly used as input to
/// `Controller::get_stick`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum ControllerStick {
  /// The left analog stick.
  Left,
  /// The right analog stick.
  Right,
}

/// The types of axises that a Controller has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
#[allow(missing_docs)]
pub enum ControllerAxis {
  /// A placeholder value representing an invalid controller axis.
  Invalid = SDL_CONTROLLER_AXIS_INVALID,
  /// The controller axis corresponding to horizontal movement of the left
  /// analog stick.
  LeftX = SDL_CONTROLLER_AXIS_LEFTX,
  /// The controller axis corresponding to vertical movement of the left
  /// analog stick.
  ///
  /// Negative values of this axis go left, and positive values go right.
  LeftY = SDL_CONTROLLER_AXIS_LEFTY,
  /// The controller axis corresponding to horizontal movement of the right
  /// analog stick.
  ///
  /// Negative values of this axis go down, and positive values go up.
  RightX = SDL_CONTROLLER_AXIS_RIGHTX,
  /// The controller axis corresponding to vertical movement of the right
  /// analog stick.
  ///
  /// Negative values of this axis go down, and positive values go up.
  RightY = SDL_CONTROLLER_AXIS_RIGHTY,
  /// The controller axis corresponding to the back-most left trigger (L2).
  ///
  /// Note that this axis is never a negative value.
  LeftTrigger = SDL_CONTROLLER_AXIS_TRIGGERLEFT,
  /// The controller axis corresponding to the back-most right trigger (R2).
  ///
  /// Note that this axis is never a negative value.
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
  /// A placeholder value representing an invalid button
  Invalid = SDL_CONTROLLER_BUTTON_INVALID,
  /// The bottom-most face button.
  ///
  /// - On gamepads layed out like an Xbox controller, this is the A button.
  /// - On gamepads layed out like a PlayStation controller, this is the cross button.
  South = SDL_CONTROLLER_BUTTON_A,
  /// The right-most face button.
  ///
  /// - On gamepads layed out like an Xbox controller, this is the B button.
  /// - On gamepads layed out like a PlayStation controller, this is the circle button.
  East = SDL_CONTROLLER_BUTTON_B,
  /// The left-most face button.
  ///
  /// - On gamepads layed out like an Xbox controller, this is the X button.
  /// - On gamepads layed out like a PlayStation controller, this is the square button.
  West = SDL_CONTROLLER_BUTTON_X,
  /// The top-most face button.
  ///
  /// - On gamepads layed out like an Xbox controller, this is the Y button.
  /// - On gamepads layed out like a PlayStation controller, this is the triangle button.
  North = SDL_CONTROLLER_BUTTON_Y,
  /// The back button on the controller, typically located to in the
  /// center-left of the controller (left of `ControllerButton::Guide`).
  Back = SDL_CONTROLLER_BUTTON_BACK,
  /// The guide button on the controller, which is typically a large button
  /// near the center of the controller.
  Guide = SDL_CONTROLLER_BUTTON_GUIDE,
  /// The start button on the controller, typically located to in the
  /// center-right of the controller (right of `ControllerButton::Guide`).
  Start = SDL_CONTROLLER_BUTTON_START,
  /// The button beneath the left joystick, sometimes also known as L3.
  LeftStick = SDL_CONTROLLER_BUTTON_LEFTSTICK,
  /// The button beneath the right joystick, sometimes also known as R3.
  RightStick = SDL_CONTROLLER_BUTTON_RIGHTSTICK,
  /// The (front-most) left shoulder button.
  ///
  /// Note that the other left shoulder button (L2) is represented by
  /// [`ControllerAxis::LeftTrigger`] and not a button.
  L1 = SDL_CONTROLLER_BUTTON_LEFTSHOULDER,
  /// The (front-most) right shoulder button.
  ///
  /// Note that the other right shoulder button (R2) is represented by
  /// the axis [`ControllerAxis::RightTrigger`], and not a button.
  R1 = SDL_CONTROLLER_BUTTON_RIGHTSHOULDER,
  /// The button corresponding to up on the controller's d-pad.
  Up = SDL_CONTROLLER_BUTTON_DPAD_UP,
  /// The button corresponding to down on the controller's d-pad.
  Down = SDL_CONTROLLER_BUTTON_DPAD_DOWN,
  /// The button corresponding to left on the controller's d-pad.
  Left = SDL_CONTROLLER_BUTTON_DPAD_LEFT,
  /// The button corresponding to right on the controller's d-pad.
  Right = SDL_CONTROLLER_BUTTON_DPAD_RIGHT,
}
