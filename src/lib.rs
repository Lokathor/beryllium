#![no_std]
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
//! ## Initialization And Usage
//!
//! Very little of SDL2 can be called before you initialize the library. This
//! must be done from the main thread and it must be done only once. `beryllium`
//! will safely block any double initialization, but I can't enforce that you're
//! on the main thread.
//!
//! ```no_run
//! let sdl = beryllium::init().expect("Couldn't initialize SDL2!");
//! ```
//!
//! Initialization gives you an [SDLToken](SDLToken) (no, I will not change the
//! name to `SdlToken`, that's stupid). This token is the "proof" that you've
//! got SDL2 initialized. Basically every other part of the library is a method
//! off of this token value (polling for events, opening a window, etc), or it's
//! a method of a struct that you create off of this token value (the window you
//! opened, an audio queue, etc).
//!
//! There's a limited number of other functions that are fine to call even if
//! SDL2 is _not_ initialized, and so they're stand alone top level functions:
//!
//! * [version](version)
//! * [get_error](get_error)
//! * [lone_message_box](lone_message_box)
//!
//! _Very little_ of SDL2 is thread-safe. Your code that interacts with SDL2
//! will mostly be locked into just the `main` thread. This isn't a huge deal in
//! practice, but it's something that people might want to know up font.
//!
//! ## Safety and Lifetimes
//!
//! I'm aiming to make as much of the API safe as is possible, but unfortunately
//! not 100% of the interface can be safe. Things are only as safe as they can
//! get, I don't want to ever over promise.
//!
//! Much of the static safety is done with lifetime tracking, which can make it
//! hard to merge different `beryllium` values into a single struct.
//! Particularly, values can't easily be combined with their "parent" value. I
//! honestly don't try to do this kind of thing in my own programs. I just have
//! a few free floating values on the stack and then I try to interact with SDL2
//! as absolutely little as possible. It's an OS abstraction layer, it's not
//! your friend.
//!
//! If you _really_ feel like you want to try and stick things together into a
//! single struct and then pass that combination around all through the program
//! and stuff, feel free to use [transmute](core::mem::transmute) to fake the
//! lifetimes involved and [ManuallyDrop](core::mem::ManuallyDrop) to carefully
//! clean it up at the end. Or you could go use the [sdl2](https://docs.rs/sdl2)
//! crate, which handles everything by tracking which subsystems are active via
//! an [Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html) value in
//! practically everything.
//!
//! Please note that safety assumptions are usually based on actually looking at
//! the current version's C code, so if an end user uses the [Dynamic
//! API](https://github.com/SDL-mirror/SDL/blob/master/docs/README-dynapi.md) to
//! override the version of SDL2 used it's _possible_ they could break safety.
//! It's honestly their fault at that point, not yours or mine. I'm not trying
//! to be glib about it, but calling _arbitrary_ code can't be safety checked.
//! That said, it's an important end user ability that should not be removed.
//!
//! ## Rewritten In Rust
//!
//! Select portions of the SDL2 API _have_ been re-written entirely in Rust
//! (insert memes here). Instead of making a call to whichever SDL2 I simply
//! ported the appropriate code to Rust.
//!
//! * **The Good:**
//!   * It's easier (for us Rust programmers) to understand Rust than C.
//!   * LLVM can inline Rust code (when appropriate and such).
//! * **The Bad:**
//!   * This takes more dev time to make sure that the new Rust matches the
//!     semantics of the C it's replacing.
//! * **The Ugly:**
//!   * If there's a bug user's can use the Dynamic API to apply a patch, so we
//!     don't want to do this for anything hardware or OS related (those are the
//!     things most like to need driver fixes).
//!
//! So it's not a lot, and it probably won't ever be a lot, but some _small_
//! parts have been rewritten in Rust.

pub use fermium;

extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

use core::{
  convert::TryFrom,
  ffi::c_void,
  marker::PhantomData,
  ops::Deref,
  ptr::{null, null_mut, NonNull},
  slice::from_raw_parts,
  sync::atomic::{AtomicBool, Ordering},
};

