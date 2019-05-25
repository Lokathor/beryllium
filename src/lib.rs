#![warn(missing_docs)]
#![deny(missing_debug_implementations)]

//! An opinionated set of "high level" wrappers for the
//! [fermium](https://github.com/Lokathor/fermium) SDL2 bindings.

use core::{marker::PhantomData, ptr::null_mut, slice::from_raw_parts};
use fermium::{SDL_EventType::*, SDL_RendererFlags::*, SDL_WindowFlags::*, *};
use libc::c_char;
use phantom_fields::phantom_fields;

/// A version number.
#[derive(Debug, Default, Clone, Copy)]
#[allow(missing_docs)]
pub struct Version {
  pub major: u8,
  pub minor: u8,
  pub patch: u8,
}

/// Gets the version of SDL2 being used at runtime.
///
/// This might be later than the one you compiled with, but it will be fully
/// SemVer compatible.
///
/// ```rust
/// let v = beryllium::version();
/// assert_eq!(v.major, 2);
/// assert!(v.minor >= 0);
/// assert!(v.patch >= 9);
/// ```
pub fn version() -> Version {
  let mut sdl_version = SDL_version::default();
  unsafe { SDL_GetVersion(&mut sdl_version) };
  Version {
    major: sdl_version.major,
    minor: sdl_version.minor,
    patch: sdl_version.patch,
  }
}

/// Obtains the current SDL2 error string.
///
/// You should never need to call this yourself, but I guess you can if you
/// really want.
pub fn get_error() -> String {
  unsafe {
    let base = SDL_GetError();
    let len = SDL_strlen(base);
    let useful_bytes = from_raw_parts(base as *const u8, len);
    String::from_utf8_lossy(useful_bytes).into_owned()
  }
}

/// The kind of message box you wish to show.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum MessageBox {
  Error = fermium::SDL_MessageBoxFlags::SDL_MESSAGEBOX_ERROR,
  Warning = fermium::SDL_MessageBoxFlags::SDL_MESSAGEBOX_WARNING,
  Information = fermium::SDL_MessageBoxFlags::SDL_MESSAGEBOX_INFORMATION,
}

/// Shows a simple stand alone message box.
///
/// This doesn't require SDL2 to be initialized. If initialization was attempted
/// and then failed because of no possible video target then this call is very
/// likely to also fail.
///
/// # Safety
///
/// As with all GUI things, you must only call this from the main thread.
pub unsafe fn show_simple_message_box(
  box_type: MessageBox, title: &str, message: &str,
) -> Result<(), String> {
  let title_null: Vec<u8> = title.bytes().chain(Some(0)).collect();
  let message_null: Vec<u8> = message.bytes().chain(Some(0)).collect();
  let output = SDL_ShowSimpleMessageBox(
    box_type as u32,
    title_null.as_ptr() as *const c_char,
    message_null.as_ptr() as *const c_char,
    null_mut(),
  );
  if output == 0 {
    Ok(())
  } else {
    Err(get_error())
  }
}

/// Initializes SDL2 and gives you a token as proof, or an error message.
///
/// # Safety
///
/// * This can only be called from the main thread (that's just a
///   [macOS](https://tinyurl.com/y5bv7g4v) limit built into Cocoa)
/// * you cannot double initialize SDL2.
pub unsafe fn init() -> Result<SDLToken, String> {
  if SDL_Init(SDL_INIT_EVERYTHING) == 0 {
    Ok(SDLToken {
      _marker: PhantomData,
    })
  } else {
    Err(get_error())
  }
}

