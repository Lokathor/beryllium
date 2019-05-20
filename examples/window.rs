#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! This is an "Opening a window" demo.

use beryllium::*;

fn main() {
  // Safety Rules: You must only init SDL2 from the main thread, and you must
  // not double initialize it.
  let sdl = unsafe { beryllium::init() }.unwrap();

  let _window = sdl
    .create_window(
      "Window Demo",                           // title
      WINDOW_POSITION_CENTERED,                // x
      WINDOW_POSITION_CENTERED,                // y
      800,                                     // width
      600,                                     // height
      WindowFlags::default().with_shown(true), // flags
    )
    .unwrap();

  'game_loop: loop {
    // At the top of every frame you process your events. You MUST process
    // events promptly or you can hang the operating system's UI.
    while let Some(event) = sdl.poll_event() {
      match event {
        Event::Quit { timestamp } => {
          println!("Quitting the program after {} milliseconds.", timestamp);
          break 'game_loop;
        }
        _ => (),
      }
    }
    // then you _would_ do the rest of your program here.
  }
}
