#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Demo of how to setup a blank window using the `gl` crate for OpenGL.

use beryllium::*;
use gl;

fn main() -> Result<(), String> {
  // Init SDL2
  let sdl = unsafe { beryllium::init() }?;

  // Make a window (include the flag for OpenGL support!)
  let window = sdl.create_window(
    "Extern Crate: `gl`",                                      // title
    WINDOW_POSITION_CENTERED,                                  // x
    WINDOW_POSITION_CENTERED,                                  // y
    800,                                                       // width
    600,                                                       // height
    WindowFlags::default().with_shown(true).with_opengl(true), // flags
  )?;

  // Set our context request settings and make a context (which is automatically
  // made current for you).
  sdl.gl_set_attribute(GLattr::ContextProfileMask, CONTEXT_PROFILE_CORE);
  sdl.gl_set_attribute(GLattr::ContextMajorVersion, 3);
  sdl.gl_set_attribute(GLattr::ContextMinorVersion, 3);
  let _ctx = unsafe { window.gl_create_context()? };

  // ONCE WE HAVE A CONTEXT (not before!) we can load up the OpenGL functions
  gl::load_with(|s| unsafe { sdl.gl_get_proc_address(s) });

  unsafe { gl::ClearColor(1.0, 0.5, 0.0, 1.0) };

  'game_loop: loop {
    while let Some(event) = sdl.poll_event() {
      #[allow(clippy::single_match)]
      match event {
        Event::Quit { timestamp } => {
          println!("Quitting the program after {} milliseconds.", timestamp);
          break 'game_loop;
        }
        _ => (),
      }
    }

    // Here is where all your fancy OpenGL drawing can go. In this demo we just
    // clear the screen over and over.
    unsafe {
      gl::Clear(gl::COLOR_BUFFER_BIT);
      window.gl_swap_window();
    };
  }

  Ok(())
}
