use beryllium::*;

fn main() {
  let sdl = SDL::init(InitFlags::Everything).expect("couldn't init SDL");
  let _win = sdl
    .create_gl_window(
      "Mouse Event Test",
      WindowPosition::default(),
      800,
      600,
      fermium::SDL_WINDOW_SHOWN as u32,
    )
    .expect("couldn't open a window");

  loop {
    match sdl.poll_events().and_then(Result::ok) {
      Some(Event::Quit(QuitEvent { .. })) => break,
      Some(Event::MouseMotion(motion)) => println!("motion: {:?}", motion),
      Some(Event::MouseButton(button)) => {
        println!("button: {:?}", button);
        if button.button == MouseButton::Left {
          println!("Left!");
        }
      },
      Some(Event::MouseWheel(wheel)) => println!("wheel: {:?}", wheel),
      _ => continue,
    }
  }
}
