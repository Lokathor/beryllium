use super::*;

#[derive(Debug, Clone, Copy)]
pub struct QuitEvent {
  pub timestamp: u32,
}

impl From<fermium::SDL_QuitEvent> for QuitEvent {
  fn from(ev: fermium::SDL_QuitEvent) -> Self {
    QuitEvent { timestamp: ev.timestamp }
  }
}

impl From<QuitEvent> for fermium::SDL_QuitEvent {
  fn from(ev: QuitEvent) -> Self {
    fermium::SDL_QuitEvent {
      type_: fermium::SDL_QUIT as u32,
      timestamp: ev.timestamp,
    }
  }
}
