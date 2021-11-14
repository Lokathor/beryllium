use beryllium::{
  event::Event,
  init::{InitFlags, Sdl},
  window::WindowFlags,
  SdlResult,
};
use core::str;
use zstring::zstr;

fn main() -> SdlResult<()> {
  let sdl = Sdl::init(InitFlags::EVERYTHING)?;
  sdl.allow_drop_events(true);

  let vk_win =
    sdl.create_vk_window(zstr!("VK Demo Window"), None, (800, 600), WindowFlags::ALLOW_HIGHDPI)?;
  #[allow(non_snake_case)]
  let _vkGetInstanceProcAddr = vk_win.get_vkGetInstanceProcAddr()?;
  // TODO: use ash or something to actually make the instance.

  'top: loop {
    // process all pending events
    while let Some(e) = sdl.poll_event() {
      match e {
        Event::Quit => break 'top,
        Event::MouseMotion { .. } => (),
        Event::Keyboard { .. } => (),
        Event::TextInput { text, .. } => {
          println!("TextInput: {:?}", str::from_utf8(&text));
        }
        other => println!("Event: {:?}", other),
      }
    }
    // now draw and swap

    // TODO: draw and swap
  }

  Ok(())
}
