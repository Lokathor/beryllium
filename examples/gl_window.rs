use core::ptr::NonNull;

use beryllium::{
  event::Event,
  init::{InitFlags, Sdl},
  window::WindowFlags,
  SdlResult,
};
use glitz::{GlFns, GL_COLOR_BUFFER_BIT};
use zstring::{zstr, ZStr};

fn main() -> SdlResult<()> {
  let sdl = Sdl::init(InitFlags::EVERYTHING)?;

  // TODO: configure GL before creating the window and context.

  let gl_win = sdl.create_gl_window(
    zstr!("GL Window Demo"),
    (100, 100),
    (800, 600),
    WindowFlags::ALLOW_HIGHDPI,
  )?;
  gl_win.set_swap_interval(1)?;

  let gl = unsafe {
    GlFns::from_loader(&|p| {
      let nn = NonNull::new(p as _).unwrap();
      gl_win.get_proc_address(ZStr::from_non_null_unchecked(nn))
    })
    .unwrap()
  };

  gl.ClearColor(0.7, 0.6, 0.5, 1.0);

  'top: loop {
    // process all pending events
    while let Some(e) = sdl.poll_event() {
      match e {
        Event::Quit => break 'top,
        other => println!("unhandled event: {:?}", other),
      }
    }
    // now draw and swap

    gl.Clear(GL_COLOR_BUFFER_BIT);
    gl_win.swap_backbuffer();
  }

  Ok(())
}
