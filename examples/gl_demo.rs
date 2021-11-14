use beryllium::{
  event::Event,
  gl_window::{GlAttr, GlContextFlags, GlProfile},
  init::{InitFlags, Sdl},
  window::WindowFlags,
  SdlResult,
};
use core::{ptr::null, str};
use glitz::{println_gl_debug_callback, GlFns, GL_COLOR_BUFFER_BIT};
use zstring::zstr;

fn main() -> SdlResult<()> {
  let sdl = Sdl::init(InitFlags::EVERYTHING)?;
  sdl.allow_drop_events(true);

  const FLAGS: i32 = if cfg!(debug_assertions) {
    GlContextFlags::FORWARD_COMPATIBLE.as_i32() | GlContextFlags::DEBUG.as_i32()
  } else {
    GlContextFlags::FORWARD_COMPATIBLE.as_i32()
  };
  sdl.gl_set_attribute(GlAttr::MajorVersion, 3)?;
  sdl.gl_set_attribute(GlAttr::MinorVersion, 3)?;
  sdl.gl_set_attribute(GlAttr::Profile, GlProfile::Core as _)?;
  sdl.gl_set_attribute(GlAttr::Flags, FLAGS)?;

  let gl_win =
    sdl.create_gl_window(zstr!("GL Demo Window"), None, (800, 600), WindowFlags::ALLOW_HIGHDPI)?;
  gl_win.set_swap_interval(1)?;

  let gl = unsafe { GlFns::from_loader(&|zs| gl_win.get_proc_address(zs)).unwrap() };
  if gl_win.is_extension_supported(zstr!("GL_KHR_debug")) {
    println!("Activating the debug callback...");
    unsafe { gl.DebugMessageCallback(Some(println_gl_debug_callback), null()) };
  }

  gl.ClearColor(0.7, 0.6, 0.5, 1.0);

  'top: loop {
    // process all pending events
    while let Some(e) = sdl.poll_event() {
      match e {
        Event::Quit => break 'top,
        Event::MouseMotion { .. } => (),
        Event::Keyboard { .. } => (),
        Event::TextInput { text, .. } => {
          println!("TextInput: {:?}", str::from_utf8(&text));
        }
        other => println!("Event: {:?}", other),
      }
    }
    // now draw and swap

    gl.Clear(GL_COLOR_BUFFER_BIT);
    gl_win.swap_backbuffer();
  }

  Ok(())
}
