use beryllium::{
  events::Event,
  init::InitFlags,
  video::{CreateWinArgs, GlProfile},
  Sdl,
};

fn main() {
  // Initializes SDL2
  let sdl = Sdl::init(InitFlags::EVERYTHING);

  // This part asks for an ES 3.1 context "just for fun", because that's what
  // works best between Windows and also Raspberry Pi. Mac doesn't support ES
  // contexts, but this is just a demo so for Mac we'll skip any configuration
  // at all and just get some "don't care" GL context.
  #[cfg(not(target_os = "macos"))]
  {
    sdl.set_gl_profile(GlProfile::ES).unwrap();
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(1).unwrap();
  }

  // Makes the window with a GL Context.
  let win = sdl
    .create_gl_window(CreateWinArgs { title: "Example GL Window", ..Default::default() })
    .unwrap();
  println!("GL window size: {:?}", win.get_window_size());
  println!("GL drawable size: {:?}", win.get_drawable_size());
  println!("GL_KHR_debug supported: {}", win.supports_extension("GL_KHR_debug"));

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