use fermium::c_char;
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

/// In case of emergency, you can break the glass.
pub use fermium as unsafe_raw_ffi;

/// Grabs up the data from a null terminated string pointer.
unsafe fn gather_string(ptr: *const c_char) -> String {
  let len = fermium::SDL_strlen(ptr);
  let useful_bytes = from_raw_parts(ptr as *const u8, len);
  String::from_utf8_lossy(useful_bytes).into_owned()
}

/// Converts a bool into a SDL_bool.
fn into_sdl_bool(flag: bool) -> fermium::SDL_bool {
  if flag {
    fermium::SDL_TRUE
  } else {
    fermium::SDL_FALSE
  }
}

/// A version number.
#[derive(Debug, Default, Clone, Copy)]
#[allow(missing_docs)]
pub struct Version {
  pub major: u8,
  pub minor: u8,
  pub patch: u8,
}
impl From<fermium::SDL_version> for Version {
  fn from(input: fermium::SDL_version) -> Self {
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
pub fn version() -> Version {
  let mut sdl_version = fermium::SDL_version::default();
  unsafe { fermium::SDL_GetVersion(&mut sdl_version) };
  Version::from(sdl_version)
}

/// Obtains the current SDL2 error string.
///
/// You should never need to call this yourself, but I guess you can if you
/// really want.
pub fn get_error() -> String {
  unsafe { gather_string(fermium::SDL_GetError()) }
}

/// The kind of message box you wish to show.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum MessageBox {
  Error = fermium::SDL_MESSAGEBOX_ERROR,
  Warning = fermium::SDL_MESSAGEBOX_WARNING,
  Information = fermium::SDL_MESSAGEBOX_INFORMATION,
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
  box_type: MessageBox,
  title: &str,
  message: &str,
) -> Result<(), String> {
  let title_null: Vec<u8> = title.bytes().chain(Some(0)).collect();
  let message_null: Vec<u8> = message.bytes().chain(Some(0)).collect();
  let output = fermium::SDL_ShowSimpleMessageBox(
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
/// # Failure
///
/// * You cannot double initialize SDL2, you'll get an error.
/// * If SDL2 fails to initialize you'll get an error and it will revert itself
///   to an uninitialized state, perhaps allowing you to try again.
/// * If you call this function on macOS or iOS and are not on the main thread,
///   you get an error.
pub fn init() -> Result<SDLToken, String> {
  #[cfg(any(target_os = "macos", target_os = "ios"))]
  {
    use objc::{class, msg_send, sel, sel_impl};
    let is_main: bool = unsafe { msg_send![class!(NSThread), isMainThread] };
    if !is_main {
      return Err("beryllium::init() must be called on the main thread!".to_string());
    }
  }
  if I_THINK_THAT_SDL2_IS_ACTIVE.swap(true, Ordering::SeqCst) {
    // Note(Lokathor): `swap` gives the old value back, so if we get back `true`
    // that means it was already active, so that's an error.
    Err("The library is currently initialized!".to_string())
  } else if unsafe { fermium::SDL_Init(fermium::SDL_INIT_EVERYTHING) } == 0 {
    Ok(SDLToken {
      _marker: PhantomData,
    })
  } else {
    let out = get_error();
    I_THINK_THAT_SDL2_IS_ACTIVE.store(false, Ordering::SeqCst);
    Err(out)
  }
}
static I_THINK_THAT_SDL2_IS_ACTIVE: AtomicBool = AtomicBool::new(false);

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
    unsafe { fermium::SDL_Quit() }
    I_THINK_THAT_SDL2_IS_ACTIVE.store(false, Ordering::SeqCst);
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
    &'sdl self,
    title: &str,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    flags: WindowFlags,
  ) -> Result<Window<'sdl>, String> {
    let title_null: Vec<u8> = title.bytes().chain(Some(0)).collect();
    let ptr = unsafe {
      fermium::SDL_CreateWindow(
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
      Ok(crate::window::Window {
        ptr,
        _marker: PhantomData,
      })
    }
  }

  /// Creates a new [Surface](Surface) with the desired format, or error.
  ///
  /// See [the wiki page](https://wiki.libsdl.org/SDL_CreateRGBSurface)
  pub fn create_rgb_surface(
    &self,
    width: i32,
    height: i32,
    format: SurfaceFormat,
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
    let ptr: *mut fermium::SDL_Surface = unsafe {
      fermium::SDL_CreateRGBSurface(0, width, height, depth, r_mask, g_mask, b_mask, a_mask)
    };
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
      let mut sdl_event = fermium::SDL_Event::default();
      if fermium::SDL_PollEvent(&mut sdl_event) == 1 {
        Some(Event::from(sdl_event))
      } else {
        None
      }
    }
  }

