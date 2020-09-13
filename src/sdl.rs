use core::{
  convert::TryFrom,
  marker::PhantomData,
  sync::atomic::{AtomicBool, Ordering},
};

use alloc::{boxed::Box, string::String, sync::Arc};

use crate::{
  sdl_get_error, Event, RendererWindow, SdlError, WindowCreationFlags, Controller,
};

static SDL_ACTIVE: AtomicBool = AtomicBool::new(false);

pub(crate) struct Initialization(PhantomData<*mut u8>);

impl Drop for Initialization {
  fn drop(&mut self) {
    unsafe { fermium::SDL_Quit() }
    SDL_ACTIVE.store(false, Ordering::SeqCst)
  }
}

impl Initialization {
  fn init(flags: InitFlags) -> Result<Arc<Initialization>, SdlError> {
    if SDL_ACTIVE.compare_and_swap(false, true, Ordering::SeqCst) {
      // true came back, so SDL was on, so this is a double init.
      Err(SdlError(Box::new(String::from("beryllium: SDL is already active!"))))
    } else {
      // false came back, so SDL was not on, so we begin normal initialization.
      #[cfg(any(target_os = "macos", target_os = "ios"))]
      {
        use objc::{class, msg_send, sel, sel_impl};
        let is_main: bool =
          unsafe { msg_send![class!(NSThread), isMainThread] };
        if !is_main {
          SDL_ACTIVE.store(false, Ordering::SeqCst);
          return Err(Box::new(String::from(
            "beryllium: SDL must be initialized on the main thread.",
          )));
        }
      }
      let ret = unsafe { fermium::SDL_Init(flags.0) };
      if ret < 0 {
        Err(sdl_get_error())
      } else {
        Ok(Arc::new(Initialization(PhantomData)))
      }
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct InitFlags(u32);
impl InitFlags {
  pub const TIMER: Self = Self(fermium::SDL_INIT_TIMER);
  pub const AUDIO: Self = Self(fermium::SDL_INIT_AUDIO);
  pub const VIDEO: Self = Self(fermium::SDL_INIT_VIDEO);
  pub const JOYSTICK: Self = Self(fermium::SDL_INIT_JOYSTICK);
  pub const HAPTIC: Self = Self(fermium::SDL_INIT_HAPTIC);
  pub const CONTORLLER: Self = Self(fermium::SDL_INIT_GAMECONTROLLER);
  pub const EVENTS: Self = Self(fermium::SDL_INIT_EVENTS);
  pub const EVERYTHING: Self = Self(fermium::SDL_INIT_EVERYTHING);
}

pub struct Sdl {
  #[allow(dead_code)]
  init: Arc<Initialization>,
}
impl core::fmt::Debug for Sdl {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "Sdl")
  }
}

impl Sdl {
  /// Initializes SDL2.
  ///
  /// ## Failure
  /// Possible failures include:
  /// * On Mac, you must initialize SDL from the main thread.
  /// * You cannot double initialize SDL.
  pub fn init(flags: InitFlags) -> Result<Self, SdlError> {
    Initialization::init(flags).map(|init| Self { init })
  }

  /// Polls for a pending event.
  ///
  /// * Always returns immediately.
  /// * If no event is pending, gives `None`.
  /// * If the event from SDL can't be parsed, you also get `None`.
  /// * The `u32` is the event's timestamp (milliseconds since SDL's
  ///   initialization).
  pub fn poll_event(&self) -> Option<(Event, u32)> {
    use fermium::{SDL_Event, SDL_PollEvent};
    let mut sdl_event = SDL_Event::default();
    let ret = unsafe { SDL_PollEvent(&mut sdl_event) };
    if ret != 0 {
      let timestamp = unsafe { sdl_event.common.timestamp };
      Event::try_from(sdl_event).map(|ev| (ev, timestamp)).ok()
    } else {
      None
    }
  }

  /// Waits for a pending event.
  ///
  /// * Blocks if no event is available.
  /// * Returns `Err` if there's a problem during the wait.
  /// * The `u32` is the event's timestamp (milliseconds since SDL's
  ///   initialization).
  pub fn wait_event(&self) -> Result<(Event, u32), SdlError> {
    use fermium::{SDL_Event, SDL_WaitEvent};
    let mut sdl_event = SDL_Event::default();
    let ret = unsafe { SDL_WaitEvent(&mut sdl_event) };
    if ret != 0 {
      let timestamp = unsafe { sdl_event.common.timestamp };
      Event::try_from(sdl_event).map(|ev| (ev, timestamp)).map_err(|()| {
        SdlError(Box::new(alloc::format!(
          "Could not parse event, {:?}",
          sdl_event
        )))
      })
    } else {
      Err(sdl_get_error())
    }
  }

  /// Waits for a pending event, but with a timeout.
  ///
  /// * Blocks if no event is available, up to the given number of milliseconds.
  /// * Returns `Err` if there's a problem during the wait, or if the wait timed
  ///   out.
  /// * The `u32` is the event's timestamp (milliseconds since SDL's
  ///   initialization).
  pub fn wait_event_timeout(
    &self, milliseconds: i32,
  ) -> Result<(Event, u32), SdlError> {
    use fermium::{SDL_Event, SDL_WaitEventTimeout};
    let mut sdl_event = SDL_Event::default();
    let ret = unsafe { SDL_WaitEventTimeout(&mut sdl_event, milliseconds) };
    if ret != 0 {
      let timestamp = unsafe { sdl_event.common.timestamp };
      Event::try_from(sdl_event).map(|ev| (ev, timestamp)).map_err(|()| {
        SdlError(Box::new(alloc::format!(
          "Could not parse event, {:?}",
          sdl_event
        )))
      })
    } else {
      Err(sdl_get_error())
    }
  }

  /// Creates a new window that uses SDL2's 2D rendering system.
  pub fn new_renderer_window(
    &self, title: &str, pos: Option<[i32; 2]>, size: [u32; 2],
    flags: WindowCreationFlags,
  ) -> Result<RendererWindow, SdlError> {
    RendererWindow::new(self.init.clone(), title, pos, size, flags)
  }

  pub fn get_number_of_joysticks(&self) -> Result<usize, SdlError> {
    let ret = unsafe { fermium::SDL_NumJoysticks() };
    if ret >= 0 {
      Ok(ret as usize)
    } else {
      Err(sdl_get_error())
    }
  }

  pub fn open_controller(&self, id: usize) -> Result<Controller, SdlError> {
    Controller::open(self.init.clone(), id)
  }
}
