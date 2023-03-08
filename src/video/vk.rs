use core::ptr::null_mut;
use raw_vulkan_handle::*;

use super::*;

/// A window powered by Vulkan.
///
/// This window doesn't hold a vulkan instance internally. You have to make your
/// own instance after creating the window.
#[repr(C)]
pub struct VkWindow {
  win: NonNull<SDL_Window>,
  /// Note(Lokathor): The init is always the LAST field!
  init: Arc<SdlInit>,
}
impl Sdl {
  #[inline]
  pub fn create_vk_window(&self, args: CreateWinArgs<'_>) -> Result<VkWindow, SdlError> {
    let title_null: String = alloc::format!("{}\0", args.title);
    let win_p: *mut SDL_Window = unsafe {
      SDL_CreateWindow(
        title_null.as_ptr().cast(),
        SDL_WINDOWPOS_CENTERED,
        SDL_WINDOWPOS_CENTERED,
        args.width,
        args.height,
        SDL_WINDOW_VULKAN.0 | args.window_flags().0,
      )
    };
    match NonNull::new(win_p) {
      Some(win) => Ok(VkWindow { win, init: self.init.clone() }),
      None => Err(get_error()),
    }
  }
}
impl Drop for VkWindow {
  #[inline]
  fn drop(&mut self) {
    unsafe { SDL_DestroyWindow(self.win.as_ptr()) }
  }
}
impl Deref for VkWindow {
  type Target = CommonWindow;
  #[inline]
  fn deref(&self) -> &Self::Target {
    unsafe { &*(self as *const Self).cast::<CommonWindow>() }
  }
}
impl VkWindow {
  #[inline]
  #[allow(non_snake_case)]
  pub fn get_vkGetInstanceProcAddr(&self) -> Option<unsafe extern "system" fn()> {
    unsafe { core::mem::transmute(SDL_Vulkan_GetVkGetInstanceProcAddr()) }
  }

  /// Gets the list of extensions required during vulkan instance creation to
  /// make it work with this window.
  #[inline]
  pub fn get_instance_extensions(&self) -> Result<Vec<*const c_char>, SdlError> {
    let mut count: c_uint = 0;
    if unsafe { SDL_Vulkan_GetInstanceExtensions(self.win.as_ptr(), &mut count, null_mut()) }.into()
    {
      return Err(get_error());
    }
    let mut buf: Vec<*const c_char> = Vec::with_capacity(count.try_into().unwrap());
    if unsafe { SDL_Vulkan_GetInstanceExtensions(self.win.as_ptr(), &mut count, buf.as_mut_ptr()) }
      .into()
    {
      return Err(get_error());
    }
    unsafe { buf.set_len(count.try_into().unwrap()) }
    Ok(buf)
  }

  /// Creates a surface for this window.
  ///
  /// ## Safety
  /// The `instance` needs to be a valid instance with the surface creation
  /// extension enabled. If you enabled all extensions listed by a call to the
  /// `get_instance_extensions` method then that will be the case.
  #[inline]
  pub unsafe fn create_surface(&self, instance: VkInstance) -> Result<VkSurfaceKHR, SdlError> {
    let mut surface = VkSurfaceKHR::default();
    if unsafe { SDL_Vulkan_CreateSurface(self.win.as_ptr(), instance, &mut surface) }.into() {
      Ok(surface)
    } else {
      Err(get_error())
    }
  }

  /// Get the size of a window's underlying drawable area in pixels (for use
  /// with setting viewport, scissor & etc).
  #[inline]
  pub fn get_drawable_size(&self) -> (i32, i32) {
    let mut w = 0;
    let mut h = 0;
    unsafe { SDL_Vulkan_GetDrawableSize(self.win.as_ptr(), &mut w, &mut h) }
    (w, h)
  }
}
