#![warn(missing_docs)]
#![deny(missing_debug_implementations)]

//! An opinionated set of "high level" wrappers for the
//! [fermium](https://github.com/Lokathor/fermium) SDL2 bindings.

use core::{marker::PhantomData, ptr::null_mut, slice::from_raw_parts};
use fermium::{
  SDL_EventType::*, SDL_GameControllerAxis::*, SDL_GameControllerButton::*, SDL_RendererFlags::*,
  SDL_WindowFlags::*, SDL_bool::*, _bindgen_ty_1::*, _bindgen_ty_2::*, _bindgen_ty_3::*,
  _bindgen_ty_4::*, _bindgen_ty_5::*, _bindgen_ty_6::*, *,
};

use libc::c_char;
use phantom_fields::phantom_fields;

mod surface;
pub use surface::*;

mod event;
pub use event::*;

mod controller;
pub use controller::*;

/// Grabs up the data from a null terminated string pointer.
unsafe fn gather_string(ptr: *const c_char) -> String {
  let len = SDL_strlen(ptr);
  let useful_bytes = from_raw_parts(ptr as *const u8, len);
  String::from_utf8_lossy(useful_bytes).into_owned()
}

/// A version number.
#[derive(Debug, Default, Clone, Copy)]
#[allow(missing_docs)]
pub struct Version {
  pub major: u8,
  pub minor: u8,
  pub patch: u8,
}
impl From<SDL_version> for Version {
  fn from(input: SDL_version) -> Self {
    Self {
      major: input.major,
      minor: input.minor,
      patch: input.patch,
    }
  }
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
  Version::from(sdl_version)
}

