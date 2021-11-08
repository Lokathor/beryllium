use core::ptr::NonNull;

use crate::{get_error, init::Sdl, SdlResult};

use fermium::{
  gamecontroller::{SDL_GameControllerClose, SDL_GameControllerOpen},
  prelude::SDL_GameController,
};

pub struct Controller {
  pub(crate) nn: NonNull<SDL_GameController>,
  #[allow(unused)]
  sdl: Sdl,
}
impl Drop for Controller {
  fn drop(&mut self) {
    unsafe { SDL_GameControllerClose(self.nn.as_ptr()) };
  }
}

impl Sdl {
  #[inline]
  pub fn game_controller_open(&self, joystick_index: i32) -> SdlResult<Controller> {
    match NonNull::new(unsafe { SDL_GameControllerOpen(joystick_index) }) {
      Some(nn) => Ok(Controller { nn, sdl: self.clone() }),
      None => Err(get_error()),
    }
  }
}
