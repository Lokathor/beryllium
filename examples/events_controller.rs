use beryllium::*;

fn main() {
  let sdl = SDL::init(InitFlags::Everything).expect("couldn't init SDL");
  let _win = sdl
    .create_gl_window(
      "Keyboard Event Test",
      WindowPosition::default(),
      800,
      600,
      fermium::SDL_WINDOW_SHOWN as u32,
    )
    .expect("couldn't open a window");

  let mut controllers = Vec::new();
  for id in 0 .. sdl.num_joysticks().expect("couldn't check joystick count") {
    if sdl.is_game_controller(id) {
      println!("trying to open {}...", id);
      match sdl.open_game_controller(id) {
        Ok(controller) => controllers.push(controller),
        Err(msg) => println!("Err opening controller: {}", msg),
      }
    }
  }

  loop {
    match sdl.poll_events().and_then(Result::ok) {
      Some(Event::Quit(QuitEvent { .. })) => break,
      Some(Event::ControllerDevice(cdevice)) => println!("cdevice: {:?}", cdevice),
      Some(Event::ControllerButton(cbutton)) => println!("cbutton: {:?}", cbutton),
      Some(Event::ControllerAxis(caxis)) => println!("caxis: {:?}", caxis),
      _ => continue,
    }
  }
}