/// Obtains the current SDL2 error string.
///
/// You should never need to call this yourself, but I guess you can if you
/// really want.
pub fn get_error() -> String {
  unsafe { gather_string(SDL_GetError()) }
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
/// * This can only be called from the main thread (because of a
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

  /// Obtains the number of joysticks connected to the system.
  pub fn number_of_joysticks(&self) -> Result<u32, String> {
    let out = unsafe { SDL_NumJoysticks() };
    if out < 0 {
      Err(get_error())
    } else {
      // Note(Lokathor): since it's supposed to be an "index" we'll pretend that
      // the ID values are unsigned values, since that's more like the Rust
      // index convention.
      Ok(out as u32)
    }
  }

  /// Says if the joystick index supports the Controller API.
  pub fn joystick_is_game_controller(&self, index: u32) -> bool {
    SDL_TRUE == unsafe { SDL_IsGameController(index as i32) }
  }

  /// Given a joystick index, attempts to get the Controller name, if any.
  pub fn controller_name(&self, index: u32) -> Option<String> {
    let ptr = unsafe { SDL_GameControllerNameForIndex(index as i32) };
    if ptr.is_null() {
      None
    } else {
      unsafe { Some(gather_string(ptr)) }
    }
  }

  /// Attempts to open the given index as a Controller.
  pub fn open_controller<'sdl>(&'sdl self, index: u32) -> Result<Controller<'sdl>, String> {
    let ptr = unsafe { SDL_GameControllerOpen(index as i32) };
    if ptr.is_null() {
      Err(get_error())
    } else {
      Ok(Controller {
        ptr,
        _marker: PhantomData,
      })
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
/// coordinate system to use, but that's what SDL2 does so that's what we're
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

/// The various pixel formats that SDL2 supports.
///
/// There's various checks you can perform on each pixel format.
///
/// Note that the "fourcc" formats, anything that gives `true` from the
/// [is_fourcc](PixelFormat:is_fourcc) method, are industry specified special
/// values, and do not follow SDL2's bit packing scheme. In other words, the
/// output they produce for any of the other check methods is not to really be
/// trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum PixelFormat {
  Unknown = SDL_PIXELFORMAT_UNKNOWN,
  Index1lsb = SDL_PIXELFORMAT_INDEX1LSB,
  Index1msb = SDL_PIXELFORMAT_INDEX1MSB,
  Index4lsb = SDL_PIXELFORMAT_INDEX4LSB,
  Index4msb = SDL_PIXELFORMAT_INDEX4MSB,
  Index8 = SDL_PIXELFORMAT_INDEX8,
  RGB332 = SDL_PIXELFORMAT_RGB332,
  RGB444 = SDL_PIXELFORMAT_RGB444,
  RGB555 = SDL_PIXELFORMAT_RGB555,
  BGR555 = SDL_PIXELFORMAT_BGR555,
  ARGB4444 = SDL_PIXELFORMAT_ARGB4444,
  RGBA4444 = SDL_PIXELFORMAT_RGBA4444,
  ABGR4444 = SDL_PIXELFORMAT_ABGR4444,
  BGRA4444 = SDL_PIXELFORMAT_BGRA4444,
  ARGB1555 = SDL_PIXELFORMAT_ARGB1555,
  RGBA5551 = SDL_PIXELFORMAT_RGBA5551,
  ABGR1555 = SDL_PIXELFORMAT_ABGR1555,
  BGRA5551 = SDL_PIXELFORMAT_BGRA5551,
  RGB565 = SDL_PIXELFORMAT_RGB565,
  BGR565 = SDL_PIXELFORMAT_BGR565,
  RGB24 = SDL_PIXELFORMAT_RGB24,
  BGR24 = SDL_PIXELFORMAT_BGR24,
  RGB888 = SDL_PIXELFORMAT_RGB888,
  RGBX8888 = SDL_PIXELFORMAT_RGBX8888,
  BGR888 = SDL_PIXELFORMAT_BGR888,
  BGRX8888 = SDL_PIXELFORMAT_BGRX8888,
  ARGB8888 = SDL_PIXELFORMAT_ARGB8888,
  RGBA8888 = SDL_PIXELFORMAT_RGBA8888,
  ABGR8888 = SDL_PIXELFORMAT_ABGR8888,
  BGRA8888 = SDL_PIXELFORMAT_BGRA8888,
  ARGB2101010 = SDL_PIXELFORMAT_ARGB2101010,
  /// Planar mode: Y + V + U (3 planes)
  YV12 = SDL_PIXELFORMAT_YV12,
  /// Planar mode: Y + U + V (3 planes)
  IYUV = SDL_PIXELFORMAT_IYUV,
  /// Packed mode: Y0+U0+Y1+V0 (1 plane)
  YUY2 = SDL_PIXELFORMAT_YUY2,
  /// Packed mode: U0+Y0+V0+Y1 (1 plane)
  UYVY = SDL_PIXELFORMAT_UYVY,
  /// Packed mode: Y0+V0+Y1+U0 (1 plane)
  YVYU = SDL_PIXELFORMAT_YVYU,
  /// Planar mode: Y + U/V interleaved (2 planes)
  NV12 = SDL_PIXELFORMAT_NV12,
  /// Planar mode: Y + V/U interleaved (2 planes)
  NV21 = SDL_PIXELFORMAT_NV21,
  /// Android video texture format
  ExternalOES = SDL_PIXELFORMAT_EXTERNAL_OES,
}
#[cfg(target_endian = "big")]
impl PixelFormat {
  /// Platform specific alias for RGBA
  pub const RGBA32: Self = PixelFormat::RGBA8888;
  /// Platform specific alias for ARGB
  pub const ARGB32: Self = PixelFormat::ARGB8888;
  /// Platform specific alias for BGRA
  pub const BGRA32: Self = PixelFormat::BGRA8888;
  /// Platform specific alias for ABGR
  pub const ABGR32: Self = PixelFormat::ABGR8888;
}
#[cfg(target_endian = "little")]
impl PixelFormat {
  /// Platform specific alias for RGBA
  pub const RGBA32: Self = PixelFormat::ABGR8888;
  /// Platform specific alias for ARGB
  pub const ARGB32: Self = PixelFormat::BGRA8888;
  /// Platform specific alias for BGRA
  pub const BGRA32: Self = PixelFormat::ARGB8888;
  /// Platform specific alias for ABGR
  pub const ABGR32: Self = PixelFormat::RGBA8888;
}
impl From<fermium::_bindgen_ty_6::Type> for PixelFormat {
  fn from(pf: fermium::_bindgen_ty_6::Type) -> Self {
    match pf {
      SDL_PIXELFORMAT_INDEX1LSB => PixelFormat::Index1lsb,
      SDL_PIXELFORMAT_INDEX1MSB => PixelFormat::Index1msb,
      SDL_PIXELFORMAT_INDEX4LSB => PixelFormat::Index4lsb,
      SDL_PIXELFORMAT_INDEX4MSB => PixelFormat::Index4msb,
      SDL_PIXELFORMAT_INDEX8 => PixelFormat::Index8,
      SDL_PIXELFORMAT_RGB332 => PixelFormat::RGB332,
      SDL_PIXELFORMAT_RGB444 => PixelFormat::RGB444,
      SDL_PIXELFORMAT_RGB555 => PixelFormat::RGB555,
      SDL_PIXELFORMAT_BGR555 => PixelFormat::BGR555,
      SDL_PIXELFORMAT_ARGB4444 => PixelFormat::ARGB4444,
      SDL_PIXELFORMAT_RGBA4444 => PixelFormat::RGBA4444,
      SDL_PIXELFORMAT_ABGR4444 => PixelFormat::ABGR4444,
      SDL_PIXELFORMAT_BGRA4444 => PixelFormat::BGRA4444,
      SDL_PIXELFORMAT_ARGB1555 => PixelFormat::ARGB1555,
      SDL_PIXELFORMAT_RGBA5551 => PixelFormat::RGBA5551,
      SDL_PIXELFORMAT_ABGR1555 => PixelFormat::ABGR1555,
      SDL_PIXELFORMAT_BGRA5551 => PixelFormat::BGRA5551,
      SDL_PIXELFORMAT_RGB565 => PixelFormat::RGB565,
      SDL_PIXELFORMAT_BGR565 => PixelFormat::BGR565,
      SDL_PIXELFORMAT_RGB24 => PixelFormat::RGB24,
      SDL_PIXELFORMAT_BGR24 => PixelFormat::BGR24,
      SDL_PIXELFORMAT_RGB888 => PixelFormat::RGB888,
      SDL_PIXELFORMAT_RGBX8888 => PixelFormat::RGBX8888,
      SDL_PIXELFORMAT_BGR888 => PixelFormat::BGR888,
      SDL_PIXELFORMAT_BGRX8888 => PixelFormat::BGRX8888,
      SDL_PIXELFORMAT_ARGB8888 => PixelFormat::ARGB8888,
      SDL_PIXELFORMAT_RGBA8888 => PixelFormat::RGBA8888,
      SDL_PIXELFORMAT_ABGR8888 => PixelFormat::ABGR8888,
      SDL_PIXELFORMAT_BGRA8888 => PixelFormat::BGRA8888,
      SDL_PIXELFORMAT_ARGB2101010 => PixelFormat::ARGB2101010,
      SDL_PIXELFORMAT_YV12 => PixelFormat::YV12,
      SDL_PIXELFORMAT_IYUV => PixelFormat::IYUV,
      SDL_PIXELFORMAT_YUY2 => PixelFormat::YUY2,
      SDL_PIXELFORMAT_UYVY => PixelFormat::UYVY,
      SDL_PIXELFORMAT_YVYU => PixelFormat::YVYU,
      SDL_PIXELFORMAT_NV12 => PixelFormat::NV12,
      SDL_PIXELFORMAT_NV21 => PixelFormat::NV21,
      SDL_PIXELFORMAT_EXTERNAL_OES => PixelFormat::ExternalOES,
      _ => PixelFormat::Unknown,
    }
  }
}
impl PixelFormat {
  /// The type of the pixel format.
  ///
  /// All unknown types convert to `PixelType::Unknown`, of course.
  pub fn pixel_type(self) -> PixelType {
    match ((self as u32 >> 24) & 0x0F) as fermium::_bindgen_ty_1::Type {
      SDL_PIXELTYPE_INDEX1 => PixelType::Index1,
      SDL_PIXELTYPE_INDEX4 => PixelType::Index4,
      SDL_PIXELTYPE_INDEX8 => PixelType::Index8,
      SDL_PIXELTYPE_PACKED8 => PixelType::Packed8,
      SDL_PIXELTYPE_PACKED16 => PixelType::Packed16,
      SDL_PIXELTYPE_PACKED32 => PixelType::Packed32,
      SDL_PIXELTYPE_ARRAYU8 => PixelType::ArrayU8,
      SDL_PIXELTYPE_ARRAYU16 => PixelType::ArrayU16,
      SDL_PIXELTYPE_ARRAYU32 => PixelType::ArrayU32,
      SDL_PIXELTYPE_ARRAYF16 => PixelType::ArrayF16,
      SDL_PIXELTYPE_ARRAYF32 => PixelType::ArrayF32,
      _ => PixelType::Unknown,
    }
  }

