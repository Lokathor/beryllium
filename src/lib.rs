#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![allow(clippy::needless_lifetimes)]
// The unsafe code relies on the idea that `usize` is at least `u32`
#![cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]

//! An opinionated set of "high level" wrappers for the
//! [fermium](https://github.com/Lokathor/fermium) SDL2 bindings.
//!
//! * Very little of SDL2 can be called before you initialize the library. As a
//!   result, there's only a very small number of functions available as top
//!   level functions:
//!   * [version](version) can be called to check what version of SDL2 is being
//!     used. Because of SDL2's [Dynamic API](https://tinyurl.com/y2fndelh), it
//!     is possible for a user to try and run the program using an future,
//!     SemVer compatible version of SDL2. It would be a great indignity if you
//!     didn't allow them to do this, but you can still check the version and
//!     log it perhaps.
//!   * [get_error](get_error) can be called at any time, though before you
//!     initialize SDL2 the error string will probably be blank. You _very
//!     rarely_ have to call this yourself. Any necessary error strings are
//!     almost always passed back to you as part of a `Result` type.
//!   * [lone_message_box](lone_message_box) opens a simple message box where
//!     you can display that some critical file is missing or something like
//!     that instead of failing with no message at all. It is `unsafe` because
//!     you must only call it from the `main` thread.
//!       * See also: [Window::modal_message_box](Window::modal_message_box)
//!   * [init](init) is how you initialize SDL2. If successful it gives you an
//!     [SDLToken](SDLToken) which has all the necessary methods to do
//!     everything else. It is `unsafe` because you must only call this from the
//!     `main` thread, and you must never double initialize SDL2.
//!
//! _Very little_ of SDL2 is thread-safe. Your code that interacts with SDL2
//! will mostly be locked into just the `main` thread. This isn't a huge deal in
//! practice, but it's something that people might want to know up font.
//!
//! ## Safety
//!
//! As much as possible, SDL2 is carefully wrapped into a safe interface for you
//! to use. That said, there's a few points that still can't be made 100% safe,
//! so you will have to use `unsafe` in some places.
//!
//! ## Lifetimes
//!
//! Sadly, we gotta track some lifetimes here. Anything with a heap allocation
//! or other OS resource needs to live within the lifetime of its parent. I
//! assure you that the lifetime tracking is happening via
//! [PhantomData](PhantomData), no runtime cost here.
//!
//! Almost all lifetime tracking is restricted to things needing to live no
//! longer than the life of the `SDLToken`, which isn't a very big deal at all.
//!
//! ## Naming
//!
//! I've attempted to stick to SDL2's naming for things unless there's an
//! obviously more Rusty name to use instead.
//!
//! The main exception I can think of is that all `AllocFoo`/`FreeFoo` and
//! `CreateFoo`/`DestroyFoo` pairs for creation and destruction have been
//! replaced with just using `new_foo` for creation and [Drop](Drop) for
//! cleanup.
//!
//! ## Organization
//!
//! Internally the code is split up into modules because that's easier to work
//! with, however the public API is that everything is just at the top level of
//! the crate because that's far easier to work with. The only down side to this
//! is that some compiler error messages will list the internal module name in
//! the path. It's a little annoying, but that's more the fault of `rustc` than
//! anything else.
//!
//! ## Failures
//!
//! If a call returns [Option](Option) or [Result](Result), I will make an
//! effort to document what's likely to cause that. However, it's always
//! possible that additional error conditions might exist.

use core::{
  convert::TryFrom,
  ffi::c_void,
  marker::PhantomData,
  ptr::{null, null_mut, NonNull},
  slice::from_raw_parts,
};
use fermium::{
  SDL_EventType::*, SDL_GLattr::*, SDL_GLcontextFlag::*, SDL_GLprofile::*,
  SDL_GameControllerAxis::*, SDL_GameControllerButton::*, SDL_Keymod::*, SDL_RendererFlags::*,
  SDL_Scancode::*, SDL_WindowEventID::*, SDL_WindowFlags::*, SDL_bool::*, _bindgen_ty_1::*,
  _bindgen_ty_2::*, _bindgen_ty_3::*, _bindgen_ty_4::*, _bindgen_ty_5::*, _bindgen_ty_6::*,
  _bindgen_ty_7::*, *,
};

use libc::c_char;
use phantom_fields::phantom_fields;

mod surface;
pub use surface::*;

mod event;
pub use event::*;

mod controller;
pub use controller::*;

mod audio;
pub use audio::*;

mod opengl;
pub use opengl::*;

mod pixel_format;
pub use pixel_format::*;

mod palette;
pub use palette::*;

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

