use super::*;

/// An abstract RGBA color value.
///
/// * Each channel ranges from 0 (none) to 255 (maximum).
/// * Alpha channel is "opacity", so 255 is opaque.
///
/// A color value's exact representation within an image depends on the
/// [PixelFormat] of the image. You can use the "get" and "map" methods of a
/// `PixelFormat` to convert between raw pixel data and a `Color` value. Note
/// that any `PixelFormat` that's less than 32 bits per pixel will lose
/// information when you go from `Color` to raw pixel value, so the conversion
/// isn't always reversible.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(C)]
pub struct Color {
  /// Red
  pub r: u8,
  /// Green
  pub g: u8,
  /// Blue
  pub b: u8,
  /// Alpha / opacity
  pub a: u8,
}
impl From<SDL_Color> for Color {
  fn from(other: SDL_Color) -> Self {
    Self {
      r: other.r,
      g: other.g,
      b: other.b,
      a: other.a,
    }
  }
}
impl From<Color> for SDL_Color {
  fn from(other: Color) -> Self {
    Self {
      r: other.r,
      g: other.g,
      b: other.b,
      a: other.a,
    }
  }
}

/// A palette of [Color] values.
///
/// The way that the `Palette` type works is slightly different from Rust's
/// normal ownership model, so please pay attention as I explain.
///
/// A `Palette` value holds a pointer to a heap allocated [SDL_Palette]
/// ([wiki](https://wiki.libsdl.org/SDL_Palette)). That `SDL_Palette` has a
/// pointer to the heap allocated `Color` values, along with a length, reference
/// count, and version number.
///
/// When you set a `Palette` on a [Surface] or [PixelFormat] it moves some
/// pointers and adjusts the reference count of the `Palette`. Now you have the
/// `Palette`, and _also_ that thing has the same `Palette`. An edit to the
/// `Palette` data in either location will affect everyone's data. Having a
/// `&mut Palette` does _not_ mean that you have an exclusive path of access to
/// the `Palette` contents.
///
/// As a result, I cannot allow you to _ever_ construct a shared reference or
/// unique reference to the `Color` data held inside the `Palette`. This means
/// no [Deref](core::ops::Deref), [Index](core::ops::Index), or
/// [IndexMut](core::ops::IndexMut), no Iterators of any kind, none of that.
/// This definitely makes the API of the `Palette` type not quite as fun as you
/// might like.
///
/// You can allocate a `Palette` by calling [SDLToken::new_palette] and
/// specifying how many `Color` values the `Palette` should hold. However, you
/// generally do not need to do this yourself, because if a `Surface` or
/// `PixelFormat` needs palette data it will automatically allocate a palette of
/// the correct size when it is created.
///
/// All slots in a new `Palette` are initialized to opaque white (`0xFF` in all
/// four color channels).
#[derive(Debug)] // TODO: We probably want a custom Debug impl
#[repr(transparent)]
pub struct Palette<'sdl> {
  pub(crate) nn: NonNull<SDL_Palette>,
  pub(crate) _marker: PhantomData<&'sdl SDLToken>,
}

impl SDLToken {
  /// Allocates a new [Palette] with the number of color slots given.
  ///
  /// The initial value of the palette color values is `0xFF` in all four
  /// channels (opaque white).
  pub fn new_palette(&self, color_count: usize) -> Result<Palette<'_>, String> {
    let max = core::i32::MAX as usize;
    if color_count > max {
      return Err("beryllium error: color_count > i32::MAX".to_string());
    }
    if color_count < 2 {
      return Err("beryllium error: color_count of a palette must be at least 2".to_string());
    }
    match NonNull::new(unsafe { SDL_AllocPalette(color_count as i32) }) {
      Some(nn) => Ok(Palette {
        nn,
        _marker: PhantomData,
      }),
      None => Err(get_error()),
    }
  }
}

impl Drop for Palette<'_> {
  fn drop(&mut self) {
    unsafe { SDL_FreePalette(self.nn.as_ptr()) }
  }
}

#[allow(clippy::len_without_is_empty)]
impl Palette<'_> {
  /// Gets the number of colors in the Palette
  pub fn len(&self) -> usize {
    unsafe { (*self.nn.as_ptr()).ncolors as usize }
  }

  /// Assigns a slice of colors into the `Palette`, starting at the position
  /// specified.
  ///
  /// Colors that don't "fit" because they would trail off the end are not copied.
  ///
  /// ## Failure
  ///
  /// * `start` values >= the length will give an error.
  pub fn set_colors(&self, start: usize, new_colors: &[Color]) -> Result<(), String> {
    if start >= self.len() {
      return Err("beryllium error: start index out of bounds".to_string());
    }
    // Note(Lokathor): We'll manually clip the input length instead of relying
    // on SDL2's dubious clipping process.
    let clipped_length = (self.len() - start).min(new_colors.len());
    debug_assert!(start + clipped_length <= self.len());
    let out = unsafe {
      SDL_SetPaletteColors(
        self.nn.as_ptr(),
        new_colors.as_ptr() as *const SDL_Color,
        start as i32,
        clipped_length as i32,
      )
    };
    if out == 0 {
      Ok(())
    } else {
      // Given our previous checks, this path should never happen.
      Err(get_error())
    }
  }

  /// Gets the [Color] at the index specified.
  ///
  /// ## Failure
  ///
  /// * `None` if the index is out of bounds.
  pub fn get_color(&self, index: usize) -> Option<Color> {
    if index >= self.len() {
      None
    } else {
      Some(unsafe { (*(*self.nn.as_ptr()).colors.add(index)).into() })
    }
  }

  /// Creates a new [Vec] with the same colors as this `Palette`.
  pub fn to_vec(&self) -> Vec<Color> {
    // Note(Lokathor): This is safe only as long as this slice never leaves
    // this function call.
    let self_slice = unsafe {
      core::slice::from_raw_parts(
        (*self.nn.as_ptr()).colors as *mut Color,
        (*self.nn.as_ptr()).ncolors as usize,
      )
    };
    self_slice.to_vec()
  }
}

impl Clone for Palette<'_> {
  /// Clones the colors into an entirely distinct `Palette` of the same length.
  ///
  /// First a new palette of the same length is allocated, then all colors are
  /// copied over.
  ///
  /// ## Panics
  ///
  /// * If the `SDL_Palette` cannot be allocated this will panic. That
  ///   essentially only happens if you're out of memory.
  /// * If the colors cannot be copied over this will panic. It should be
  ///   impossible for that to fail, but hey.
  fn clone(&self) -> Self {
    match NonNull::new(unsafe { SDL_AllocPalette(self.len() as i32) }) {
      Some(nn) => {
        let out = Self {
          nn,
          _marker: PhantomData,
        };
        // Note(Lokathor): This is safe only as long as this slice never leaves
        // this function call.
        let self_slice = unsafe {
          core::slice::from_raw_parts(
            (*self.nn.as_ptr()).colors as *mut Color,
            (*self.nn.as_ptr()).ncolors as usize,
          )
        };
        out
          .set_colors(0, self_slice)
          .expect("Failed to copy the color data!");
        out
      }
      None => panic!("OOM: couldn't allocate an SDL_Palette!"),
    }
  }
}
