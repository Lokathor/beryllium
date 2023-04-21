#![warn(clippy::missing_inline_in_public_items)]

extern crate alloc;

use alloc::sync::Arc;
use fermium::prelude::SDL_SetHint;
use init::{InitFlags, SdlInit};

pub mod controller;
pub mod error;
pub mod events;
pub mod init;
pub mod surface;
pub mod video;

#[derive(Clone)]
#[repr(transparent)]
pub struct Sdl {
  init: Arc<SdlInit>,
}
impl Sdl {
  #[inline]
  pub fn init(flags: InitFlags) -> Self {
    Self { init: SdlInit::try_new_arc(flags).unwrap() }
  }

  #[inline]
  pub fn set_controller_use_button_labels(&self, labels: bool) -> bool {
    const SDL_HINT_GAMECONTROLLER_USE_BUTTON_LABELS: &[u8] =
      b"SDL_GAMECONTROLLER_USE_BUTTON_LABELS\0";
    let value: &[u8] = if labels { b"1\0" } else { b"0\0" };
    unsafe {
      SDL_SetHint(SDL_HINT_GAMECONTROLLER_USE_BUTTON_LABELS.as_ptr().cast(), value.as_ptr().cast())
    }
    .into()
  }
}
