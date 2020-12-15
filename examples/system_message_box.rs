use beryllium::show_buttons_message_box;

// We just show three message boxes and exit.
fn main() {
  println!(
    "{:?}",
    show_buttons_message_box(
      "System Color Scheme Information",
      "Please make a selection",
      &["Pizza", "Sushi", "Carrots"],
      true,
      Some(0),
      None,
    )
  );
}