/// Shows a basic, stand alone message box.
///
/// This doesn't require SDL2 to be initialized. If initialization was attempted
/// and then failed because of no possible video target then this call is very
/// likely to also fail.
///
/// # Safety
///
/// As with all GUI things, you must only call this from the main thread.
pub unsafe fn lone_message_box(
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

  /// Attempts to load the named dynamic library into the program.
  pub fn load_cdylib<'sdl>(&'sdl self, name: &str) -> Result<CDyLib<'sdl>, String> {
    let name_null: Vec<u8> = name.bytes().chain(Some(0)).collect();
    let ptr = unsafe { SDL_LoadObject(name_null.as_ptr() as *const c_char) };
    if ptr.is_null() {
      Err(get_error())
    } else {
      Ok(CDyLib {
        ptr,
        _marker: PhantomData,
      })
    }
  }

  /// Attempts to open a default audio device in "queue" mode.
  ///
  /// If successful, the device will initially be paused.
  pub fn open_default_audio_queue<'sdl>(
    &'sdl self, request: DefaultAudioQueueRequest,
  ) -> Result<AudioQueue<'sdl>, String> {
    //
    let mut desired_spec = SDL_AudioSpec::default();
    desired_spec.freq = request.frequency;
    desired_spec.format = request.format.0;
    desired_spec.channels = request.channels;
    desired_spec.samples = request.samples;
    //
    let mut changes = 0;
    if request.allow_frequency_change {
      changes |= SDL_AUDIO_ALLOW_FREQUENCY_CHANGE as i32;
    }
    if request.allow_format_change {
      changes |= SDL_AUDIO_ALLOW_FORMAT_CHANGE as i32;
    }
    if request.allow_channels_change {
      changes |= SDL_AUDIO_ALLOW_CHANNELS_CHANGE as i32;
    }
    //
    let mut obtained_spec = SDL_AudioSpec::default();
    //
    let audio_device_id =
      unsafe { SDL_OpenAudioDevice(null(), 0, &desired_spec, &mut obtained_spec, changes) };
    if audio_device_id == 0 {
      Err(get_error())
    } else {
      Ok(AudioQueue {
        dev: audio_device_id,
        frequency: obtained_spec.freq,
        format: AudioFormat(obtained_spec.format),
        channels: obtained_spec.channels,
        silence: obtained_spec.silence,
        sample_count: usize::from(obtained_spec.samples),
        buffer_size: obtained_spec.size as usize,
        _marker: PhantomData,
      })
    }
  }

  /// Attempts to set a given attribute, returns `true` if successful.
  ///
  /// Depending on the attribute, this can often be viewed as a "minimum
  /// request". Once you create the context you should examine it to see what
  /// you actually got.
  pub fn gl_set_attribute(&self, attr: GLattr, value: i32) -> bool {
    0 == unsafe { SDL_GL_SetAttribute(attr as fermium::SDL_GLattr::Type, value) }
  }

  /// Resets all previously set attributes to their default values.
  pub fn gl_reset_attributes(&self) {
    unsafe { SDL_GL_ResetAttributes() }
  }

  /// Gets a function pointer to the named OpenGL function
  pub unsafe fn gl_get_proc_address(&self, name: &str) -> *const c_void {
    let name_null: Vec<u8> = name.bytes().chain(Some(0)).collect();
    SDL_GL_GetProcAddress(name_null.as_ptr() as *const c_char) as *const c_void
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
  pub unsafe fn gl_get_drawable_size(&self) -> (i32, i32) {
    let mut w = 0;
    let mut h = 0;
    SDL_GL_GetDrawableSize(self.ptr, &mut w, &mut h);
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
impl From<Rect> for SDL_Rect {
  fn from(other: Rect) -> Self {
    Self {
      x: other.x,
      y: other.y,
      w: other.w,
      h: other.h,
    }
  }
}

/// Handle to a C ABI dynamic library that has been loaded.
///
/// You can make your own libs that will work with this using the `cdylib` crate
/// type
/// ([here](https://rust-embedded.github.io/book/interoperability/rust-with-c.html)
/// is a short tutorial).
///
/// Do not attempt to mix this with the `dylib` crate type. That's a crate type
/// you should not use, it's basically for `rustc` internal usage only.
#[derive(Debug)]
#[repr(transparent)]
pub struct CDyLib<'sdl> {
  ptr: *mut c_void,
  _marker: PhantomData<&'sdl SDLToken>,
}
impl<'sdl> Drop for CDyLib<'sdl> {
  fn drop(&mut self) {
    unsafe { SDL_UnloadObject(self.ptr) }
  }
}
impl<'sdl> CDyLib<'sdl> {
  /// Attempts to look up a function by name, getting its pointer.
  ///
  /// Once this function returns, you will have to
  /// [transmute](core::mem::transmute) the optional NonNull value you get into
  /// an optional `fn` value of some sort.
  ///
  /// You _probably_ want to transmute it into `Option<unsafe extern "C"
  /// fn(INPUTS) -> OUTPUTS>`, but it's possible that you might need to use some
  /// other ABI for example. This whole thing is obviously not at all safe. You
  /// absolutely must get the `fn` type correct when doing this `transmute`.
  ///
  /// # Safety
  ///
  /// * The returned value _does not_ have a lifetime linking it back to this
  ///   shared library. Making sure that the function pointer is not used after
  ///   this library unloads is up to you.
  pub unsafe fn find_function(&self, name: &str) -> Option<NonNull<c_void>> {
    let name_null: Vec<u8> = name.bytes().chain(Some(0)).collect();
    let see_void = SDL_LoadFunction(self.ptr, name_null.as_ptr() as *const c_char);
    core::mem::transmute::<*mut c_void, Option<NonNull<c_void>>>(see_void)
  }
}
