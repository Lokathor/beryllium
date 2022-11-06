use beryllium::{events::Event, init::InitFlags, video::CreateWinArgs, Sdl};

fn main() {
  // Initializes SDL2
  let sdl = Sdl::init(InitFlags::EVERYTHING);

  // TODO: set our GL attributes

  // Makes the window with a GL Context.
  let win = sdl
    .create_gl_window(CreateWinArgs { title: "Example GL Window", ..Default::default() })
    .unwrap();
  println!("GL window size: {:?}", win.get_window_size());
  println!("GL drawable size: {:?}", win.get_drawable_size());

  let mut controllers = Vec::new();

  // program "main loop".
  'the_loop: loop {
    // Process events from this frame.
    #[allow(clippy::never_loop)]
    while let Some((event, _timestamp)) = sdl.poll_events() {
      println!("{event:?}");
      if event == Event::Quit {
        break 'the_loop;
      }
      if let Event::ControllerAdded { index } = event {
        match sdl.open_game_controller(index) {
          Ok(controller) => {
            println!("Opened `{}`", controller.get_name());
            controllers.push(controller);
          }
          Err(msg) => println!("Couldn't open {index}: {msg:?}"),
        }
      }
    }

    // TODO: post-events drawing

    // TODO: swap buffers.
  }

  // All the cleanup is handled by the various drop impls.
}
