use super::*;

use core::sync::atomic::{AtomicBool, Ordering};

use alloc::{boxed::Box, string::String};

use fermium::{
  SDL_Init, SDL_InitFlags, SDL_Quit, SDL_INIT_AUDIO, SDL_INIT_EVENTS,
  SDL_INIT_EVERYTHING, SDL_INIT_GAMECONTROLLER, SDL_INIT_HAPTIC,
  SDL_INIT_JOYSTICK, SDL_INIT_TIMER, SDL_INIT_VIDEO,
};

/// Flags of what systems to initialize.
pub struct InitFlags(SDL_InitFlags);
impl_bit_ops_for_tuple_newtype!(InitFlags);
impl InitFlags {
  /// Timer subsystem.
  pub const TIMER: Self = Self(SDL_INIT_TIMER);

  /// Audio capture/playback.
  pub const AUDIO: Self = Self(SDL_INIT_AUDIO);

  /// Video / Windowing.
  ///
  /// Implies `EVENTS`.
  pub const VIDEO: Self = Self(SDL_INIT_VIDEO);

  /// Joystick input.
  ///
  /// Implies `EVENTS`.
  pub const JOYSTICK: Self = Self(SDL_INIT_JOYSTICK);

  /// Joystick force-feedback.
  pub const HAPTIC: Self = Self(SDL_INIT_HAPTIC);

  /// Game Controller abstraction.
  ///
  /// Implies `JOYSTICK`.
  pub const GAME_CONTROLLER: Self = Self(SDL_INIT_GAMECONTROLLER);

  /// Events subsystem.
  pub const EVENTS: Self = Self(SDL_INIT_EVENTS);

  /// All subsystems.
  pub const EVERYTHING: Self = Self(SDL_INIT_EVERYTHING);

  /// Turn on no subsystems. Primarily useful for some unit tests.
  pub const NONE: Self = Self(SDL_InitFlags(0));
}

pub(crate) struct Init(());

impl Drop for Init {
  fn drop(&mut self) {
    unsafe { SDL_Quit() };
    SDL_ACTIVE.store(false, Ordering::SeqCst);
  }
}

static SDL_ACTIVE: AtomicBool = AtomicBool::new(false);

impl Init {
  /// Attempt to initialize SDL2 according to the flags given.
  pub fn new(flags: InitFlags) -> BerylliumResult<Self> {
    if !SDL_ACTIVE.compare_and_swap(false, true, Ordering::SeqCst) {
      let r = unsafe { SDL_Init(flags.0) };
      if r < 0 {
        let err = get_error();
        SDL_ACTIVE.store(false, Ordering::SeqCst);
        Err(err)
      } else {
        Ok(Init(()))
      }
    } else {
      Err(BerylliumError(Box::new(String::from(
        "beryllium: failed to init, already initialized.",
      ))))
    }
  }
}

#[test]
fn impl_init_new() {
  let a = Init::new(InitFlags::NONE);
  assert!(a.is_ok());
  let b = Init::new(InitFlags::NONE);
  assert!(b.is_err());
}
