use core::convert::{TryFrom, TryInto};

use fermium::{
  SDL_Event, SDL_EventType, SDL_KEYDOWN, SDL_KEYUP, SDL_QUIT, SDL_WINDOWEVENT,
};

use crate::WindowID;

#[non_exhaustive]
pub enum Event {
  Quit,
  // TODO: DisplayEvent
  Window(WindowEvent),
  Keyboard(KeyboardEvent),
  /* TODO: TextEditing,
   * TODO: TextInput, */
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
        SDL_KEYDOWN | SDL_KEYUP => Event::Keyboard(sdl_event.key.into()),
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

  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  #[non_exhaustive]
  pub enum WindowEvent {
    Shown { window_id: WindowID },
    Hidden { window_id: WindowID },
    Exposed { window_id: WindowID },
    Maximized { window_id: WindowID },
    Minimized { window_id: WindowID },
    Restored { window_id: WindowID },
    MouseEntered { window_id: WindowID },
    MouseLeft { window_id: WindowID },
    FocusGained { window_id: WindowID },
    FocusLost { window_id: WindowID },
    Close { window_id: WindowID },
    TakeFocus { window_id: WindowID },
    HitTest { window_id: WindowID },
    Moved { window_id: WindowID, x: i32, y: i32 },
    Resized { window_id: WindowID, width: u32, height: u32 },
    SizeChanged { window_id: WindowID, width: u32, height: u32 },
  }

  impl TryFrom<SDL_WindowEvent> for WindowEvent {
    type Error = ();
    #[inline]
    #[must_use]
    fn try_from(window_event: SDL_WindowEvent) -> Result<Self, Self::Error> {
      let window_id = WindowID(window_event.windowID);
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

pub use keyboard_event::*;
mod keyboard_event {
  use super::*;
  use fermium::{SDL_KeyboardEvent, SDL_Keysym, SDL_PRESSED};
  //
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct Scancode(u32);
  //
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct Keycode(u32);
  //
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct KeyModifiers(u16);
  //
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct KeyboardEvent {
    pub window_id: WindowID,
    pub scancode: Scancode,
    pub keycode: Keycode,
    pub modifiers: KeyModifiers,
    pub is_pressed: bool,
    pub repeat: u8,
  }
  impl From<SDL_KeyboardEvent> for KeyboardEvent {
    #[inline]
    fn from(keyboard_event: SDL_KeyboardEvent) -> Self {
      Self {
        window_id: WindowID(keyboard_event.windowID),
        scancode: Scancode(keyboard_event.keysym.scancode as u32),
        keycode: Keycode(keyboard_event.keysym.sym as u32),
        modifiers: KeyModifiers(keyboard_event.keysym.mod_),
        is_pressed: keyboard_event.state as u32 == SDL_PRESSED,
        repeat: keyboard_event.repeat,
      }
    }
  }
  // TODO: Key constants
}
