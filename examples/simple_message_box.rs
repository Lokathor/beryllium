#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! A demo that shows off simple message boxes.

use beryllium::*;

fn main() -> Result<(), String> {
  // Safety Rules: We can only affect the GUI from the main thread (a macOS
  // limitation), and this affects the GUI, so we can only call this from the
  // main thread.
  unsafe {
    // If you use the top level function for it you get a message box that is
    // not modal to any window. However, it's still a blocking operation. This
    // ability is intended for if you need to display a message to the user
    // without yet having a window.
    lone_message_box(
      MessageBox::Information,
      "Example: Simple Message Box",
      "This message box stands alone.",
    )?
  }

  // This is the same as the `window` example
  let sdl = beryllium::init()?;
  let window = sdl.create_window(
    "Simple Message Box Window",             // title
    WINDOW_POSITION_CENTERED,                // x
    WINDOW_POSITION_CENTERED,                // y
    800,                                     // width
    600,                                     // height
    WindowFlags::default().with_shown(true), // flags
  )?;

  // We can also make a message box as a Window method, which makes message
  // boxes that are modal to that window.
  window.modal_message_box(
    MessageBox::Information,
    "Example: Modal Simple Message Box",
    "This message box is modal to the parent window.",
  )
}
