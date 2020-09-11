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
