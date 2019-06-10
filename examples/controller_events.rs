#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// XXX finish me

use beryllium::*;
use std::collections::HashMap;

fn main() -> Result<(), String> {
  let sdl = unsafe { beryllium::init()? };

  let _window = sdl.create_window(
    "Controller Event Demo",                 // title
    WINDOW_POSITION_CENTERED,                // x
    WINDOW_POSITION_CENTERED,                // y
    800,                                     // width
    600,                                     // height
    WindowFlags::default().with_shown(true), // flags
  )?;


  let mut controllers: Vec<Option<Controller<'_>>> = Vec::new();
  let mut instance_id_to_idx: HashMap<i32, usize> = HashMap::new();
  println!("Plug in a controller");

  'game_loop: loop {
    while let Some(event) = sdl.poll_event() {
      match event {
        Event::Quit { timestamp } => {
          println!("Quitting the program after {} milliseconds.", timestamp);
          break 'game_loop;
        }
        Event::ControllerAdded { device_index, .. } => {
          println!("Controller added: {}", device_index);
          let idx = device_index as usize;
          if controllers.len() <= idx {
            controllers.resize_with(idx + 1, || None);
          }

          if controllers[idx].is_some() {
            // This seems to double-fire sometimes for me. Might be Mac-specific...
            eprintln!("  Device already exists at index {}!?", idx);
            continue;
          }

          let controller = match sdl.open_controller(device_index) {
            Ok(controller) => controller,
            Err(e) => {
              eprintln!("Failed to open controller: {}", e);
              continue;
            }
          };

          let instance_id = match controller.get_instance_id() {
            Ok(id) => id,
            Err(e) => {
              eprintln!("Failed to get instance id: {}", e);
              continue;
            }
          };

          println!("  with instance id: {}", instance_id);

          controllers[idx] = Some(controller);

          instance_id_to_idx.insert(instance_id, idx);
        }

        Event::ControllerRemoved { instance_id, .. } => {
          let idx = if let Some(idx) = instance_id_to_idx.remove(&instance_id) {
            idx
          } else {
            eprintln!("Unknown controller instance id: {}?!", instance_id);
            continue;
          };
          println!("Removing controller {} with device index {}", instance_id, idx);
          controllers[idx] = None;
        }
        _ => (),
      }
    }
  }

  Ok(())
}
