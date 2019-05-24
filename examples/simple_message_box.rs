#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! This is an "Opening a window" demo.

use beryllium::*;

fn main() -> Result<(), String> {
  show_simple_message_box(
    MessageBox::Information,
    "Example: Simple Message Box",
    "This is the message box you asked for.",
    None,
  )
}
