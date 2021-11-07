use beryllium::{
  init::{InitFlags, Sdl},
  window::WindowFlags,
};
use zstring::zstr;

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

  // TODO: ability to destroy the window?
}
