use core::{
  ptr::NonNull,
  sync::atomic::{AtomicBool, Ordering},
};

use alloc::{boxed::Box, string::String};
use fermium::{
  c_void,
  prelude::{
    SDL_CreateWindow, SDL_DestroyWindow, SDL_GLContext, SDL_GL_CreateContext, SDL_GL_DeleteContext,
    SDL_GL_ExtensionSupported, SDL_GL_GetDrawableSize, SDL_GL_GetProcAddress, SDL_GL_SetAttribute,
    SDL_GL_SetSwapInterval, SDL_GL_SwapWindow, SDL_GLattr, SDL_GLcontextFlag, SDL_Window,
    SDL_GL_ACCELERATED_VISUAL, SDL_GL_ACCUM_ALPHA_SIZE, SDL_GL_ACCUM_BLUE_SIZE,
    SDL_GL_ACCUM_GREEN_SIZE, SDL_GL_ACCUM_RED_SIZE, SDL_GL_ALPHA_SIZE, SDL_GL_BLUE_SIZE,
    SDL_GL_BUFFER_SIZE, SDL_GL_CONTEXT_DEBUG_FLAG, SDL_GL_CONTEXT_EGL, SDL_GL_CONTEXT_FLAGS,
    SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG, SDL_GL_CONTEXT_MAJOR_VERSION,
    SDL_GL_CONTEXT_MINOR_VERSION, SDL_GL_CONTEXT_NO_ERROR, SDL_GL_CONTEXT_PROFILE_COMPATIBILITY,
    SDL_GL_CONTEXT_PROFILE_CORE, SDL_GL_CONTEXT_PROFILE_ES, SDL_GL_CONTEXT_PROFILE_MASK,
    SDL_GL_CONTEXT_RELEASE_BEHAVIOR, SDL_GL_CONTEXT_RESET_ISOLATION_FLAG,
    SDL_GL_CONTEXT_RESET_NOTIFICATION, SDL_GL_CONTEXT_ROBUST_ACCESS_FLAG, SDL_GL_DEPTH_SIZE,
    SDL_GL_DOUBLEBUFFER, SDL_GL_FRAMEBUFFER_SRGB_CAPABLE, SDL_GL_GREEN_SIZE,
    SDL_GL_MULTISAMPLEBUFFERS, SDL_GL_MULTISAMPLESAMPLES, SDL_GL_RED_SIZE, SDL_GL_RETAINED_BACKING,
    SDL_GL_SHARE_WITH_CURRENT_CONTEXT, SDL_GL_STENCIL_SIZE, SDL_GL_STEREO, SDL_TRUE,
    SDL_WINDOWPOS_CENTERED,
  },
};
use zstring::ZStr;

use crate::{
  get_error,
  init::Sdl,
  window::{Window, WindowFlags},
  SdlError, SdlResult,
};

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum GlAttr {
  RedSize = SDL_GL_RED_SIZE.0,
  GreenSize = SDL_GL_GREEN_SIZE.0,
  BlueSize = SDL_GL_BLUE_SIZE.0,
  AlphaSize = SDL_GL_ALPHA_SIZE.0,
  BufferSize = SDL_GL_BUFFER_SIZE.0,
  Doublebuffer = SDL_GL_DOUBLEBUFFER.0,
  DepthSize = SDL_GL_DEPTH_SIZE.0,
  StencilSize = SDL_GL_STENCIL_SIZE.0,
  AccumRedSize = SDL_GL_ACCUM_RED_SIZE.0,
  AccumGreenSize = SDL_GL_ACCUM_GREEN_SIZE.0,
  AccumBlueSize = SDL_GL_ACCUM_BLUE_SIZE.0,
  AccumAlphaSize = SDL_GL_ACCUM_ALPHA_SIZE.0,
  Stereo = SDL_GL_STEREO.0,
  MultisampleBuffers = SDL_GL_MULTISAMPLEBUFFERS.0,
  MultisampleSamples = SDL_GL_MULTISAMPLESAMPLES.0,
  AcceleratedVisual = SDL_GL_ACCELERATED_VISUAL.0,
  RetainedBacking = SDL_GL_RETAINED_BACKING.0,
  MajorVersion = SDL_GL_CONTEXT_MAJOR_VERSION.0,
  MinorVersion = SDL_GL_CONTEXT_MINOR_VERSION.0,
  EGL = SDL_GL_CONTEXT_EGL.0,
  Flags = SDL_GL_CONTEXT_FLAGS.0,
  Profile = SDL_GL_CONTEXT_PROFILE_MASK.0,
  ShareWithCurrentContext = SDL_GL_SHARE_WITH_CURRENT_CONTEXT.0,
  FramebufferSrgbCapable = SDL_GL_FRAMEBUFFER_SRGB_CAPABLE.0,
  ReleaseBehavior = SDL_GL_CONTEXT_RELEASE_BEHAVIOR.0,
  ResetNotification = SDL_GL_CONTEXT_RESET_NOTIFICATION.0,
  ContextNoError = SDL_GL_CONTEXT_NO_ERROR.0,
}

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum GlProfile {
  Core = SDL_GL_CONTEXT_PROFILE_CORE.0 as i32,
  Compatibility = SDL_GL_CONTEXT_PROFILE_COMPATIBILITY.0 as i32,
  ES = SDL_GL_CONTEXT_PROFILE_ES.0 as i32,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct GlContextFlags(SDL_GLcontextFlag);
impl_bit_ops_for_tuple_newtype!(GlContextFlags);
impl GlContextFlags {
  pub const DEBUG: Self = Self(SDL_GL_CONTEXT_DEBUG_FLAG);
  pub const FORWARD_COMPATIBLE: Self = Self(SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG);
  pub const ROBUST_ACCESS: Self = Self(SDL_GL_CONTEXT_ROBUST_ACCESS_FLAG);
  pub const CONTEXT_RESET_ISOLATION: Self = Self(SDL_GL_CONTEXT_RESET_ISOLATION_FLAG);

  pub const fn as_i32(self) -> i32 {
    self.0 .0 as i32
  }
}

static GL_WINDOW_EXISTS: AtomicBool = AtomicBool::new(false);

#[repr(C)]
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
    GL_WINDOW_EXISTS.store(false, Ordering::SeqCst);
  }
}
impl core::ops::Deref for GlWindow {
  type Target = Window;
  fn deref(&self) -> &Self::Target {
    unsafe { core::mem::transmute(self) }
  }
}

