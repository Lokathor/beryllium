use super::*;

/// A palette of [Color](Color) values.
///
/// The way that the `Palette` type works is very different from Rust's normal
/// ownership model, so please pay attention as I explain.
///
/// A `Palette` value holds a pointer to a heap allocated
/// [SDL_Palette](SDL_Palette). That `SDL_Palette` has a pointer to the heap
/// allocated `Color` values, along with a length, reference count, and version
/// number.
///
/// You allocate a `Palette` by calling
/// [SDLToken::new_palette](SDLToken::new_palette) and specifying how many
/// `Color` values the `Palette` should hold. All slots in a new `Palette` are
/// initialized to opaque white (`0xFF` in all four color channel).
///
/// When you set a Palette on a [Surface](Surface) or [PixelFormat](PixelFormat)
/// it moves some pointers and adjusts the reference count of the `Palette`. Now
/// you have the `Palette`, and _also_ that thing has the same `Palette`. An
/// edit to the `Palette` data in either location will affect everyone's data.
///
/// As a result, I cannot allow you to _ever_ construct a shared reference or
/// unique reference to the `Color` data held inside the `Palette`. This means
/// no [Deref](Deref), [Index](Index), or [IndexMut](IndexMut), no Iterators of
/// any kind, none of that. This definitely makes the API of the `Palette` type
/// not quite as fun as you might like.
#[derive(Debug)] // TODO: We probably want a custom Debug impl
#[repr(transparent)]
pub struct Palette<'sdl> {
  pub(crate) ptr: *mut SDL_Palette, // TODO: NonNull<SDL_Palette> ?
  pub(crate) _marker: PhantomData<&'sdl SDLToken>,
}

// TODO: PAST HERE NEEDS A CAREFUL SECOND PASS VERIFICATION

impl<'sdl> Clone for Palette<'sdl> {
  fn clone(&self) -> Self {
    let ptr = unsafe { SDL_AllocPalette(self.len() as i32) };
    if ptr.is_null() {
      panic!("OOM: Could not allocate a new Palette!");
    } else {
      let mut n = Palette {
        ptr,
        _marker: PhantomData,
      };
      let self_slice = unsafe {
        core::slice::from_raw_parts(
          (*self.ptr).colors as *mut Color,
          (*self.ptr).ncolors as usize,
        )
      };
      n.set_colors(0, self_slice)
        .expect("Failed to copy over the color data!");
      n
    }
  }
}
impl<'sdl> Drop for Palette<'sdl> {
  fn drop(&mut self) {
    unsafe { SDL_FreePalette(self.ptr) }
  }
}
impl SDLToken {
  /// Allocates a new [Palette](Palette) with the number of color slots given.
  ///
  /// The initial value of the palette color values is 0xFF in all four channels
  /// (opaque white).
  pub fn new_palette<'sdl>(&'sdl self, color_count: usize) -> Result<Palette<'sdl>, String> {
    let max = core::i32::MAX as usize;
    if color_count > max {
      return Err("beryllium error: color_count > i32::MAX".to_string());
    }
    if color_count < 2 {
      return Err("beryllium error: color_count of a palette must be at least 2".to_string());
    }
    let ptr = unsafe { SDL_AllocPalette(color_count as i32) };
    if ptr.is_null() {
      Err(get_error())
    } else {
      Ok(Palette {
        ptr,
        _marker: PhantomData,
      })
    }
  }
}

#[allow(clippy::len_without_is_empty)]
impl<'sdl> Palette<'sdl> {
  /// Gets the number of colors in the Palette
  pub fn len(&self) -> usize {
    unsafe { (*self.ptr).ncolors as usize }
  }

  /// Gets the [Color](Color) at the index specified.
  pub fn get_color(&self, index: usize) -> Option<Color> {
    if index < self.len() {
      Some(unsafe { (*(*self.ptr).colors.add(index)).into() })
    } else {
      None
    }
  }

  /// Assigns the given color to the index specified.
  ///
  /// This is shorthand for [set_colors](Palette::set_colors) with a single
  /// element slice. If you have many colors in a row to set you should use that
  /// instead.
  pub fn set_color(&mut self, index: usize, color: Color) -> Result<(), String> {
    self.set_colors(index, core::slice::from_ref(&color))
  }

  /// Assigns a slice of colors into the `Palette`, starting at the position
  /// specified.
  ///
  /// This seems silly, but SDL2 has a specific routine it uses for altering
  /// palette colors and we have to go though that, which normal use of the
  /// `IndexMut` trait would not do.
  ///
  /// `start` values >= the length will give an error. The input slice is
  /// automatically clipped as necessary, so as to not go out of bounds.
  pub fn set_colors(&mut self, start: usize, new_colors: &[Color]) -> Result<(), String> {
    if start >= self.len() {
      return Err("beryllium error: start index out of bounds".to_string());
    }
    // We'll manually clip the input slice instead of relying on SDL2's dubious
    // clipping process.
    let clipped_length = (self.len() - start).min(new_colors.len());
    debug_assert!(start + clipped_length <= self.len());
    let out = unsafe {
      SDL_SetPaletteColors(
        self.ptr,
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
}
