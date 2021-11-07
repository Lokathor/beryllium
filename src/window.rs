use core::{
  ptr::NonNull,
  sync::atomic::{AtomicBool, Ordering},
};

use alloc::{boxed::Box, string::String};
use fermium::{
  c_void,
  prelude::{
    SDL_CreateWindow, SDL_DestroyWindow, SDL_GLContext, SDL_GL_CreateContext, SDL_GL_DeleteContext,
    SDL_GL_ExtensionSupported, SDL_GL_GetDrawableSize, SDL_GL_GetProcAddress,
    SDL_GL_SetSwapInterval, SDL_GL_SwapWindow, SDL_Window, SDL_WindowFlags, SDL_TRUE,
    SDL_WINDOW_ALLOW_HIGHDPI, SDL_WINDOW_BORDERLESS, SDL_WINDOW_FULLSCREEN, SDL_WINDOW_HIDDEN,
    SDL_WINDOW_INPUT_GRABBED, SDL_WINDOW_MAXIMIZED, SDL_WINDOW_METAL, SDL_WINDOW_MINIMIZED,
    SDL_WINDOW_OPENGL, SDL_WINDOW_RESIZABLE, SDL_WINDOW_VULKAN,
  },
};
use zstring::ZStr;

use crate::{get_error, init::Sdl, SdlError, SdlResult};

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct WindowFlags(SDL_WindowFlags);
impl_bit_ops_for_tuple_newtype!(WindowFlags);
impl WindowFlags {
  pub const FULLSCREEN: Self = Self(SDL_WINDOW_FULLSCREEN);
  pub const OPENGL: Self = Self(SDL_WINDOW_OPENGL);
  pub const HIDDEN: Self = Self(SDL_WINDOW_HIDDEN);
  pub const BORDERLESS: Self = Self(SDL_WINDOW_BORDERLESS);
  pub const RESIZABLE: Self = Self(SDL_WINDOW_RESIZABLE);
  pub const MAXIMIZED: Self = Self(SDL_WINDOW_MAXIMIZED);
  pub const MINIMIZED: Self = Self(SDL_WINDOW_MINIMIZED);
  pub const GRABBED: Self = Self(SDL_WINDOW_INPUT_GRABBED);
  pub const ALLOW_HIGHDPI: Self = Self(SDL_WINDOW_ALLOW_HIGHDPI);
  pub const VULKAN: Self = Self(SDL_WINDOW_VULKAN);
  pub const METAL: Self = Self(SDL_WINDOW_METAL);
}

static WINDOW_EXISTS: AtomicBool = AtomicBool::new(false);

pub struct GlWindow {
  pub(crate) win: NonNull<SDL_Window>,
  pub(crate) ctx: NonNull<c_void>,
  #[allow(unused)]
  sdl: Sdl,
}
impl Drop for GlWindow {
  fn drop(&mut self) {
    unsafe {
      SDL_GL_DeleteContext(SDL_GLContext(self.ctx.as_ptr()));
      SDL_DestroyWindow(self.win.as_ptr());
    }
    WINDOW_EXISTS.store(false, Ordering::SeqCst);
  }
}

impl Sdl {
  pub fn create_gl_window(
    &self, title: ZStr<'_>, (x, y): (i32, i32), (w, h): (i32, i32), flags: WindowFlags,
  ) -> SdlResult<GlWindow> {
    match WINDOW_EXISTS.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
      Ok(_) => {
        match NonNull::new(unsafe {
          SDL_CreateWindow(title.as_ptr().cast(), x, y, w, h, (WindowFlags::OPENGL | flags).0 .0)
        }) {
          Some(win) => match NonNull::new(unsafe { SDL_GL_CreateContext(win.as_ptr()).0 }) {
            Some(ctx) => Ok(GlWindow { win, ctx, sdl: self.clone() }),
            None => {
              let e = Err(get_error());
              unsafe { SDL_DestroyWindow(win.as_ptr()) };
              WINDOW_EXISTS.store(false, Ordering::SeqCst);
              e
            }
          },
          None => {
            let e = Err(get_error());
            WINDOW_EXISTS.store(false, Ordering::SeqCst);
            e
          }
        }
      }
      Err(_) => Err(SdlError(Box::new(String::from("beryllium: You already have an open window")))),
    }
  }
}

impl GlWindow {
  pub fn swap_backbuffer(&self) {
    unsafe { SDL_GL_SwapWindow(self.win.as_ptr()) }
  }

  pub fn set_swap_interval(&self, interval: i32) -> SdlResult<()> {
    if unsafe { SDL_GL_SetSwapInterval(interval) } == 0 {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  pub fn get_proc_address(&self, name: ZStr<'_>) -> *mut c_void {
    unsafe { SDL_GL_GetProcAddress(name.as_ptr().cast()) }
  }

  pub fn get_drawable_size(&self) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    unsafe { SDL_GL_GetDrawableSize(self.win.as_ptr(), &mut x, &mut y) };
    (x, y)
  }

  pub fn is_extension_supported(&self, extension: ZStr<'_>) -> bool {
    SDL_TRUE == unsafe { SDL_GL_ExtensionSupported(extension.as_ptr().cast()) }
  }
}
