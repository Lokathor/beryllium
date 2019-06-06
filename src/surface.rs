use super::*;

/// The desired format for a surface you are creating.
///
/// * `Indexed` formats store their pixel data as indexes into the Surface's
///   palette of [Color](Color) values.
/// * `Direct` formats store their pixel data "inline", according to the masks
///   specified. You can specify a mask of 0 to get a default mask position, but
///   if you give an Alpha mask of 0 you won't have Alpha support in that
///   Surface.
#[derive(Debug, Clone, Copy)]
#[allow(missing_docs)]
pub enum SurfaceFormat {
  /// 4 bits per pixel paletted.
  Indexed4,
  /// 8 bits per pixel paletted.
  Indexed8,
  /// 16 bits per pixel direct color.
  Direct16 {
    r_mask: u32,
    g_mask: u32,
    b_mask: u32,
    a_mask: u32,
  },
  /// 24 bits per pixel direct color.
  Direct24 {
    r_mask: u32,
    g_mask: u32,
    b_mask: u32,
    a_mask: u32,
  },
  /// 32 bits per pixel direct color.
  Direct32 {
    r_mask: u32,
    g_mask: u32,
    b_mask: u32,
    a_mask: u32,
  },
}
impl SurfaceFormat {
  /// Alias for the default Direct16 surface format.
  ///
  /// Note that this format is non-Alpha
  pub const DIRECT16_DEFAULT: Self = SurfaceFormat::Direct16 {
    r_mask: 0,
    g_mask: 0,
    b_mask: 0,
    a_mask: 0,
  };
  /// Alias for the default Direct24 surface format.
  ///
  /// Note that this format is non-Alpha
  pub const DIRECT24_DEFAULT: Self = SurfaceFormat::Direct24 {
    r_mask: 0,
    g_mask: 0,
    b_mask: 0,
    a_mask: 0,
  };
  /// Alias for the default Direct32 surface format.
  ///
  /// Note that this format is non-Alpha
  pub const DIRECT32_DEFAULT: Self = SurfaceFormat::Direct32 {
    r_mask: 0,
    g_mask: 0,
    b_mask: 0,
    a_mask: 0,
  };
}

/// Handle to a "surface", a CPU-side image.
///
/// This is fairly easy to edit, but you have to upload it to the GPU before you
/// can get it on the screen.
#[derive(Debug)]
#[repr(transparent)]
pub struct Surface<'sdl> {
  pub(crate) ptr: *mut SDL_Surface,
  pub(crate) _marker: PhantomData<&'sdl SDLToken>,
}
impl<'sdl> Drop for Surface<'sdl> {
  fn drop(&mut self) {
    unsafe { SDL_FreeSurface(self.ptr) }
  }
}
impl<'sdl> Surface<'sdl> {
  /// Lock, edit, unlock, as one easy cycle.
  ///
  /// If the Surface cannot be locked you'll get an error, otherwise your
  /// closure will be called with the base pointer to the Surface's pixel data.
  ///
  /// # Safety
  ///
  /// * You can't store the pointer and use it past the closure
  /// * You have to follow standard 2D raw pixel editing rules.
  ///   * `y * pitch + x * size_of::<PixelType>()`
  ///   * Stay in bounds and all that jazz
  pub unsafe fn lock_edit<F: FnMut(*mut u8)>(&mut self, mut op: F) -> Result<(), String> {
    let lock_output = SDL_LockSurface(self.ptr);
    if lock_output == 0 {
      op((*self.ptr).pixels as *mut u8);
      SDL_UnlockSurface(self.ptr);
      Ok(())
    } else {
      Err(get_error())
    }
  }

  /// Width in pixels
  pub fn width(&self) -> i32 {
    unsafe { (*self.ptr).w }
  }

  /// Height in pixels
  pub fn height(&self) -> i32 {
    unsafe { (*self.ptr).h }
  }

  /// Pitch in **bytes**
  pub fn pitch(&self) -> i32 {
    unsafe { (*self.ptr).pitch }
  }

  /// The current clipping rectangle for blits.
  pub fn clip_rect(&self) -> Rect {
    let mut rect = SDL_Rect::default();
    unsafe { SDL_GetClipRect(self.ptr, &mut rect) };
    rect.into()
  }

  /// Assigns a new clipping rectangle.
  ///
  /// * `Some(rect)` will clip blits to be within that rect only.
  /// * `None` will disable clipping.
  ///
  /// Returns `true` if the given rectangle intersects at least part of the
  /// `Surface` (or if it was `None`). Otherwise you get `false` and all blits
  /// will be completely clipped.
  ///
  /// Either way, blits are clipped to be within bounds of the `Surface`, so you
  /// don't have to worry about that.
  pub fn set_clip_rect(&mut self, opt_rect: Option<Rect>) -> bool {
    match opt_rect {
      Some(rect) => {
        let r: SDL_Rect = rect.into();
        SDL_TRUE == unsafe { SDL_SetClipRect(self.ptr, &r) }
      }
      None => SDL_TRUE == unsafe { SDL_SetClipRect(self.ptr, null()) },
    }
  }
}
