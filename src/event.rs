use fermium::prelude::{SDL_Event, SDL_PollEvent, SDL_QUIT};

use crate::init::Sdl;

impl Sdl {
  pub fn poll_event(&self) -> Option<Event> {
    let mut e = SDL_Event::default();
    if unsafe { SDL_PollEvent(&mut e) } == 0 {
      None
    } else {
      Event::try_from(e).ok()
    }
  }
}

pub enum Event {
  Quit,
}
impl TryFrom<SDL_Event> for Event {
  type Error = ();
  fn try_from(e: SDL_Event) -> Result<Self, Self::Error> {
    Ok(match unsafe { e.type_ } {
      SDL_QUIT => Self::Quit,
      _ => return Err(()),
    })
  }
}
