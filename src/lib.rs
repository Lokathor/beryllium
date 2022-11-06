#![no_std]
#![allow(dead_code)]
#![warn(clippy::missing_inline_in_public_items)]

extern crate alloc;

use alloc::sync::Arc;
use init::{InitFlags, SdlInit};

pub mod controller;
pub mod error;
pub mod events;
pub mod init;
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
}
