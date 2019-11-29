
use super::*;

/// A game controller.
///
/// This is a more abstract interface over top of a joystick.
///
/// You don't need to do much with it, just keep it open somewhere and events
/// will come in through [`poll_events`](SDL::poll_events).
pub struct Controller {
  #[allow(unused)]
  pub(crate) init_token: Arc<Initialization>,
  pub(crate) device: *mut fermium::SDL_GameController,
  pub(crate) joystick_id: fermium::SDL_JoystickID,
}
impl Drop for Controller {
  fn drop(&mut self) {
    unsafe {
      fermium::SDL_GameControllerClose(self.device)
    }
  }
}
impl Controller {
  /// Get the instance ID for the joystick backing this controller.
  pub fn joystick_id(&self) -> fermium::SDL_JoystickID {
    self.joystick_id
  }
}