  /// Ordering of channel or bits in the pixel format.
  ///
  /// Unknown values convert to one of the `None` variants.
  pub fn pixel_order(self) -> PixelOrder {
    let bits = (self as u32 >> 20) & 0x0F;
    if self.is_packed() {
      match bits as fermium::_bindgen_ty_4::Type {
        SDL_PACKEDORDER_ABGR => PixelOrder::Packed(PackedPixelOrder::ABGR),
        SDL_PACKEDORDER_ARGB => PixelOrder::Packed(PackedPixelOrder::ARGB),
        SDL_PACKEDORDER_BGRA => PixelOrder::Packed(PackedPixelOrder::BGRA),
        SDL_PACKEDORDER_BGRX => PixelOrder::Packed(PackedPixelOrder::BGRX),
        SDL_PACKEDORDER_RGBA => PixelOrder::Packed(PackedPixelOrder::RGBA),
        SDL_PACKEDORDER_RGBX => PixelOrder::Packed(PackedPixelOrder::RGBX),
        SDL_PACKEDORDER_XBGR => PixelOrder::Packed(PackedPixelOrder::XBGR),
        SDL_PACKEDORDER_XRGB => PixelOrder::Packed(PackedPixelOrder::XRGB),
        _ => PixelOrder::Packed(PackedPixelOrder::None),
      }
    } else if self.is_array() {
      match bits as fermium::_bindgen_ty_4::Type {
        SDL_ARRAYORDER_ABGR => PixelOrder::Array(ArrayPixelOrder::ABGR),
        SDL_ARRAYORDER_ARGB => PixelOrder::Array(ArrayPixelOrder::ARGB),
        SDL_ARRAYORDER_BGR => PixelOrder::Array(ArrayPixelOrder::BGR),
        SDL_ARRAYORDER_BGRA => PixelOrder::Array(ArrayPixelOrder::BGRA),
        SDL_ARRAYORDER_RGB => PixelOrder::Array(ArrayPixelOrder::RGB),
        SDL_ARRAYORDER_RGBA => PixelOrder::Array(ArrayPixelOrder::RGBA),
        _ => PixelOrder::Array(ArrayPixelOrder::None),
      }
    } else {
      match bits as fermium::_bindgen_ty_2::Type {
        SDL_BITMAPORDER_1234 => PixelOrder::Bitmap(BitmapPixelOrder::_1234),
        SDL_BITMAPORDER_4321 => PixelOrder::Bitmap(BitmapPixelOrder::_4321),
        _ => PixelOrder::Bitmap(BitmapPixelOrder::None),
      }
    }
  }

