use beryllium::*;

fn main() {
  let sdl = Sdl::init(InitFlags::EVERYTHING).unwrap();
  let rend_win = sdl
    .new_renderer_window(
      "Event Test",
      None,
      [800, 600],
      WindowCreationFlags::default(),
    )
    .unwrap();
  let mut controllers = vec![];
  let joystick_count = sdl.get_number_of_joysticks().unwrap();
  for n in 0 .. joystick_count {
    controllers.push(sdl.open_controller(n).unwrap())
  }
  'main: loop {
    while let Some((event, time)) = sdl.poll_event() {
      println!("[{time}] {event:?}", time = time, event = event);
      if matches!(event, Event::Quit) {
        break 'main;
      }
    }
    rend_win.clear().unwrap();
    rend_win.present();
  }
}
