#![warn(missing_docs)]
#![deny(missing_debug_implementations)]

//! An opinionated set of "high level" wrappers for the
//! [fermium](https://github.com/Lokathor/fermium) SDL2 bindings.

use core::{convert::TryFrom, marker::PhantomData, ptr::null_mut, slice::from_raw_parts};
use fermium::{SDL_EventType::*, SDL_WindowFlags::*, *};
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
  /// Note that not all possible flags have an effect! See
  /// [SDL_CreateWindow](https://wiki.libsdl.org/SDL_CreateWindow) for guidance.
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

  /// Polls for an event, getting it out of the queue if one is there.
  pub fn poll_event(&self) -> Option<Event> {
    unsafe {
      let mut event = SDL_Event::default();
      if SDL_PollEvent(&mut event) == 1 {
        Some(Event::try_from(event).unwrap_or(Event::UnknownEventType))
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
impl TryFrom<SDL_Event> for Event {
  // TODO: a real error type here?
  type Error = ();

  /// Fails if the input has an unknown `type_` value.
  fn try_from(event: SDL_Event) -> Result<Self, Self::Error> {
    unsafe {
      match event.type_ as SDL_EventType::Type {
        SDL_QUIT => Ok(Event::Quit {
          timestamp: event.quit.timestamp,
        }),
        SDL_MOUSEMOTION => Ok(Event::MouseMotion {
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
        }),
        SDL_MOUSEBUTTONDOWN | SDL_MOUSEBUTTONUP => Ok(Event::MouseButtonEvent {
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
        }),
        SDL_MOUSEWHEEL => Ok(Event::MouseWheel {
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
        }),
        _ => Err(()),
      }
    }
  }
}
