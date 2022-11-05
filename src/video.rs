use crate::{
  error::{get_error, SdlError},
  init::SdlInit,
  Sdl,
};
use alloc::{string::String, sync::Arc};
use core::{
  ops::Deref,
  ptr::NonNull,
  sync::atomic::{AtomicBool, Ordering},
};
use fermium::prelude::*;

pub struct CreateWinArgs<'s> {
  pub title: &'s str,
  pub width: i32,
  pub height: i32,
  pub allow_high_dpi: bool,
  pub borderless: bool,
  pub resizable: bool,
}
impl Default for CreateWinArgs<'_> {
  #[inline]
  fn default() -> Self {
    Self {
      title: "DefaultName",
      width: 800,
      height: 600,
      allow_high_dpi: true,
      borderless: false,
      resizable: false,
    }
  }
}

/// Provides the methods common to all types of window.
#[repr(C)]
pub struct CommonWindow {
  win: NonNull<SDL_Window>,
}
impl CommonWindow {
  #[inline]
  pub fn get_window_size(&self) -> (i32, i32) {
    let mut width = 0_i32;
    let mut height = 0_i32;
    unsafe { SDL_GetWindowSize(self.win.as_ptr(), &mut width, &mut height) }
    (width, height)
  }
}

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
            SDL_WINDOW_OPENGL.0 | SDL_WINDOW_ALLOW_HIGHDPI.0,
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
}
