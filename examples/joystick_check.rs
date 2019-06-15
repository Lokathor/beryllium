use beryllium::*;

fn main() -> Result<(), String> {
  let sdl = unsafe { init()? };

  let joystick_count = sdl.number_of_joysticks()?;
  if joystick_count > 0 {
    for index in 0..joystick_count {
      if sdl.joystick_is_game_controller(JoystickID(index)) {
        match sdl.controller_name(index) {
          Some(name) => println!("Joystick #{} has controller name: {}", index, name),
          None => println!("Joystick #{} is a controller with no name", index),
        }
      } else {
        println!("Joystick #{} doesn't support the controller API.", index);
      }
    }
  } else {
    println!("There are no joysticks connected.");
    println!("Please plug at least one in and run the example again.");
  }

  Ok(())
}
