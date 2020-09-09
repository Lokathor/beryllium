use core::convert::{TryFrom, TryInto};

use fermium::{SDL_Event, SDL_EventType, SDL_QUIT, SDL_WINDOWEVENT};

#[non_exhaustive]
pub enum Event {
  Quit,
  // TODO: display
  Window(WindowEvent),
}

impl TryFrom<SDL_Event> for Event {
  type Error = ();
  #[inline]
  #[must_use]
  fn try_from(sdl_event: SDL_Event) -> Result<Self, Self::Error> {
    // Safety: `sdl_event` is a union so there's all sorts of union access here
    unsafe {
      Ok(match sdl_event.type_ as SDL_EventType {
        SDL_QUIT => Event::Quit,
        SDL_WINDOWEVENT => Event::Window(sdl_event.window.try_into()?),
        _ => return Err(()),
      })
    }
  }
}

pub use window_event::*;
mod window_event {
  use super::*;
  use fermium::{
    SDL_WindowEvent, SDL_WindowEventID, SDL_WINDOWEVENT_CLOSE,
    SDL_WINDOWEVENT_ENTER, SDL_WINDOWEVENT_EXPOSED,
    SDL_WINDOWEVENT_FOCUS_GAINED, SDL_WINDOWEVENT_FOCUS_LOST,
    SDL_WINDOWEVENT_HIDDEN, SDL_WINDOWEVENT_HIT_TEST, SDL_WINDOWEVENT_LEAVE,
    SDL_WINDOWEVENT_MAXIMIZED, SDL_WINDOWEVENT_MINIMIZED,
    SDL_WINDOWEVENT_MOVED, SDL_WINDOWEVENT_RESIZED, SDL_WINDOWEVENT_RESTORED,
    SDL_WINDOWEVENT_SHOWN, SDL_WINDOWEVENT_SIZE_CHANGED,
    SDL_WINDOWEVENT_TAKE_FOCUS,
  };

  #[non_exhaustive]
  pub enum WindowEvent {
    Shown { window_id: u32 },
    Hidden { window_id: u32 },
    Exposed { window_id: u32 },
    Maximized { window_id: u32 },
    Minimized { window_id: u32 },
    Restored { window_id: u32 },
    MouseEntered { window_id: u32 },
    MouseLeft { window_id: u32 },
    FocusGained { window_id: u32 },
    FocusLost { window_id: u32 },
    Close { window_id: u32 },
    TakeFocus { window_id: u32 },
    HitTest { window_id: u32 },
    Moved { window_id: u32, x: i32, y: i32 },
    Resized { window_id: u32, width: u32, height: u32 },
    SizeChanged { window_id: u32, width: u32, height: u32 },
  }

  impl TryFrom<SDL_WindowEvent> for WindowEvent {
    type Error = ();
    #[inline]
    #[must_use]
    fn try_from(window_event: SDL_WindowEvent) -> Result<Self, Self::Error> {
      let window_id = window_event.windowID;
      Ok(match window_event.event as SDL_WindowEventID {
        SDL_WINDOWEVENT_SHOWN => Self::Shown { window_id },
        SDL_WINDOWEVENT_HIDDEN => Self::Hidden { window_id },
        SDL_WINDOWEVENT_EXPOSED => Self::Exposed { window_id },
        SDL_WINDOWEVENT_MAXIMIZED => Self::Maximized { window_id },
        SDL_WINDOWEVENT_MINIMIZED => Self::Minimized { window_id },
        SDL_WINDOWEVENT_RESTORED => Self::Restored { window_id },
        SDL_WINDOWEVENT_ENTER => Self::MouseEntered { window_id },
        SDL_WINDOWEVENT_LEAVE => Self::MouseLeft { window_id },
        SDL_WINDOWEVENT_FOCUS_GAINED => Self::FocusGained { window_id },
        SDL_WINDOWEVENT_FOCUS_LOST => Self::FocusLost { window_id },
        SDL_WINDOWEVENT_CLOSE => Self::Close { window_id },
        SDL_WINDOWEVENT_TAKE_FOCUS => Self::TakeFocus { window_id },
        SDL_WINDOWEVENT_HIT_TEST => Self::HitTest { window_id },
        SDL_WINDOWEVENT_MOVED => {
          let x = window_event.data1 as i32;
          let y = window_event.data2 as i32;
          Self::Moved { window_id, x, y }
        }
        SDL_WINDOWEVENT_RESIZED => {
          let width = window_event.data1 as u32;
          let height = window_event.data2 as u32;
          Self::Resized { window_id, width, height }
        }
        SDL_WINDOWEVENT_SIZE_CHANGED => {
          let width = window_event.data1 as u32;
          let height = window_event.data2 as u32;
          Self::SizeChanged { window_id, width, height }
        }
        _ => return Err(()),
      })
    }
  }
}
