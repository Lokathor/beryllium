use super::*;

/// A palette of [Color](Color) values.
///
/// A `Palette` value is a handle to a heap allocated `SDL_Palette`. That
/// `SDL_Palette` then has a pointer to the `Color` values, length, and some
/// reference count and version info. So you can think of it as being _similar
/// to_ `*mut Rc<[Color]>` or something like that.
#[derive(Debug)]
#[repr(transparent)]
pub struct Palette<'sdl> {
  pub(crate) ptr: *mut SDL_Palette,
  pub(crate) _marker: PhantomData<&'sdl SDLToken>,
}
impl<'sdl> Clone for Palette<'sdl> {
  fn clone(&self) -> Self {
    let mut n =
      unsafe { Self::new((*self.ptr).ncolors).expect("OOM: Could not allocate a new Palette!") };
    n.set_colors(0, &self)
      .expect("Failed to copy over the color data!");
    n
  }
}
impl<'sdl> Drop for Palette<'sdl> {
  fn drop(&mut self) {
    unsafe { SDL_FreePalette(self.ptr) }
  }
}
impl<'sdl> Deref for Palette<'sdl> {
  type Target = [Color];

  /// We can _read_ the Color values normally, we just can't _write_ the
  /// `Palette` normally.
  fn deref(&self) -> &Self::Target {
    unsafe {
      core::slice::from_raw_parts(
        (*self.ptr).colors as *mut Color,
        (*self.ptr).ncolors as usize,
      )
    }
  }
}
impl<'sdl> Palette<'sdl> {
  /// Allocates a new palette with the number of color slots given.
  ///
  /// The initial value of the palette color values is 0xFF in all four channels
  /// (opaque white).
  pub fn new(color_count: i32) -> Result<Palette<'sdl>, String> {
    let ptr = unsafe { SDL_AllocPalette(color_count) };
    if ptr.is_null() {
      Err(get_error())
    } else {
      Ok(Self {
        ptr,
        _marker: PhantomData,
      })
    }
  }

  /// Gets the number of colors in the Palette
  pub fn len(&self) -> usize {
    unsafe { (*self.ptr).ncolors as usize }
  }

  /// Assigns a slice of colors into the `Palette`, starting at the position
  /// specified.
  ///
  /// This seems silly, but SDL2 has a specific routine it uses for altering
  /// palette colors and we have to go though that, which normal use of the
  /// `IndexMut` trait would not do.
  pub fn set_colors(&mut self, start: usize, new_colors: &[Color]) -> Result<(), String> {
    let max = core::i32::MAX as usize;
    if start > max {
      return Err("beryllium error: start index > i32::MAX".to_string());
    }
    let len = new_colors.len();
    if len > max {
      return Err("beryllium error: slice length > i32::MAX".to_string());
    }
    let out = unsafe {
      SDL_SetPaletteColors(
        self.ptr,
        new_colors.as_ptr() as *const SDL_Color,
        start as i32,
        new_colors.len() as i32,
      )
    };
    if out == 0 {
      Ok(())
    } else {
      Err(get_error())
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
}
