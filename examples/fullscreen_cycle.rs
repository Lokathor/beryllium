#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! A demo about how to use a Surface (CPU-side image memory).
//!
//! Our demo is that we'll store where the mouse goes, and turn those pixels
//! white, so you see a "trail" of sorts.

use beryllium::*;

fn main() -> Result<(), String> {
  let sdl = unsafe { beryllium::init()? };

  let window = sdl.create_window(
    "Surface Demo",                          // title
    WINDOW_POSITION_CENTERED,                // x
    WINDOW_POSITION_CENTERED,                // y
    800,                                     // width
    600,                                     // height
    WindowFlags::default().with_shown(true), // flags
  )?;
  let mut style = FullscreenStyle::Windowed;

  'game_loop: loop {
    while let Some(event) = sdl.poll_event() {
      match event {
        Event::Quit { timestamp } => {
          println!("Quitting the program after {} milliseconds.", timestamp);
          break 'game_loop;
        }
        // pressing F11
        Event::Keyboard {
          key: KeyInfo {
            keycode: Some(Keycode::F11),
            ..
          },
          is_key_down: true,
          ..
        } => {
          if style == FullscreenStyle::Windowed {
            style = FullscreenStyle::Fullscreen;
          } else if style == FullscreenStyle::Fullscreen {
            style = FullscreenStyle::FullscreenDesktop;
          } else {
            style = FullscreenStyle::Windowed;
          }
          window.set_fullscreen_style(style)?;
        }
        // TODO: fullscreen cycle here!
        _ => (),
      }
    }
  }

  Ok(())
}
