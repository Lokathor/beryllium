use core::ptr::NonNull;

use beryllium::{
  event::Event,
  init::{InitFlags, Sdl},
  window::WindowFlags,
};
use glitz::{GlFns, GL_COLOR_BUFFER_BIT};
use zstring::{zstr, ZStr};

fn main() {
  let sdl = Sdl::init(InitFlags::EVERYTHING).unwrap();
  let win = sdl
    .create_window(
      zstr!("GL Window Demo"),
      100,
      100,
      800,
      600,
      WindowFlags::OPENGL | WindowFlags::ALLOW_HIGHDPI,
    )
    .unwrap();

  // TODO: configure GL before creating the context.

  let gl_ctx = unsafe { sdl.gl_create_context(&win).unwrap() };
  // TODO: set vsync?

  let gl = unsafe {
    GlFns::from_loader(&|p| {
      let nn = NonNull::new(p as _).unwrap();
      gl_ctx.get_proc_address(ZStr::from_non_null_unchecked(nn))
    })
    .unwrap()
  };

  gl.ClearColor(0.7, 0.6, 0.5, 1.0);

  'top: loop {
    // process all pending events
    while let Some(e) = sdl.poll_event() {
      match e {
        Event::Quit => break 'top,
      }
    }
    // now draw and swap

    gl.Clear(GL_COLOR_BUFFER_BIT);
    gl_ctx.swap_window(&win);
  }

  // TODO: ability to destroy the window?
}
