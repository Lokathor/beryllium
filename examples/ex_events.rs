use beryllium::*;

fn main() {
  let sdl = Sdl::init(InitFlags::EVERYTHING).unwrap();
  println!("{:?}", sdl);
}
