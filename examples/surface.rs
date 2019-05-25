#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! A demo about how to use a Surface.
//!
//! Our "demo" is that we'll store where the mouse goes, and turn those pixels
//! white, so you see a "trail" of sorts.

use beryllium::*;

fn main() -> Result<(), String> {
  let sdl = unsafe { beryllium::init()? };

  let window = sdl.create_window(
    "Surface Demo",                          // title
    WINDOW_POSITION_CENTERED,                // x
    WINDOW_POSITION_CENTERED,                // y
    800,                                     // width
    600,                                     // height
    WindowFlags::default().with_shown(true), // flags
  )?;

  let mut surface = sdl.create_rgb_surface(800, 600, SurfaceFormat::DIRECT32_DEFAULT)?;
  let pitch = surface.pitch();

  // Safety Rules: Each renderer goes with exactly one Window, and you can't use
  // them with the wrong Window. Similarly, Textures come from a Renderer, and
  // you can't use a texture with the wrong Renderer. If you only make a single
  // Renderer that's easy to do. If you make more than one it's up to you to
  // keep it straight.
  let renderer = unsafe {
    window.create_renderer(
      None,
      RendererFlags::default()
        .with_accelerated(true)
        .with_present_vsync(true),
    )?
  };

  let mut mouse_points = vec![];

  'game_loop: loop {
    mouse_points.clear();
    while let Some(event) = sdl.poll_event() {
      match event {
        Event::Quit { timestamp } => {
          println!("Quitting the program after {} milliseconds.", timestamp);
          break 'game_loop;
        }
        Event::MouseMotion { x, y, .. } => {
          mouse_points.push((x, y));
        }
        _ => (),
      }
    }
    // Safety Rules: We have to lock the surface before it's safe to edit the
    // pixel data directly. We can't pass store this pointer past the closure's
    // use, we also must follow standard 2D pixel buffer editing rules, not go
    // out of bounds, etc. The biggest problem is that the ptr has to be a byte
    // pointer since the pixel format can't easily be known at the type level (I
    // mean it's possible, but would make a huge type-explosion thing that's not
    // really worth it).
    unsafe {
      surface.lock_edit(|ptr| {
        for (x, y) in mouse_points.drain(..) {
          let row_ptr = ptr.offset((y * pitch) as isize) as *mut u32;
          row_ptr.offset(x as isize).write(core::u32::MAX);
        }
      })?;
    }
    renderer.clear()?;
    {
      let texture = renderer.create_texture_from_surface(&surface)?;
      renderer.copy(&texture, None, None)?;
    }
    renderer.present();
  }

  Ok(())
}
