use core::ptr::NonNull;

use fermium::{
  c_void,
  prelude::{
    SDL_GLContext, SDL_GL_CreateContext, SDL_GL_DeleteContext,
    SDL_GL_ExtensionSupported, SDL_GL_GetDrawableSize, SDL_GL_GetProcAddress,
    SDL_GL_SetSwapInterval, SDL_GL_SwapWindow, SDL_TRUE,
  },
};
use zstring::ZStr;

use crate::{get_error, init::Sdl, window::Window, SdlResult};

pub struct GlContext {
  nn: NonNull<c_void>,
  #[allow(unused)]
  sdl: Sdl,
}

impl Sdl {
  pub unsafe fn gl_create_context(&self, win: &Window) -> SdlResult<GlContext> {
    match NonNull::new(SDL_GL_CreateContext(win.nn.as_ptr()).0) {
      Some(nn) => Ok(GlContext { nn, sdl: self.clone() }),
      None => Err(get_error()),
    }
  }
}

impl GlContext {
  pub unsafe fn delete_context(self) {
    SDL_GL_DeleteContext(SDL_GLContext(self.nn.as_ptr()))
  }

  pub fn swap_window(&self, win: &Window) {
    unsafe { SDL_GL_SwapWindow(win.nn.as_ptr()) }
  }

  pub fn set_swap_interval(interval: i32) -> SdlResult<()> {
    if unsafe { SDL_GL_SetSwapInterval(interval) } == 0 {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  pub fn get_proc_address(&self, name: ZStr<'_>) -> *mut c_void {
    unsafe { SDL_GL_GetProcAddress(name.as_ptr().cast()) }
  }

  pub fn get_drawable_size(&self, win: &Window) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    unsafe { SDL_GL_GetDrawableSize(win.nn.as_ptr(), &mut x, &mut y) };
    (x, y)
  }

  pub fn is_extension_supported(&self, extension: ZStr<'_>) -> bool {
    SDL_TRUE == unsafe { SDL_GL_ExtensionSupported(extension.as_ptr().cast()) }
  }
}
