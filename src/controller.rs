use core::ptr::NonNull;

use alloc::sync::Arc;

use fermium::SDL_GameController;

use crate::{sdl_get_error, Initialization, SdlError};

pub struct Controller {
  nn: NonNull<SDL_GameController>,
  // Note(Lokathor): As long as the window lives, we have to also keep SDL
  // itself alive.
  #[allow(dead_code)]
  init: Arc<Initialization>,
}
impl Drop for Controller {
  // Note(Lokathor): The drop for the Arc runs *after* this drop code.
  fn drop(&mut self) {
    unsafe { fermium::SDL_GameControllerClose(self.nn.as_ptr()) }
  }
}
impl Controller {
  pub(crate) fn open(
    init: Arc<Initialization>, id: usize,
  ) -> Result<Self, SdlError> {
    NonNull::new(unsafe { fermium::SDL_GameControllerOpen(id as i32) })
      .ok_or_else(sdl_get_error)
      .map(|nn| Controller { init, nn })
  }
}
