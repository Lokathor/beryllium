use beryllium::{show_simple_message_box, MessageBoxStyle};

// We just show three message boxes and exit.
fn main() {
  show_simple_message_box(
    "Information",
    "Here is some information.",
    MessageBoxStyle::Information,
  )
  .unwrap();
  show_simple_message_box(
    "Warning",
    "Here is a warning.",
    MessageBoxStyle::Warning,
  )
  .unwrap();
  show_simple_message_box("Error", "And now an error.", MessageBoxStyle::Error)
    .unwrap();
}