  /// Channel bits pattern of the pixel format.
  ///
  /// Converts any possible unknown layout to `PixelLayout::None`.
  pub fn pixel_layout(self) -> PixelLayout {
    match ((self as u32 >> 16) & 0x0F) as fermium::_bindgen_ty_1::Type {
      SDL_PACKEDLAYOUT_332 => PixelLayout::_332,
      SDL_PACKEDLAYOUT_4444 => PixelLayout::_4444,
      SDL_PACKEDLAYOUT_1555 => PixelLayout::_1555,
      SDL_PACKEDLAYOUT_5551 => PixelLayout::_5551,
      SDL_PACKEDLAYOUT_565 => PixelLayout::_565,
      SDL_PACKEDLAYOUT_8888 => PixelLayout::_8888,
      SDL_PACKEDLAYOUT_2101010 => PixelLayout::_2101010,
      SDL_PACKEDLAYOUT_1010102 => PixelLayout::_1010102,
      _ => PixelLayout::None,
    }
  }

  /// Bits of color information per pixel.
  pub fn bits_per_pixel(self) -> u32 {
    (self as u32 >> 8) & 0xFF
  }

  /// Bytes used per pixel.
  ///
  /// Note: Formats with less than 8 bits per pixel give a result of 0 bytes per
  /// pixel. Weird and all, but that's how it is.
  pub fn bytes_per_pixel(self) -> u32 {
    if self.is_fourcc() {
      match self {
        PixelFormat::YUY2 | PixelFormat::UYVY | PixelFormat::YVYU => 2,
        _ => 1,
      }
    } else {
      self as u32 & 0xFF
    }
  }

  /// Is this format an indexed format?
  pub fn is_indexed(self) -> bool {
    !self.is_fourcc()
      && match self.pixel_type() {
        PixelType::Index1 | PixelType::Index4 | PixelType::Index8 => true,
        _ => false,
      }
  }

