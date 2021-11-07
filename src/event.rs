use fermium::prelude::{
  SDL_Event, SDL_Keycode, SDL_PollEvent, SDL_Scancode, SDL_KEYDOWN, SDL_KEYUP, SDL_MOUSEBUTTONDOWN,
  SDL_MOUSEBUTTONUP, SDL_MOUSEMOTION, SDL_MOUSEWHEEL, SDL_MOUSEWHEEL_FLIPPED, SDL_PRESSED,
  SDL_QUIT,
};

use crate::init::Sdl;

impl Sdl {
  #[inline]
  #[must_use]
  pub fn poll_event(&self) -> Option<Event> {
    let mut e = SDL_Event::default();
    if unsafe { SDL_PollEvent(&mut e) } == 0 {
      None
    } else {
      Event::try_from(e).ok()
    }
  }
}

#[derive(Debug)]
pub enum Event {
  Quit,
  Keyboard {
    timestamp: u32,
    window_id: u32,
    is_pressed: bool,
    repeat: u8,
    // TODO: better types here
    scancode: SDL_Scancode,
    keycode: SDL_Keycode,
    modifiers: u16,
  },
  MouseButton {
    timestamp: u32,
    window_id: u32,
    mouse_id: u32,
    button: u8,
    is_pressed: bool,
    click_count: u8,
    win_x: i32,
    win_y: i32,
  },
  MouseMotion {
    timestamp: u32,
    window_id: u32,
    mouse_id: u32,
    button_state: u32,
    win_x: i32,
    win_y: i32,
    delta_x: i32,
    delta_y: i32,
  },
  MouseWheel {
    timestamp: u32,
    window_id: u32,
    mouse_id: u32,
    /// positive to the right and negative to the left
    delta_x: i32,
    /// positive away from the user and negative toward the user
    delta_y: i32,
  },
}
impl TryFrom<SDL_Event> for Event {
  type Error = ();
  #[inline]
  #[must_use]
  fn try_from(event: SDL_Event) -> Result<Self, Self::Error> {
    unsafe {
      Ok(match event.type_ {
        SDL_QUIT => Self::Quit,
        SDL_KEYDOWN | SDL_KEYUP => Self::Keyboard {
          timestamp: event.key.timestamp,
          window_id: event.key.windowID,
          is_pressed: event.key.state == SDL_PRESSED,
          repeat: event.key.repeat,
          scancode: event.key.keysym.scancode,
          keycode: event.key.keysym.sym,
          modifiers: event.key.keysym.mod_,
        },
        SDL_MOUSEBUTTONDOWN | SDL_MOUSEBUTTONUP => Self::MouseButton {
          timestamp: event.button.timestamp,
          window_id: event.button.windowID,
          mouse_id: event.button.which,
          button: event.button.button,
          is_pressed: event.button.state == SDL_PRESSED,
          click_count: event.button.clicks,
          win_x: event.button.x,
          win_y: event.button.y,
        },
        SDL_MOUSEMOTION => Self::MouseMotion {
          timestamp: event.motion.timestamp,
          window_id: event.motion.windowID,
          mouse_id: event.motion.which,
          button_state: event.motion.state,
          win_x: event.motion.x,
          win_y: event.motion.y,
          delta_x: event.motion.xrel,
          delta_y: event.motion.yrel,
        },
        SDL_MOUSEWHEEL => Self::MouseWheel {
          timestamp: event.wheel.timestamp,
          window_id: event.wheel.windowID,
          mouse_id: event.wheel.which,
          delta_x: event.wheel.x
            * if event.wheel.direction == SDL_MOUSEWHEEL_FLIPPED { -1 } else { 1 },
          delta_y: event.wheel.y
            * if event.wheel.direction == SDL_MOUSEWHEEL_FLIPPED { -1 } else { 1 },
        },
        _other => return Err(()),
      })
    }
  }
}
