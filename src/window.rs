use super::*;

/// Flags that a window might have.
///
/// This is for use with [create_window](SDLToken::create_window) as well as
/// other methods that examine the state of a window.
#[derive(Debug, Default, Clone, Copy)]
#[repr(transparent)]
pub struct WindowFlags(pub(crate) fermium::SDL_WindowFlags);
#[allow(bad_style)]
type SDL_WindowFlags_Type = fermium::SDL_WindowFlags;
use fermium::{
  SDL_WINDOW_ALLOW_HIGHDPI, SDL_WINDOW_ALWAYS_ON_TOP, SDL_WINDOW_BORDERLESS, SDL_WINDOW_FOREIGN,
  SDL_WINDOW_FULLSCREEN, SDL_WINDOW_FULLSCREEN_DESKTOP, SDL_WINDOW_HIDDEN, SDL_WINDOW_INPUT_FOCUS,
  SDL_WINDOW_INPUT_GRABBED, SDL_WINDOW_MAXIMIZED, SDL_WINDOW_MINIMIZED, SDL_WINDOW_MOUSE_CAPTURE,
  SDL_WINDOW_MOUSE_FOCUS, SDL_WINDOW_OPENGL, SDL_WINDOW_POPUP_MENU, SDL_WINDOW_RESIZABLE,
  SDL_WINDOW_SHOWN, SDL_WINDOW_SKIP_TASKBAR, SDL_WINDOW_TOOLTIP, SDL_WINDOW_UTILITY,
  SDL_WINDOW_VULKAN,
};
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
pub const WINDOW_POSITION_CENTERED: i32 = fermium::SDL_WINDOWPOS_CENTERED_MASK as i32;

/// Gives the window an undefined position on this axis.
///
/// See [create_window](SDLToken::create_window).
pub const WINDOW_POSITION_UNDEFINED: i32 = fermium::SDL_WINDOWPOS_UNDEFINED_MASK as i32;

/// Handle to a window on the screen.
#[derive(Debug)]
#[repr(transparent)]
pub struct Window<'sdl> {
  pub(crate) ptr: *mut fermium::SDL_Window,
  pub(crate) _marker: PhantomData<&'sdl SDLToken>,
}
impl<'sdl> Drop for Window<'sdl> {
  fn drop(&mut self) {
    unsafe { fermium::SDL_DestroyWindow(self.ptr) }
  }
}

unsafe impl<'sdl> raw_window_handle::HasRawWindowHandle for Window<'sdl> {
  fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
    #[cfg(any(
      target_os = "linux",
      target_os = "dragonfly",
      target_os = "freebsd",
      target_os = "netbsd",
      target_os = "openbsd"
    ))]
    use raw_window_handle::unix::{WaylandHandle, XcbHandle, XlibHandle};
    #[cfg(windows)]
    use raw_window_handle::windows::WindowsHandle;
    #[cfg(target_os = "macos")]
    use raw_window_handle::macos::MacOSHandle;
    use raw_window_handle::RawWindowHandle;
    let mut wm_info = fermium::SDL_SysWMinfo::default();
    let b = unsafe { fermium::SDL_GetWindowWMInfo(self.ptr, &mut wm_info) };
    if b == fermium::SDL_TRUE {
      match wm_info.subsystem {
        #[cfg(windows)]
        fermium::SDL_SYSWM_WINDOWS => {
          RawWindowHandle::Windows(WindowsHandle {
            hwnd: unsafe { wm_info.info.win.window as *mut core::ffi::c_void },
            ..WindowsHandle::empty()
          })
        }
        #[cfg(any(
          target_os = "linux",
          target_os = "dragonfly",
          target_os = "freebsd",
          target_os = "netbsd",
          target_os = "openbsd"
        ))]
        fermium::SDL_SYSWM_WAYLAND => {
          RawWindowHandle::Wayland(WaylandHandle {
            surface: unsafe { wm_info.info.wl.surface as *mut core::ffi::c_void },
            display: unsafe { wm_info.info.wl.display as *mut core::ffi::c_void },
            ..WaylandHandle::empty()
          })
        }
        #[cfg(any(
          target_os = "linux",
          target_os = "dragonfly",
          target_os = "freebsd",
          target_os = "netbsd",
          target_os = "openbsd"
        ))]
        fermium::SDL_SYSWM_X11 => {
          RawWindowHandle::Xlib(XlibHandle {
            window: unsafe { wm_info.info.x11.window },
            display: unsafe { wm_info.info.x11.display as *mut core::ffi::c_void },
            ..WaylandHandle::empty()
          })
        }
        #[cfg(target_os = "macos")]
        fermium::SDL_SYSWM_COCOA => {
          RawWindowHandle::MacOS(MacOSHandle {
            ns_window: unsafe { wm_info.info.cocoa.window as *mut core::ffi::c_void },
            ns_view: unsafe { core::ptr::null_mut() },
            ..MacOSHandle::empty()
          })
        }
        _ => panic!("SDL2 is using a window subsystem that is not supported by the raw-window-handle API and Osspial wrote the trait to be infallible despite that clearly not always being the case. https://github.com/rust-windowing/raw-window-handle/issues/new"),
      }
    } else {
      panic!("Could not retrieve window info and Osspial wrote the trait to be infallible despite that clearly not always being the case. https://github.com/rust-windowing/raw-window-handle/issues/new");
    }
  }
}

