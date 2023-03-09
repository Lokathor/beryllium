use super::*;

static GL_WINDOW_ACTIVE: AtomicBool = AtomicBool::new(false);

/// A window powered by GL.
///
/// Because GL only allows one draw context per thread, and because SDL2 isn't
/// thread-safe by default, you can only make one of these.
#[repr(C)]
pub struct GlWindow {
  win: NonNull<SDL_Window>,
  ctx: SDL_GLContext,
  /// Note(Lokathor): The init is always the LAST field!
  init: Arc<SdlInit>,
}
impl Sdl {
  /// You can only have one GL window active!
  #[inline]
  pub fn create_gl_window(&self, args: CreateWinArgs<'_>) -> Result<GlWindow, SdlError> {
    match GL_WINDOW_ACTIVE.compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire) {
      Ok(_) => {
        let title_null: String = alloc::format!("{}\0", args.title);
        let win_p: *mut SDL_Window = unsafe {
          SDL_CreateWindow(
            title_null.as_ptr().cast(),
            SDL_WINDOWPOS_CENTERED,
            SDL_WINDOWPOS_CENTERED,
            args.width,
            args.height,
            SDL_WINDOW_OPENGL.0 | args.window_flags().0,
          )
        };
        match NonNull::new(win_p) {
          Some(win) => {
            let ctx: SDL_GLContext = unsafe { SDL_GL_CreateContext(win_p) };
            if ctx.0.is_null() {
              unsafe { SDL_DestroyWindow(win_p) }
              Err(get_error())
            } else {
              Ok(GlWindow { win, ctx, init: self.init.clone() })
            }
          }
          None => Err(get_error()),
        }
      }
      Err(_) => Err(SdlError::new("beryllium: GL window already active.")),
    }
  }
}
impl Drop for GlWindow {
  #[inline]
  fn drop(&mut self) {
    unsafe { SDL_GL_DeleteContext(self.ctx) }
    unsafe { SDL_DestroyWindow(self.win.as_ptr()) }
    GL_WINDOW_ACTIVE.store(false, Ordering::Release);
  }
}
impl Deref for GlWindow {
  type Target = CommonWindow;
  #[inline]
  fn deref(&self) -> &Self::Target {
    unsafe { &*(self as *const Self).cast::<CommonWindow>() }
  }
}
impl GlWindow {
  #[inline]
  pub fn get_drawable_size(&self) -> (i32, i32) {
    let mut width = 0_i32;
    let mut height = 0_i32;
    unsafe { SDL_GL_GetDrawableSize(self.win.as_ptr(), &mut width, &mut height) }
    (width, height)
  }

  #[inline]
  pub fn swap_window(&self) {
    unsafe { SDL_GL_SwapWindow(self.win.as_ptr()) }
  }

  #[inline]
  pub fn supports_extension(&self, ext: &str) -> bool {
    let ext_null = alloc::format!("{ext}\0");
    unsafe { SDL_GL_ExtensionSupported(ext_null.as_ptr().cast()) }.into()
  }

  /// ## Safety
  /// * The pointer must point to a zero-terminated string that names a GL
  ///   command that's supported by the current GL context's version and
  ///   supported extensions.
  #[inline]
  pub unsafe fn get_proc_address(&self, name: *const u8) -> *mut c_void {
    SDL_GL_GetProcAddress(name.cast())
  }

  #[inline]
  pub fn set_swap_interval(&self, interval: GlSwapInterval) -> Result<(), SdlError> {
    if 0 == unsafe { SDL_GL_SetSwapInterval(interval as i32) } {
      Ok(())
    } else {
      Err(get_error())
    }
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct GlContextFlags(SDL_GLcontextFlag);
impl GlContextFlags {
  pub const DEBUG: Self = Self(SDL_GL_CONTEXT_DEBUG_FLAG);
  pub const FORWARD_COMPATIBLE: Self = Self(SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG);
  pub const ROBUST_ACCESS: Self = Self(SDL_GL_CONTEXT_ROBUST_ACCESS_FLAG);
  pub const RESET_ISOLATION: Self = Self(SDL_GL_CONTEXT_RESET_ISOLATION_FLAG);
}
impl core::ops::BitOr for GlContextFlags {
  type Output = Self;
  #[inline]
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
impl core::ops::BitOrAssign for GlContextFlags {
  #[inline]
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum GlProfile {
  Core = SDL_GL_CONTEXT_PROFILE_CORE.0 as i32,
  Compatibility = SDL_GL_CONTEXT_PROFILE_COMPATIBILITY.0 as i32,
  ES = SDL_GL_CONTEXT_PROFILE_ES.0 as i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum GlSwapInterval {
  Immediate = 0,
  Vsync = 1,
  AdaptiveVsync = -1,
}

impl Sdl {
  /// Sets the *minimum* number of depth buffer bits (default=16).
  #[inline]
  pub fn set_gl_depth_bits(&self, count: u8) -> Result<(), SdlError> {
    if 0 == unsafe { SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, i32::from(count)) } {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  /// Sets the *minimum* number of stencil buffer bits (default=0).
  #[inline]
  pub fn set_gl_stencil_bits(&self, count: u8) -> Result<(), SdlError> {
    if 0 == unsafe { SDL_GL_SetAttribute(SDL_GL_STENCIL_SIZE, i32::from(count)) } {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  /// Number of multisample buffers (default=0).
  ///
  /// * Set to 1 to allow multisampling.
  /// * *More* than one multisample buffer is extremely unlikely to be available
  /// * Also be sure to call [`set_gl_multisample_count`].
  #[inline]
  pub fn set_gl_multisample_buffers(&self, count: u8) -> Result<(), SdlError> {
    if 0 == unsafe { SDL_GL_SetAttribute(SDL_GL_MULTISAMPLEBUFFERS, i32::from(count)) } {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  /// Number of multisamples per normal sample (default=0).
  ///
  /// * Usually you should set this to a power of 2. Picking 4 is a "safe
  ///   default" that will usually be available, but some platforms allow 16 or
  ///   more. This has diminishing returns as far as what the user can actually
  ///   perceive, so more than 16 is likely a waste of GPU time.
  /// * Also be sure to call [`set_gl_multisample_buffers`].
  #[inline]
  pub fn set_gl_multisample_count(&self, count: u8) -> Result<(), SdlError> {
    if 0 == unsafe { SDL_GL_SetAttribute(SDL_GL_MULTISAMPLESAMPLES, i32::from(count)) } {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  #[inline]
  pub fn set_gl_context_major_version(&self, major: u8) -> Result<(), SdlError> {
    if 0 == unsafe { SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, i32::from(major)) } {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  #[inline]
  pub fn set_gl_context_minor_version(&self, minor: u8) -> Result<(), SdlError> {
    if 0 == unsafe { SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, i32::from(minor)) } {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  #[inline]
  pub fn set_gl_context_flags(&self, flags: GlContextFlags) -> Result<(), SdlError> {
    if 0 == unsafe { SDL_GL_SetAttribute(SDL_GL_CONTEXT_FLAGS, flags.0 .0 as i32) } {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  #[inline]
  pub fn set_gl_profile(&self, profile: GlProfile) -> Result<(), SdlError> {
    if 0 == unsafe { SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, profile as i32) } {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  #[inline]
  pub fn set_gl_framebuffer_srgb_capable(&self, capable: bool) -> Result<(), SdlError> {
    if 0 == unsafe { SDL_GL_SetAttribute(SDL_GL_FRAMEBUFFER_SRGB_CAPABLE, i32::from(capable)) } {
      Ok(())
    } else {
      Err(get_error())
    }
  }
}
