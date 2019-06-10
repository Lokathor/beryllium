#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unreachable_patterns)]
// fuck off clippy the types aren't the same on all systems
#![allow(clippy::cast_lossless)]
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

mod cdylib;
pub use cdylib::*;

mod window;
pub use window::*;

mod renderer;
pub use renderer::*;

mod texture;
pub use texture::*;

mod rect;
pub use rect::*;

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
  pub fn create_rgb_surface(
    &self, width: i32, height: i32, format: SurfaceFormat,
  ) -> Result<Surface<'_>, String> {
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
  pub fn open_controller(&self, index: u32) -> Result<Controller<'_>, String> {
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
    match NonNull::new(unsafe { SDL_LoadObject(name_null.as_ptr() as *const c_char) }) {
      Some(nn) => Ok(CDyLib {
        nn,
        _marker: PhantomData,
      }),
      None => Err(get_error()),
    }
  }

  /// Attempts to open a default audio device in "queue" mode.
  ///
  /// If successful, the device will initially be paused.
  pub fn open_default_audio_queue(
    &self, request: DefaultAudioQueueRequest,
  ) -> Result<AudioQueue<'_>, String> {
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

/// Basic struct for 2D positions.
///
/// Used with some parts of the [Renderer].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(C)]
pub struct Point {
  x: i32,
  y: i32,
}
impl From<SDL_Point> for Point {
  fn from(other: SDL_Point) -> Self {
    Self {
      x: other.x,
      y: other.y,
    }
  }
}
impl From<Point> for SDL_Point {
  fn from(other: Point) -> Self {
    Self {
      x: other.x,
      y: other.y,
    }
  }
}
