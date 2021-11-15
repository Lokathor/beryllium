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
  println!("SDL2 will require the following extensions for surface creation:");
  for ext in vk_win.get_required_instance_extensions()?.into_iter() {
    println!("`{}`", ext);
  }
  // TODO: use ash or something to actually make the instance.

  'top: loop {
    // process all pending events
    while let Some(e) = sdl.poll_event() {
      match e {
        Event::Quit => break 'top,
        _ => (),
      }
    }
    // now draw and swap

    // TODO: draw and swap
  }

  Ok(())
}
