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

  // program "main loop".
  'the_loop: loop {
    // Process events from this frame.
    #[allow(clippy::never_loop)]
    while let Some((event, _timestamp)) = sdl.poll_events() {
      println!("{event:?}");
      if event == Event::Quit {
        break 'the_loop;
      }
    }

    // TODO: post-events drawing

    // TODO: swap buffers.
  }

  // All the cleanup is handled by the various drop impls.
}