impl Sdl {
  pub fn gl_set_attribute(&self, attr: GlAttr, value: i32) -> SdlResult<()> {
    if unsafe { SDL_GL_SetAttribute(SDL_GLattr(attr as u32), value) } == 0 {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  #[inline]
  pub fn create_gl_window(
    &self, title: ZStr<'_>, position: Option<(i32, i32)>, (w, h): (i32, i32), flags: WindowFlags,
  ) -> SdlResult<GlWindow> {
    if (flags & (WindowFlags::VULKAN | WindowFlags::METAL)).0 .0 != 0 {
      return Err(SdlError(Box::new(String::from(
        "beryllium: You can't specify the VULKAN or METAL window flags on a GL window",
      ))));
    }
    let (x, y) = position.unwrap_or((SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED));
    match GL_WINDOW_EXISTS.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
      Ok(_) => {
        match NonNull::new(unsafe {
          SDL_CreateWindow(title.as_ptr().cast(), x, y, w, h, (WindowFlags::OPENGL | flags).0 .0)
        }) {
          Some(win) => match NonNull::new(unsafe { SDL_GL_CreateContext(win.as_ptr()).0 }) {
            Some(ctx) => Ok(GlWindow { win, ctx, sdl: self.clone() }),
            None => {
              let e = Err(get_error());
              unsafe { SDL_DestroyWindow(win.as_ptr()) };
              GL_WINDOW_EXISTS.store(false, Ordering::SeqCst);
              e
            }
          },
          None => {
            let e = Err(get_error());
            GL_WINDOW_EXISTS.store(false, Ordering::SeqCst);
            e
          }
        }
      }
      Err(_) => {
        Err(SdlError(Box::new(String::from("beryllium: You already have an open GL window"))))
      }
    }
  }
}

impl GlWindow {
  #[inline]
  pub fn swap_backbuffer(&self) {
    unsafe { SDL_GL_SwapWindow(self.win.as_ptr()) }
  }

  #[inline]
  pub fn set_swap_interval(&self, interval: i32) -> SdlResult<()> {
    if unsafe { SDL_GL_SetSwapInterval(interval) } == 0 {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  #[inline]
  pub fn get_proc_address(&self, name: ZStr<'_>) -> *mut c_void {
    unsafe { SDL_GL_GetProcAddress(name.as_ptr().cast()) }
  }

  #[inline]
  pub fn get_drawable_size(&self) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    unsafe { SDL_GL_GetDrawableSize(self.win.as_ptr(), &mut x, &mut y) };
    (x, y)
  }

  #[inline]
  pub fn is_extension_supported(&self, extension: ZStr<'_>) -> bool {
    SDL_TRUE == unsafe { SDL_GL_ExtensionSupported(extension.as_ptr().cast()) }
  }
}
