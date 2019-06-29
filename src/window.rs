use super::*;

/// Flags that a window might have.
///
/// This is for use with [create_window](SDLToken::create_window) as well as
/// other methods that examine the state of a window.
#[derive(Debug, Default, Clone, Copy)]
#[repr(transparent)]
pub struct WindowFlags(pub(crate) SDL_WindowFlags::Type);
#[allow(bad_style)]
type SDL_WindowFlags_Type = SDL_WindowFlags::Type;
#[allow(missing_docs)]
impl WindowFlags {
  phantom_fields! {
    self.0: SDL_WindowFlags_Type,
    fullscreen: SDL_WINDOW_FULLSCREEN,
    /// Make a window with OpenGL Support
    opengl: SDL_WINDOW_OPENGL,
    /// The window is _currently_ being shown.
    ///
    /// Newly created windows are always shown, you don't need to pass this flag
    /// to the window creation (though it doesn't hurt I guess).
    shown: SDL_WINDOW_SHOWN,
    hidden: SDL_WINDOW_HIDDEN,
    borderless: SDL_WINDOW_BORDERLESS,
    resizable: SDL_WINDOW_RESIZABLE,
    minimized: SDL_WINDOW_MINIMIZED,
    maximized: SDL_WINDOW_MAXIMIZED,
    input_grabbed: SDL_WINDOW_INPUT_GRABBED,
    input_focus: SDL_WINDOW_INPUT_FOCUS,
    mouse_focus: SDL_WINDOW_MOUSE_FOCUS,
    fullscreen_desktop: SDL_WINDOW_FULLSCREEN_DESKTOP,
    foreign: SDL_WINDOW_FOREIGN,
    /// Window should be created in high-DPI mode if supported.
    ///
    /// On macOS `NSHighResolutionCapable` must be set true in the application's
    /// `Info.plist` for this to have any effect.
    allow_high_dpi: SDL_WINDOW_ALLOW_HIGHDPI,
    /// Distinct from "input grabbed".
    mouse_capture: SDL_WINDOW_MOUSE_CAPTURE,
    always_on_top: SDL_WINDOW_ALWAYS_ON_TOP,
    /// Window should not be added to the taskbar list (eg: a dialog box).
    skip_taskbar: SDL_WINDOW_SKIP_TASKBAR,
    utility: SDL_WINDOW_UTILITY,
    tooltip: SDL_WINDOW_TOOLTIP,
    popup_menu: SDL_WINDOW_POPUP_MENU,
    vulkan: SDL_WINDOW_VULKAN,
  }
}

/// Centers the window along the axis used.
///
/// See [create_window](SDLToken::create_window).
pub const WINDOW_POSITION_CENTERED: i32 = SDL_WINDOWPOS_CENTERED_MASK as i32;

/// Gives the window an undefined position on this axis.
///
/// See [create_window](SDLToken::create_window).
pub const WINDOW_POSITION_UNDEFINED: i32 = SDL_WINDOWPOS_UNDEFINED_MASK as i32;

