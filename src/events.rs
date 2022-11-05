use fermium::prelude::{SDL_Event, SDL_PollEvent, SDL_QUIT};

use crate::Sdl;

impl Sdl {
  #[inline]
  pub fn poll_events(&self) -> Option<(Event, u32)> {
    let mut sdl_event: SDL_Event = SDL_Event::default();
    if unsafe { SDL_PollEvent(&mut sdl_event) } != 0 {
      Event::try_from(sdl_event).ok().map(|e| (e, unsafe { sdl_event.common.timestamp }))
    } else {
      None
    }
  }
}

#[derive(Debug, Clone)]
pub enum Event {
  Quit,
}

impl TryFrom<SDL_Event> for Event {
  type Error = ();
  #[inline]
  fn try_from(sdl_event: SDL_Event) -> Result<Self, Self::Error> {
    Ok(match unsafe { sdl_event.common.type_ } {
      SDL_QUIT => Event::Quit,
      _ => return Err(()),
    })
  }
}
