#![allow(missing_docs)]

use super::*;
use fermium::{SDL_Event, SDL_EventType};

// Note: sub-modules are named after the `SDL_Event` union variant that they
// handle.

mod quit;
pub use quit::*;
mod window;
pub use window::*;
mod motion;
pub use motion::*;
mod button;
pub use button::*;
mod wheel;
pub use wheel::*;
mod key;
pub use key::*;

pub enum Event {
  Quit(QuitEvent),
  Window(WindowEvent),
  MouseMotion(MouseMotionEvent),
  MouseButton(MouseButtonEvent),
  MouseWheel(MouseWheelEvent),
  Keyboard(KeyboardEvent),
  // TODO
}
impl TryFrom<SDL_Event> for Event {
  type Error = SDL_Event;

  fn try_from(ev: SDL_Event) -> Result<Self, SDL_Event> {
    Ok(unsafe {
      match ev.type_ as SDL_EventType {
        fermium::SDL_QUIT => Event::Quit(QuitEvent::from(ev.quit)),
        fermium::SDL_WINDOWEVENT => {
          Event::Window(if let Ok(we) = WindowEvent::try_from(ev.window) {
            we
          } else {
            return Err(ev);
          })
        }
        fermium::SDL_MOUSEMOTION => {
          Event::MouseMotion(MouseMotionEvent::from(ev.motion))
        }
        fermium::SDL_MOUSEBUTTONDOWN | fermium::SDL_MOUSEBUTTONUP => {
          Event::MouseButton(MouseButtonEvent::from(ev.button))
        }
        fermium::SDL_MOUSEWHEEL => {
          Event::MouseWheel(MouseWheelEvent::from(ev.wheel))
        }
        fermium::SDL_KEYDOWN | fermium::SDL_KEYUP => {
          Event::Keyboard(KeyboardEvent::from(ev.key))
        }
        _ => return Err(ev),
      }
    })
  }
}