/// The `SDLToken` is proof that you have initialized SDL2.
///
/// Most of SDL2 requires you to have performed initialization, and so most of
/// its abilities are either methods off of this struct or off of things that
/// you make from methods of this struct.
#[derive(Debug)]
pub struct SDLToken {
  _marker: PhantomData<*mut u8>,
}
impl Drop for SDLToken {
  fn drop(&mut self) {
    unsafe { SDL_Quit() }
  }
}
#[test]
fn test_sdl_token_zero_size() {
  assert_eq!(core::mem::size_of::<SDLToken>(), 0)
}
impl SDLToken {
  /// Creates a new window, or gives an error message.
  ///
  /// Note that not all possible flags have an effect! See [the
  /// wiki](https://wiki.libsdl.org/SDL_CreateWindow) for guidance.
  pub fn create_window<'sdl>(
    &'sdl self, title: &str, x: i32, y: i32, w: i32, h: i32, flags: WindowFlags,
  ) -> Result<Window<'sdl>, String> {
    let title_null: Vec<u8> = title.bytes().chain(Some(0)).collect();
    let ptr = unsafe {
      SDL_CreateWindow(
        title_null.as_ptr() as *const c_char,
        x,
        y,
        w,
        h,
        flags.0 as u32,
      )
    };
    if ptr.is_null() {
      Err(get_error())
    } else {
      Ok(Window {
        ptr,
        _marker: PhantomData,
      })
    }
  }

  /// Creates a new [Surface](Surface) with the desired format, or error.
  ///
  /// See [the wiki page](https://wiki.libsdl.org/SDL_CreateRGBSurface)
  pub fn create_rgb_surface<'sdl>(
    &'sdl self, width: i32, height: i32, format: SurfaceFormat,
  ) -> Result<Surface<'sdl>, String> {
    let (depth, r_mask, g_mask, b_mask, a_mask) = match format {
      SurfaceFormat::Indexed4 => (4, 0, 0, 0, 0),
      SurfaceFormat::Indexed8 => (8, 0, 0, 0, 0),
      SurfaceFormat::Direct16 {
        r_mask,
        g_mask,
        b_mask,
        a_mask,
      } => (16, r_mask, g_mask, b_mask, a_mask),
      SurfaceFormat::Direct24 {
        r_mask,
        g_mask,
        b_mask,
        a_mask,
      } => (24, r_mask, g_mask, b_mask, a_mask),
      SurfaceFormat::Direct32 {
        r_mask,
        g_mask,
        b_mask,
        a_mask,
      } => (32, r_mask, g_mask, b_mask, a_mask),
    };
    let ptr: *mut SDL_Surface =
      unsafe { SDL_CreateRGBSurface(0, width, height, depth, r_mask, g_mask, b_mask, a_mask) };
    if ptr.is_null() {
      Err(get_error())
    } else {
      Ok(Surface {
        ptr,
        _marker: PhantomData,
      })
    }
  }

  /// Polls for an event, getting it out of the queue if one is there.
  pub fn poll_event(&self) -> Option<Event> {
    unsafe {
      let mut event = SDL_Event::default();
      if SDL_PollEvent(&mut event) == 1 {
        Some(Event::from(event))
      } else {
        None
      }
    }
  }
}

