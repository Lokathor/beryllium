// This line makes your release build program not have a dummy terminal attached
// to it on Win32. It has no effect on the other operating systems.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! This is an "Opening a window" demo.

use beryllium::*;

fn main() {
  let sdl = unsafe { beryllium::init() }.unwrap();

  let window = sdl
    .create_window(
      "Window Demo",
      None,
      800,
      600,
      WindowFlags::default().with_shown(true),
    )
    .unwrap();

  'game_loop: loop {
    // At the top of every frame you process your events. You MUST process
    // events or you can hang the operating system's UI.
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
