#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! A demo for using a fullscreen window.
//!
//! Press F11 to cycle through the modes.

use beryllium::*;

fn main() -> Result<(), String> {
  let sdl = beryllium::init()?;

  let window = sdl.create_window(
    "Surface Demo",           // title
    WINDOW_POSITION_CENTERED, // x
    WINDOW_POSITION_CENTERED, // y
    800,                      // width
    600,                      // height
    WindowFlags::default(),   // flags
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
          key_info: KeyInfo { keycode: Some(Keycode::F11), .. },
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