/// Flags that a window might have.
///
/// This is for use with [create_window](SDLToken::create_window) as well as
/// other methods that examine the state of a window.
#[derive(Debug, Default, Clone, Copy)]
#[repr(transparent)]
pub struct WindowFlags(SDL_WindowFlags::Type);
#[allow(bad_style)]
type SDL_WindowFlags_Type = SDL_WindowFlags::Type;
#[allow(missing_docs)]
impl WindowFlags {
  phantom_fields! {
    self.0: SDL_WindowFlags_Type,
    fullscreen: SDL_WINDOW_FULLSCREEN,
    opengl: SDL_WINDOW_OPENGL,
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

/// Flags for renderer creation.
///
/// See [Window::create_renderer](Window::create_renderer]
#[derive(Debug, Default, Clone, Copy)]
#[repr(transparent)]
pub struct RendererFlags(SDL_RendererFlags::Type);
#[allow(bad_style)]
type SDL_RendererFlags_Type = SDL_RendererFlags::Type;
#[allow(missing_docs)]
impl RendererFlags {
  phantom_fields! {
    self.0: SDL_RendererFlags_Type,
    accelerated: SDL_RENDERER_ACCELERATED,
    present_vsync: SDL_RENDERER_PRESENTVSYNC,
    software: SDL_RENDERER_SOFTWARE,
    target_texture: SDL_RENDERER_TARGETTEXTURE,
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
  ptr: *mut SDL_Window,
  _marker: PhantomData<&'sdl SDLToken>,
}
impl<'sdl> Drop for Window<'sdl> {
  fn drop(&mut self) {
    unsafe { SDL_DestroyWindow(self.ptr) }
  }
}
impl<'sdl> Window<'sdl> {
  /// Like the [show_simple_message_box](show_simple_message_box) function, but
  /// modal to the `Window`.
  ///
  /// Because you need a valid `Window` to call this method, we don't need to
  /// mark it as `unsafe`.
  pub fn show_simple_message_box(
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
}

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
  pub const DIRECT16_DEFAULT: Self = SurfaceFormat::Direct16 {
    r_mask: 0,
    g_mask: 0,
    b_mask: 0,
    a_mask: 0,
  };
  /// Alias for the default Direct24 surface format.
  pub const DIRECT24_DEFAULT: Self = SurfaceFormat::Direct24 {
    r_mask: 0,
    g_mask: 0,
    b_mask: 0,
    a_mask: 0,
  };
  /// Alias for the default Direct32 surface format.
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
  ptr: *mut SDL_Surface,
  _marker: PhantomData<&'sdl SDLToken>,
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
  pub unsafe fn lock_edit<'surface, F: FnMut(*mut u8)>(
    &'surface mut self, mut op: F,
  ) -> Result<(), String> {
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

  //TODO: clip_rect
}

/// Handle to some SDL2 rendering state.
///
/// Helps you do things like upload data to the GPU and blit image data around.
#[derive(Debug)]
#[repr(transparent)]
pub struct Renderer<'sdl, 'win> {
  ptr: *mut SDL_Renderer,
  _marker: PhantomData<&'win Window<'sdl>>,
}
impl<'sdl, 'win> Drop for Renderer<'sdl, 'win> {
  fn drop(&mut self) {
    unsafe { SDL_DestroyRenderer(self.ptr) }
  }
}
impl<'sdl, 'win> Renderer<'sdl, 'win> {
  /// Makes a texture with the contents of the surface specified.
  ///
  /// The TextureAccess hint for textures from this is "static".
  ///
  /// The pixel format might be different from the surface's pixel format.
  pub fn create_texture_from_surface<'ren>(
    &'ren self, surf: &Surface,
  ) -> Result<Texture<'sdl, 'win, 'ren>, String> {
    let ptr: *mut SDL_Texture = unsafe { SDL_CreateTextureFromSurface(self.ptr, surf.ptr) };
    if ptr.is_null() {
      Err(get_error())
    } else {
      Ok(Texture {
        ptr,
        _marker: PhantomData,
      })
    }
  }

  /// Clears the entire target, ignoring the viewport and clip rect.
  pub fn clear(&self) -> Result<(), String> {
    if unsafe { SDL_RenderClear(self.ptr) } == 0 {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  /// Blits the texture to the rendering target.
  ///
  /// * `src`: Optional clip rect of where to copy _from_. If None, the whole
  ///   texture is used.
  /// * `dst`: Optional clip rect of where to copy data _to_. If None, the whole
  ///   render target is used.
  ///
  /// The image is stretched as necessary if the `src` and `dst` are different
  /// sizes. This is a GPU operation, so it's fast no matter how much upscale or
  /// downscale you do.
  pub fn copy(&self, t: &Texture, src: Option<Rect>, dst: Option<Rect>) -> Result<(), String> {
    unsafe {
      let src_ptr = core::mem::transmute::<Option<&Rect>, *const SDL_Rect>(src.as_ref());
      let dst_ptr = core::mem::transmute::<Option<&Rect>, *const SDL_Rect>(dst.as_ref());
      if SDL_RenderCopy(self.ptr, t.ptr, src_ptr, dst_ptr) == 0 {
        Ok(())
      } else {
        Err(get_error())
      }
    }
  }

  /// Presents the backbuffer to the user.
  ///
  /// After a present, all backbuffer data should be assumed to be invalid, and
  /// you should also clear the backbuffer before doing the next render pass
  /// even if you intend to write to every pixel.
  pub fn present(&self) {
    unsafe { SDL_RenderPresent(self.ptr) };
  }
}

/// Handle to a "texture", a GPU-side image.
///
/// This is harder to directly edit, but operations are faster, and you can
/// display it in the Window.
#[derive(Debug)]
#[repr(transparent)]
pub struct Texture<'sdl, 'win, 'ren> {
  ptr: *mut SDL_Texture,
  _marker: PhantomData<&'ren Renderer<'sdl, 'win>>,
}
impl<'sdl, 'win, 'ren> Drop for Texture<'sdl, 'win, 'ren> {
  fn drop(&mut self) {
    unsafe { SDL_DestroyTexture(self.ptr) }
  }
}

/// TODO: make this a proper newtype
pub type MouseButtonState = u32;
/// TODO: make this a proper newtype
pub type MouseButton = u8;

/// The various events that can happen.
#[derive(Debug, Clone, Copy)]
pub enum Event {
  /// Quit was requested by the user
  Quit {
    /// Time, in milliseconds, since SDL2 was initialized.
    timestamp: u32,
  },
  /// Event for any time the user moves the mouse within a window, or if
  /// `warp_mouse_in_window` is called.
  MouseMotion {
    /// Time, in milliseconds, since SDL2 was initialized.
    timestamp: u32,
    /// Window with mouse focus, if any
    window_id: u32,
    /// The mouse that generated this event. If `None` it was a touch event.
    mouse_id: Option<u32>,
    /// State of the mouse buttons during this event
    state: MouseButtonState,
    /// X, relative to the window
    x: i32,
    /// Y, relative to the window
    y: i32,
    /// Change in X position
    delta_x: i32,
    /// Change in Y position
    delta_y: i32,
  },
  /// Generated whenever a mouse button is pressed or released.
  MouseButtonEvent {
    /// Time, in milliseconds, since SDL2 was initialized.
    timestamp: u32,
    /// Window with mouse focus, if any
    window_id: u32,
    /// The mouse that generated this event. If `None` it was a touch event.
    mouse_id: Option<u32>,
    /// The button that changed
    button: MouseButton,
    /// If the button is now pressed or released
    is_pressed: bool,
    /// 1 for single-click, 2 for double-click, etc
    clicks: u8,
    /// X, relative to the window
    x: i32,
    /// Y, relative to the window
    y: i32,
  },
  /// Generated whenever the user moves the mouse wheel.
  MouseWheel {
    /// Time, in milliseconds, since SDL2 was initialized.
    timestamp: u32,
    /// Window with mouse focus, if any
    window_id: u32,
    /// The mouse that generated this event. If `None` it was a touch event.
    mouse_id: Option<u32>,
    /// Horizontal scroll, negative Left or positive Right
    x: i32,
    /// Vertical scroll, negative to User or positive away from User
    y: i32,
    /// Mouse wheel isn't consistent on all platforms. If this bool is set, the
    /// meaning of the `x` and `y` field is inverted compared to normal.
    is_flipped: bool,
  },
  /// It's always possible that we'll load some future version which will have
  /// event variants we don't understand, which we have to just ignore.
  UnknownEventType,
}
impl From<SDL_Event> for Event {
  /// Parses "without fail", but will turn unknown events into `UnknownEventType`.
  ///
  /// So, it's not lossless I guess. Whatever.
  fn from(event: SDL_Event) -> Self {
    unsafe {
      match event.type_ as SDL_EventType::Type {
        SDL_QUIT => Event::Quit {
          timestamp: event.quit.timestamp,
        },
        SDL_MOUSEMOTION => Event::MouseMotion {
          timestamp: event.motion.timestamp,
          window_id: event.motion.windowID,
          mouse_id: if event.motion.which == SDL_TOUCH_MOUSEID {
            None
          } else {
            Some(event.motion.which)
          },
          state: event.motion.state,
          x: event.motion.x,
          y: event.motion.y,
          delta_x: event.motion.xrel,
          delta_y: event.motion.yrel,
        },
        SDL_MOUSEBUTTONDOWN | SDL_MOUSEBUTTONUP => Event::MouseButtonEvent {
          timestamp: event.button.timestamp,
          window_id: event.button.windowID,
          mouse_id: if event.button.which == SDL_TOUCH_MOUSEID {
            None
          } else {
            Some(event.button.which)
          },
          button: event.button.button,
          is_pressed: u32::from(event.button.state) == SDL_PRESSED,
          clicks: event.button.clicks,
          x: event.button.x,
          y: event.button.y,
        },
        SDL_MOUSEWHEEL => Event::MouseWheel {
          timestamp: event.wheel.timestamp,
          window_id: event.wheel.windowID,
          mouse_id: if event.wheel.which == SDL_TOUCH_MOUSEID {
            None
          } else {
            Some(event.wheel.which)
          },
          x: event.wheel.x,
          y: event.wheel.y,
          is_flipped: event.wheel.direction as fermium::SDL_MouseWheelDirection::Type
            == fermium::SDL_MouseWheelDirection::SDL_MOUSEWHEEL_FLIPPED,
        },
        _ => Event::UnknownEventType,
      }
    }
  }
}

/// A standard color, separate from any format.
///
/// Use [PixelFormat::map_rgb](PixelFormat::map_rgb) to turn this into color
/// data in a particular pixel format.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Color {
  r: u8,
  g: u8,
  b: u8,
  a: u8,
}
impl From<SDL_Color> for Color {
  fn from(other: SDL_Color) -> Self {
    Self {
      r: other.r,
      g: other.g,
      b: other.b,
      a: other.a,
    }
  }
}

/// Rectangle struct, origin at the upper left.
///
/// Naturally, having the origin at the upper left is a terrible and heretical
/// coordinate system to use but that's what SDL2 does so that's what we're
/// stuck with.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Rect {
  x: i32,
  y: i32,
  w: i32,
  h: i32,
}
impl From<SDL_Rect> for Rect {
  fn from(other: SDL_Rect) -> Self {
    Self {
      x: other.x,
      y: other.y,
      w: other.w,
      h: other.h,
    }
  }
}
