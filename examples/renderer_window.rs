use beryllium::{
  events::Event,
  init::InitFlags,
  video::{CreateWinArgs, RendererFlags},
  Sdl,
};

fn main() {
  // Initializes SDL2
  let sdl = Sdl::init(InitFlags::EVERYTHING);

  for info in sdl.get_renderer_driver_infos().unwrap() {
    println!("RendererDriver: {info:?}");
  }

  // Makes the window with an associated SDL Renderer.
  let win = sdl
    .create_renderer_window(
      CreateWinArgs { title: "Example Renderer Window", ..Default::default() },
      RendererFlags::ACCELERATED_VSYNC,
    )
    .unwrap();
  println!("Created The Renderer Window!");
  println!("Selected Renderer Info: {:?}", win.get_renderer_info());

  // program "main loop".
  'the_loop: loop {
    // Process events from this frame.
    #[allow(clippy::never_loop)]
    #[allow(clippy::single_match)]
    while let Some((event, _timestamp)) = sdl.poll_events() {
      match event {
        Event::Quit => break 'the_loop,
        _ => (),
      }
    }

    win.set_draw_color(u8::MAX, u8::MAX, u8::MAX, u8::MAX).unwrap();
    win.clear().unwrap();

    win.set_draw_color(0, 0, 0, u8::MAX).unwrap();
    win.draw_line(1, 1, 50, 50).unwrap();

    win.present();
  }

  // All the cleanup is handled by the various drop impls.
}
