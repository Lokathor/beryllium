#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! This is an "Opening a window" demo.

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
      // process our event queue
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
    // draw those points into the surface
    let pitch = surface.pitch();
    unsafe {
      surface.lock_edit(|ptr| {
        for (x, y) in mouse_points.drain(..) {
          let row_ptr = ptr.offset((y * pitch) as isize) as *mut u32;
          row_ptr.offset(x as isize).write(core::u32::MAX);
        }
      })?;
    }
    // upload our surface data to the GPU by making a texture
    renderer.clear()?;
    {
      let texture = renderer.create_texture_from_surface(&surface)?;
      renderer.copy(&texture, None, None)?;
    }
    renderer.present();
  }

  Ok(())
}
