#![no_std]
#![warn(missing_docs)]

//! Easy to use wrapper of the [`fermium`](https://docs.rs/fermium) crate.

use core::ptr::null_mut;

extern crate alloc;

use alloc::{sync::Arc, vec::Vec};

use fermium::c_char;

macro_rules! err_guard {
  ($is_err:expr) => {
    if $is_err {
      return Err(crate::get_error());
    }
  };
}

macro_rules! impl_bit_ops_for_tuple_newtype {
  ($t:ty) => {
    impl core::ops::BitAnd for $t {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
      }
    }
    impl core::ops::BitAndAssign for $t {
      #[inline]
      #[must_use]
      fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
      }
    }
    impl core::ops::BitOr for $t {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
      }
    }
    impl core::ops::BitOrAssign for $t {
      #[inline]
      #[must_use]
      fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
      }
    }
    impl core::ops::BitXor for $t {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
      }
    }
    impl core::ops::BitXorAssign for $t {
      #[inline]
      #[must_use]
      fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
      }
    }
    impl core::ops::Not for $t {
      type Output = Self;
      #[inline]
      #[must_use]
      fn not(self) -> Self::Output {
        Self(!self.0)
      }
    }
  };
}

fn make_null_str(s: &str) -> Vec<c_char> {
  s.as_bytes().iter().copied().map(|u| u as c_char).chain(Some(0)).collect()
}

mod error;
pub use error::*;

mod init;
pub use init::*;

/// Handle to the SDL2 API.
pub struct Sdl(Arc<Init>);

impl Sdl {
  /// Initialize SDL2 according to the flags given.
  ///
  /// * You can't double-initialize SDL2.
  pub fn init(flags: InitFlags) -> BerylliumResult<Self> {
    let init = Init::new(flags)?;
    Ok(Self(Arc::new(init)))
  }
}

/// Stylization for a message box.
#[allow(missing_docs)]
pub enum MessageBoxStyle {
  Error,
  Warning,
  Information,
}

/// Show a simple message box (title + message).
///
/// * The message box isn't modal to any window.
/// * This blocks until the message box is closed.
/// * There is only one button ("okay") for the user to click.
/// * The `style` value changes the icon that goes with the message box, but the
///   details vary by OS.
pub fn show_simple_message_box(
  title: &str, message: &str, style: MessageBoxStyle,
) -> BerylliumResult<()> {
  use fermium::messagebox::*;

  let flags = match style {
    MessageBoxStyle::Error => SDL_MESSAGEBOX_ERROR,
    MessageBoxStyle::Warning => SDL_MESSAGEBOX_WARNING,
    MessageBoxStyle::Information => SDL_MESSAGEBOX_INFORMATION,
  };
  let title_null = make_null_str(title);
  let message_null = make_null_str(message);
  let i = unsafe {
    SDL_ShowSimpleMessageBox(
      flags,
      title_null.as_ptr(),
      message_null.as_ptr(),
      null_mut(),
    )
  };
  err_guard!(i < 0);
  Ok(())
}

/// Show a message box with a list of buttons you provide.
///
/// * The message box isn't modal to any window.
/// * This blocks until the message box is closed.
/// * The `buttons` is a list of button texts and ID values.
/// * `buttons_left_to_right` sets if the buttons should be given left to right
///   (otherwise they are right to left).
/// * `return_default` is the index of the button that the return key should
///   default to selecting, if any.
/// * `escape_default` is the index of the button that the escape key should
///   default to selecting, if any. This will also be selected if the message
///   box is forced to close via other means, such as the user selecting "close
///   window" in the taskbar.
///
/// **Returns:**
/// * Ok: The index of the button that was clicked, or `usize::MAX` if the
///   message box was closed without any button being selected and there is no
///   `escape_default` given.
/// * Err: The error that occurred when trying to show the message box.
pub fn show_buttons_message_box(
  title: &str, message: &str, buttons: &[&str], buttons_left_to_right: bool,
  return_default: Option<usize>, escape_default: Option<usize>,
) -> BerylliumResult<usize> {
  use fermium::messagebox::*;

  assert!(buttons.len() <= i32::MAX as usize);

  let flags = if buttons_left_to_right {
    SDL_MESSAGEBOX_BUTTONS_LEFT_TO_RIGHT
  } else {
    SDL_MESSAGEBOX_BUTTONS_RIGHT_TO_LEFT
  };
  let title_null = make_null_str(title);
  let message_null = make_null_str(message);
  let buttons_null: Vec<Vec<c_char>> =
    buttons.iter().map(|s| make_null_str(s)).collect();
  let mut button_data: Vec<SDL_MessageBoxButtonData> = buttons_null
    .iter()
    .enumerate()
    .map(|(i, text)| SDL_MessageBoxButtonData {
      flags: SDL_MessageBoxButtonFlags(0),
      buttonid: i as i32,
      text: text.as_ptr(),
    })
    .collect();
  if let Some(i) = return_default {
    button_data[i].flags |= SDL_MESSAGEBOX_BUTTON_RETURNKEY_DEFAULT;
  };
  if let Some(i) = escape_default {
    button_data[i].flags |= SDL_MESSAGEBOX_BUTTON_ESCAPEKEY_DEFAULT;
  };

  let data = SDL_MessageBoxData {
    flags,
    window: null_mut(),
    title: title_null.as_ptr(),
    message: message_null.as_ptr(),
    numbuttons: button_data.len() as _,
    buttons: button_data.as_ptr(),
    colorScheme: null_mut(),
  };
  let mut clicked_id = 0;
  let i = unsafe { SDL_ShowMessageBox(&data, &mut clicked_id) };
  err_guard!(i < 0);
  Ok(clicked_id as isize as usize)
}