  /// Obtains the number of joysticks connected to the system.
  pub fn number_of_joysticks(&self) -> Result<i32, String> {
    let out = unsafe { fermium::SDL_NumJoysticks() };
    if out < 0 {
      Err(get_error())
    } else {
      Ok(out)
    }
  }

  /// Says if the joystick index supports the Controller API.
  pub fn joystick_is_game_controller(&self, id: JoystickID) -> bool {
    fermium::SDL_TRUE == unsafe { fermium::SDL_IsGameController(id.0) }
  }

  /// Given a joystick index, attempts to get the Controller name, if any.
  pub fn controller_name(&self, id: JoystickID) -> Option<String> {
    let ptr = unsafe { fermium::SDL_GameControllerNameForIndex(id.0) };
    if ptr.is_null() {
      None
    } else {
      unsafe { Some(gather_string(ptr)) }
    }
  }

  /// Attempts to open the given id as a [Controller].
  ///
  /// Not all joysticks support the Controller API, so this can fail.
  pub fn open_controller(&self, id: JoystickID) -> Result<Controller<'_>, String> {
    let ptr = unsafe { fermium::SDL_GameControllerOpen(id.0) };
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
    match NonNull::new(unsafe { fermium::SDL_LoadObject(name_null.as_ptr() as *const c_char) }) {
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
    &self,
    request: DefaultAudioQueueRequest,
  ) -> Result<AudioQueue<'_>, String> {
    //
    let mut desired_spec = fermium::SDL_AudioSpec::default();
    desired_spec.freq = request.frequency;
    desired_spec.format = request.format.0;
    desired_spec.channels = request.channels;
    desired_spec.samples = request.samples;
    //
    let mut changes = 0;
    if request.allow_frequency_change {
      changes |= fermium::SDL_AUDIO_ALLOW_FREQUENCY_CHANGE as i32;
    }
    if request.allow_format_change {
      changes |= fermium::SDL_AUDIO_ALLOW_FORMAT_CHANGE as i32;
    }
    if request.allow_channels_change {
      changes |= fermium::SDL_AUDIO_ALLOW_CHANNELS_CHANGE as i32;
    }
    //
    let mut obtained_spec = fermium::SDL_AudioSpec::default();
    //
    let audio_device_id = unsafe {
      fermium::SDL_OpenAudioDevice(null(), 0, &desired_spec, &mut obtained_spec, changes)
    };
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
    0 == unsafe { fermium::SDL_GL_SetAttribute(attr as fermium::SDL_GLattr, value) }
  }

  /// Resets all previously set attributes to their default values.
  pub fn gl_reset_attributes(&self) {
    unsafe { fermium::SDL_GL_ResetAttributes() }
  }

  /// Gets a function pointer to the named OpenGL function
  pub unsafe fn gl_get_proc_address(&self, name: &str) -> *const c_void {
    let name_null: Vec<u8> = name.bytes().chain(Some(0)).collect();
    fermium::SDL_GL_GetProcAddress(name_null.as_ptr() as *const c_char) as *const c_void
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
impl From<fermium::SDL_Point> for Point {
  fn from(other: fermium::SDL_Point) -> Self {
    Self {
      x: other.x,
      y: other.y,
    }
  }
}
impl From<Point> for fermium::SDL_Point {
  fn from(other: Point) -> Self {
    Self {
      x: other.x,
      y: other.y,
    }
  }
}