/// Handle to a window on the screen.
#[derive(Debug)]
#[repr(transparent)]
pub struct Window<'sdl> {
  pub(crate) ptr: *mut SDL_Window,
  pub(crate) _marker: PhantomData<&'sdl SDLToken>,
}
impl<'sdl> Drop for Window<'sdl> {
  fn drop(&mut self) {
    unsafe { SDL_DestroyWindow(self.ptr) }
  }
}
impl<'sdl> Window<'sdl> {
  /// Like the [lone_message_box](lone_message_box) function, but
  /// modal to this `Window`.
  ///
  /// Because you need a valid `Window` to call this method, we don't need to
  /// mark it as `unsafe`.
  pub fn modal_message_box(
    &self, box_type: MessageBox, title: &str, message: &str,
  ) -> Result<(), String> {
    let title_null: Vec<u8> = title.bytes().chain(Some(0)).collect();
    let message_null: Vec<u8> = message.bytes().chain(Some(0)).collect();
    let output = unsafe {
      SDL_ShowSimpleMessageBox(
        box_type as u32,
        title_null.as_ptr() as *const c_char,
        message_null.as_ptr() as *const c_char,
        self.ptr,
      )
    };
    if output == 0 {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  /// Makes a renderer for the window.
  ///
  /// # Safety
  ///
  /// * Each renderer must only be used with its own window
  /// * Each renderer must only be used with textures that it created
  ///
  /// If you only have a single renderer then this is trivially proved, if you
  /// make more than one renderer it's up to you to verify this.
  pub unsafe fn create_renderer<'win>(
    &'win self, driver_index: Option<usize>, flags: RendererFlags,
  ) -> Result<Renderer<'sdl, 'win>, String> {
    let index = driver_index.map(|u| u as i32).unwrap_or(-1);
    let ptr = SDL_CreateRenderer(self.ptr, index, flags.0 as u32);
    if ptr.is_null() {
      Err(get_error())
    } else {
      Ok(Renderer {
        ptr,
        _marker: PhantomData,
      })
    }
  }

  /// Sets the title of the window.
  pub fn set_title(&self, title: &str) {
    let title_null: Vec<u8> = title.bytes().chain(Some(0)).collect();
    unsafe { SDL_SetWindowTitle(self.ptr, title_null.as_ptr() as *const c_char) }
  }

  /// Returns the title of the window in UTF-8 format or "" if there is no title.
  pub fn get_title(&self) -> String {
    unsafe { gather_string(SDL_GetWindowTitle(self.ptr)) }
  }

  /// Gets the logical size of the window (in screen coordinates).
  ///
  /// Use the GL Drawable Size or Renderer Output Size checks to get the
  /// physical pixel count, if you need that.
  pub fn size(&self) -> (i32, i32) {
    let mut w = 0;
    let mut h = 0;
    unsafe { SDL_GetWindowSize(self.ptr, &mut w, &mut h) };
    (w, h)
  }

  /// Sets the logical size of the window.
  ///
  /// Note that fullscreen windows automatically match the size of the display
  /// mode, so use [set_display_mode](Window::set_display_mode) instead.
  pub fn set_size(&self, width: i32, height: i32) {
    unsafe { SDL_SetWindowSize(self.ptr, width, height) }
  }

  /// Obtains info about the fullscreen settings of the window.
  pub fn display_mode(&self) -> Result<DisplayMode, String> {
    let mut mode = SDL_DisplayMode::default();
    let out = unsafe { SDL_GetWindowDisplayMode(self.ptr, &mut mode) };
    if out == 0 {
      Ok(DisplayMode::from(mode))
    } else {
      Err(get_error())
    }
  }

  /// Assigns the fullscreen display mode for the window.
  ///
  /// * If `Some(mode)`, attempts to set the mode given.
  /// * If `None`, it will use the window's dimensions, and the desktop's
  ///   current format and refresh rate.
  pub fn set_display_mode(&self, opt_mode: Option<DisplayMode>) -> Result<(), String> {
    let out = match opt_mode {
      Some(mode) => {
        let sdl_mode: SDL_DisplayMode = mode.into();
        unsafe { SDL_SetWindowDisplayMode(self.ptr, &sdl_mode) }
      }
      None => unsafe { SDL_SetWindowDisplayMode(self.ptr, null_mut()) },
    };
    if out == 0 {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  /// Sets the window's fullscreen style.
  ///
  /// * Fullscreen: Performs an actual video mode change.
  /// * Fullscreen Desktop: "fake" fullscreen with full resolution but no video
  ///   mode change.
  /// * Windowed: Don't use fullscreen.
  pub fn set_fullscreen_style(&self, style: FullscreenStyle) -> Result<(), String> {
    let out = unsafe { SDL_SetWindowFullscreen(self.ptr, style as u32) };
    if out == 0 {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  /// Creates a context for this window and makes it current.
  pub unsafe fn gl_create_context<'win>(&'win self) -> Result<GLContext<'sdl, 'win>, String> {
    let ctx = SDL_GL_CreateContext(self.ptr);
    if ctx.is_null() {
      Err(get_error())
    } else {
      Ok(GLContext {
        ctx,
        _marker: PhantomData,
      })
    }
  }

  /// Obtains the size of the drawable space in the window.
  ///
  /// This gives you a number of "physical pixels", so it might be different
  /// from the "logical pixels" value you get when you call
  /// [size](Window::size). This is primarily for use with
  /// [glViewport](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewport.xhtml)
  pub fn gl_get_drawable_size(&self) -> (i32, i32) {
    let mut w = 0;
    let mut h = 0;
    unsafe { SDL_GL_GetDrawableSize(self.ptr, &mut w, &mut h) };
    (w, h)
  }

  /// Swaps the window's OpenGL buffers.
  ///
  /// If double buffering isn't enabled this just does nothing.
  pub unsafe fn gl_swap_window(&self) {
    SDL_GL_SwapWindow(self.ptr)
  }

  /// Makes the given context the current context in this window.
  pub unsafe fn gl_make_current(&self, ctx: &GLContext) -> Result<(), String> {
    let out = SDL_GL_MakeCurrent(self.ptr, ctx.ctx);
    if out == 0 {
      Ok(())
    } else {
      Err(get_error())
    }
  }
}

/// The window's fullscreen style.
///
/// * Windowed size is controlled with [set_size](Window::set_size)
/// * Fullscreen and FullscreenDesktop size is controlled with [set_display_mode](Window::set_display_mode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
pub enum FullscreenStyle {
  /// Performs an actual video mode change.
  Fullscreen = SDL_WINDOW_FULLSCREEN,
  /// "fakes" a fullscreen window without a video mode change.
  FullscreenDesktop = SDL_WINDOW_FULLSCREEN_DESKTOP,
  /// Makes the window actually work like a window.
  Windowed = 0,
}

/// A description of a fullscreen display mode.
#[derive(Debug, Clone, Copy)]
pub struct DisplayMode {
  /// The screen's format
  pub format: PixelFormatEnum,
  /// Width, in logical units
  pub width: i32,
  /// Height, in logical units
  pub height: i32,
  /// Refresh rate in Hz, or 0 if unspecified.
  pub refresh_rate: i32,
  driver_data: *mut c_void,
}
impl From<SDL_DisplayMode> for DisplayMode {
  fn from(sdl_mode: SDL_DisplayMode) -> Self {
    Self {
      format: PixelFormatEnum::from(sdl_mode.format as fermium::_bindgen_ty_6::Type),
      width: sdl_mode.w,
      height: sdl_mode.h,
      refresh_rate: sdl_mode.refresh_rate,
      driver_data: sdl_mode.driverdata,
    }
  }
}
impl From<DisplayMode> for SDL_DisplayMode {
  fn from(mode: DisplayMode) -> Self {
    Self {
      format: mode.format as u32,
      w: mode.width,
      h: mode.height,
      refresh_rate: mode.refresh_rate,
      driverdata: mode.driver_data,
    }
  }
}
impl DisplayMode {
  /// Constructs a new display mode as specified.
  ///
  /// This is necessary because the display mode has a hidden driver data
  /// pointer which must be initialized to null and not altered by outside users.
  pub const fn new(format: PixelFormatEnum, width: i32, height: i32, refresh_rate: i32) -> Self {
    Self {
      format,
      width,
      height,
      refresh_rate,
      driver_data: null_mut(),
    }
  }
}