impl<'sdl> Window<'sdl> {
  /// Like the [lone_message_box](lone_message_box) function, but
  /// modal to this `Window`.
  ///
  /// Because you need a valid `Window` to call this method, we don't need to
  /// mark it as `unsafe`.
  pub fn modal_message_box(
    &self,
    box_type: MessageBox,
    title: &str,
    message: &str,
  ) -> Result<(), String> {
    let title_null: Vec<u8> = title.bytes().chain(Some(0)).collect();
    let message_null: Vec<u8> = message.bytes().chain(Some(0)).collect();
    let output = unsafe {
      fermium::SDL_ShowSimpleMessageBox(
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

  /// Use this function to get the window flags.
  pub fn flags(&self) -> WindowFlags {
    let mut flags = WindowFlags::default();
    flags.0 = unsafe { fermium::SDL_GetWindowFlags(self.ptr) } as fermium::SDL_WindowFlags;
    flags
  }

  /// Returns the user-resizable state of a window.
  pub fn resizable(&self) -> bool {
    self.flags().resizable()
  }

  /// Use this function to set the user-resizable state of a window.
  pub fn set_resizable(&self, resizable: bool) {
    unsafe { fermium::SDL_SetWindowResizable(self.ptr, into_sdl_bool(resizable)) }
  }

  /// Returns the title of the window in UTF-8 format or "" if there is no title.
  pub fn title(&self) -> String {
    unsafe { gather_string(fermium::SDL_GetWindowTitle(self.ptr)) }
  }

  /// Sets the title of the window.
  pub fn set_title(&self, title: &str) {
    let title_null: Vec<u8> = title.bytes().chain(Some(0)).collect();
    unsafe { fermium::SDL_SetWindowTitle(self.ptr, title_null.as_ptr() as *const c_char) }
  }

  /// Use this function to show a window.
  pub fn show(&self) {
    unsafe { fermium::SDL_ShowWindow(self.ptr) }
  }

  /// Use this function to hide a window.
  pub fn hide(&self) {
    unsafe { fermium::SDL_HideWindow(self.ptr) }
  }

  /// Gets the logical size of the window (in screen coordinates).
  ///
  /// For physical pixel counts use the method appropriate to your backend:
  /// [GLWindow::drawable_size] or [RendererWindow::output_size].
  pub fn size(&self) -> (i32, i32) {
    let mut w = 0;
    let mut h = 0;
    unsafe { fermium::SDL_GetWindowSize(self.ptr, &mut w, &mut h) };
    (w, h)
  }

  /// Sets the logical size of the window.
  ///
  /// Note that fullscreen windows automatically match the size of the display
  /// mode, so use [set_display_mode](Window::set_display_mode) instead.
  pub fn set_size(&self, width: i32, height: i32) {
    unsafe { fermium::SDL_SetWindowSize(self.ptr, width, height) }
  }

  /// Sets the maximum logical size of the window.
  pub fn set_maximum_size(&self, width: i32, height: i32) {
    unsafe { fermium::SDL_SetWindowMaximumSize(self.ptr, width, height) }
  }

  /// Sets the minimum logical size of the window.
  pub fn set_minimum_size(&self, width: i32, height: i32) {
    unsafe { fermium::SDL_SetWindowMinimumSize(self.ptr, width, height) }
  }

  /// Obtains info about the fullscreen settings of the window.
  ///
  /// Use this function to get info about the display mode that the Window uses when it's fullscreen.
  pub fn window_display_mode(&self) -> Result<DisplayMode, String> {
    let mut mode = fermium::SDL_DisplayMode::default();
    let out = unsafe { fermium::SDL_GetWindowDisplayMode(self.ptr, &mut mode) };
    if out == 0 {
      Ok(DisplayMode::from(mode))
    } else {
      Err(get_error())
    }
  }

  /// Obtains info about the monitor settings that the center of the window is being displayed on.
  ///
  /// Use this function to get information about the Desktop display mode (even if a Window is currently fullscreen).
  pub fn desktop_display_mode(&self) -> Result<DisplayMode, String> {
    let index = unsafe { fermium::SDL_GetWindowDisplayIndex(self.ptr) };
    if index < 0 {
      return Err(get_error());
    }
    let mut mode = fermium::SDL_DisplayMode::default();
    let out = unsafe { fermium::SDL_GetDesktopDisplayMode(index, &mut mode) };
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
        let sdl_mode: fermium::SDL_DisplayMode = mode.into();
        unsafe { fermium::SDL_SetWindowDisplayMode(self.ptr, &sdl_mode) }
      }
      None => unsafe { fermium::SDL_SetWindowDisplayMode(self.ptr, null_mut()) },
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
    let out = unsafe { fermium::SDL_SetWindowFullscreen(self.ptr, style as u32) };
    if out == 0 {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  /// Attempts to promote this `Window` into a [RendererWindow](RendererWindow).
  ///
  /// You can request details via the flags and also `Some(index)` for a
  /// particular driver index. If you request `None` then the first driver index
  /// that supports the flags will be suggested.
  ///
  /// ## Failure
  ///
  /// Well [the wiki](https://wiki.libsdl.org/SDL_CreateRenderer) isn't super
  /// precise about things that can make it fail, but at a minimum if you
  /// request an impossible situation it'll probably fail.
  ///
  /// In case of failure, you get both this window back as a normal window and
  /// also the error message.
  pub unsafe fn try_into_renderer(
    self,
    driver_index: Option<usize>,
    flags: RendererFlags,
  ) -> Result<RendererWindow<'sdl>, (Self, String)> {
    let index = driver_index.map(|u| u as i32).unwrap_or(-1);
    let ptr = fermium::SDL_CreateRenderer(self.ptr, index, flags.0 as u32);
    if ptr.is_null() {
      Err((self, get_error()))
    } else {
      Ok(RendererWindow { ptr, window: self })
    }
  }

  /// Attempts to promote this `Window` into a [GLWindow](GLWindow).
  ///
  /// ## Failure
  ///
  /// OpenGL is a nightmare and context creation can fail because of cosmic rays
  /// or really even if there aren't cosmic rays.
  pub fn try_into_gl(self) -> Result<GLWindow<'sdl>, (Self, String)> {
    let ctx = unsafe { fermium::SDL_GL_CreateContext(self.ptr) };
    if ctx.is_null() {
      Err((self, get_error()))
    } else {
      Ok(GLWindow { ctx, window: self })
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
impl From<fermium::SDL_DisplayMode> for DisplayMode {
  fn from(sdl_mode: fermium::SDL_DisplayMode) -> Self {
    Self {
      format: PixelFormatEnum::from(sdl_mode.format as fermium::SDL_PixelFormatEnum),
      width: sdl_mode.w,
      height: sdl_mode.h,
      refresh_rate: sdl_mode.refresh_rate,
      driver_data: sdl_mode.driverdata,
    }
  }
}
impl From<DisplayMode> for fermium::SDL_DisplayMode {
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
