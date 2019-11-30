use beryllium::*;
use core::time::Duration;

fn main() {
  let sdl = SDL::init(InitFlags::Everything).expect("couldn't init SDL");
  let _win = sdl
    .create_gl_window(
      "Basic Delay",
      WindowPosition::default(),
      800,
      600,
      fermium::SDL_WINDOW_SHOWN as u32,
    )
    .expect("couldn't open a window");
  sdl.delay_ms(Duration::from_secs(1).as_millis() as u32);
  sdl.delay_duration(Duration::from_secs(1));
}
