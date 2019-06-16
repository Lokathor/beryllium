use beryllium::*;

fn main() -> Result<(), String> {
  let sdl = unsafe { init()? };

  let joystick_count = sdl.number_of_joysticks()?;
  if joystick_count > 0 {
    for index in 0..joystick_count {
      let id = JoystickID(index);
      if sdl.joystick_is_game_controller(id) {
        match sdl.controller_name(id) {
          Some(name) => println!("{:?} has controller name: {}", id, name),
          None => println!("{:?} is a controller with no name", id),
        }
      } else {
        println!("{:?} doesn't support the controller API.", id);
      }
    }
  } else {
    println!("There are no joysticks connected.");
    println!("Please plug at least one in and run the example again.");
  }

  Ok(())
}
