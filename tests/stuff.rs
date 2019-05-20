use beryllium::*;

#[test]
fn test_get_error() {
  // before it's used, the SDL2 error string is blank
  assert_eq!(&get_error(), "");
}