  /// Is this format a packed format?
  pub fn is_packed(self) -> bool {
    !self.is_fourcc()
      && match self.pixel_type() {
        PixelType::Packed8 | PixelType::Packed16 | PixelType::Packed32 => true,
        _ => false,
      }
  }

  /// Is this format a packed format?
  pub fn is_array(self) -> bool {
    !self.is_fourcc()
      && match self.pixel_type() {
        PixelType::ArrayU8
        | PixelType::ArrayU16
        | PixelType::ArrayU32
        | PixelType::ArrayF16
        | PixelType::ArrayF32 => true,
        _ => false,
      }
  }

  /// Is this format a format with an alpha channel?
  pub fn is_alpha(self) -> bool {
    match self.pixel_order() {
      PixelOrder::Packed(PackedPixelOrder::ARGB)
      | PixelOrder::Packed(PackedPixelOrder::RGBA)
      | PixelOrder::Packed(PackedPixelOrder::ABGR)
      | PixelOrder::Packed(PackedPixelOrder::BGRA)
      | PixelOrder::Array(ArrayPixelOrder::ARGB)
      | PixelOrder::Array(ArrayPixelOrder::RGBA)
      | PixelOrder::Array(ArrayPixelOrder::ABGR)
      | PixelOrder::Array(ArrayPixelOrder::BGRA) => true,
      _ => false,
    }
  }

  /// Is this a [four-character code](https://en.wikipedia.org/wiki/FourCC) format?
  ///
  /// True for pixel formats representing unique formats, for example YUV formats.
  pub fn is_fourcc(self) -> bool {
    (self as u32 > 0) && (((self as u32 >> 28) & 0x0F) != 1)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum PixelType {
  Unknown = SDL_PIXELTYPE_UNKNOWN,
  Index1 = SDL_PIXELTYPE_INDEX1,
  Index4 = SDL_PIXELTYPE_INDEX4,
  Index8 = SDL_PIXELTYPE_INDEX8,
  Packed8 = SDL_PIXELTYPE_PACKED8,
  Packed16 = SDL_PIXELTYPE_PACKED16,
  Packed32 = SDL_PIXELTYPE_PACKED32,
  ArrayU8 = SDL_PIXELTYPE_ARRAYU8,
  ArrayU16 = SDL_PIXELTYPE_ARRAYU16,
  ArrayU32 = SDL_PIXELTYPE_ARRAYU32,
  ArrayF16 = SDL_PIXELTYPE_ARRAYF16,
  ArrayF32 = SDL_PIXELTYPE_ARRAYF32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum PixelOrder {
  Bitmap(BitmapPixelOrder),
  Packed(PackedPixelOrder),
  Array(ArrayPixelOrder),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum BitmapPixelOrder {
  None = SDL_BITMAPORDER_NONE,
  _4321 = SDL_BITMAPORDER_4321,
  _1234 = SDL_BITMAPORDER_1234,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum PackedPixelOrder {
  None = SDL_PACKEDORDER_NONE,
  XRGB = SDL_PACKEDORDER_XRGB,
  RGBX = SDL_PACKEDORDER_RGBX,
  ARGB = SDL_PACKEDORDER_ARGB,
  RGBA = SDL_PACKEDORDER_RGBA,
  XBGR = SDL_PACKEDORDER_XBGR,
  BGRX = SDL_PACKEDORDER_BGRX,
  ABGR = SDL_PACKEDORDER_ABGR,
  BGRA = SDL_PACKEDORDER_BGRA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum ArrayPixelOrder {
  None = SDL_ARRAYORDER_NONE,
  RGB = SDL_ARRAYORDER_RGB,
  RGBA = SDL_ARRAYORDER_RGBA,
  ARGB = SDL_ARRAYORDER_ARGB,
  BGR = SDL_ARRAYORDER_BGR,
  BGRA = SDL_ARRAYORDER_BGRA,
  ABGR = SDL_ARRAYORDER_ABGR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum PixelLayout {
  None = SDL_PACKEDLAYOUT_NONE,
  _332 = SDL_PACKEDLAYOUT_332,
  _4444 = SDL_PACKEDLAYOUT_4444,
  _1555 = SDL_PACKEDLAYOUT_1555,
  _5551 = SDL_PACKEDLAYOUT_5551,
  _565 = SDL_PACKEDLAYOUT_565,
  _8888 = SDL_PACKEDLAYOUT_8888,
  _2101010 = SDL_PACKEDLAYOUT_2101010,
  _1010102 = SDL_PACKEDLAYOUT_1010102,
}
