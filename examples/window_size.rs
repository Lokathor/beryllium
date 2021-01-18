use beryllium::*;

fn main() {
  let sdl = SDL::init(InitFlags::Everything).expect("couldn't init SDL");
  let _win = sdl
    .create_gl_window(
      "Basic Window",
      WindowPosition::default(),
      800,
      600,
      WindowFlags::Shown | WindowFlags::Resizable,
    )
    .expect("couldn't open a window");

  _win.set_maximum_size(900, 900);
  _win.set_minimum_size(100, 100);

  loop {
    match sdl.poll_events().and_then(Result::ok) {
      // Quit event
      Some(Event::Quit(QuitEvent { .. })) => break,
      // Keyboard event
      Some(Event::Keyboard(KeyboardEvent {
        key: KeyInfo { keycode, .. },
        is_pressed,
        ..
      })) => {
        // If the key is pressed
        if is_pressed {
          // If the key is SPACE, set the window size to (1000, 1000)
          if keycode == Keycode::SPACE {
            _win.set_window_size(1000, 1000);
          }
          // If the key is RETURN, set the window set to (800, 600)
          if keycode == Keycode::RETURN {
            _win.set_window_size(800, 600)
          }
          // If the key is F11, toggle the fullscreen
          if keycode == Keycode::F11 {
            _win.set_fullscreen(!_win.is_fullscreen());
          }
        }
      }
      Some(Event::Window(WindowEvent { event, .. })) => match event {
        WindowEventEnum::Resized { w, h } => {
          println!("w: {}, h: {}", w, h);
        }
        _ => (),
      },
      _ => continue,
    }
  }
}
