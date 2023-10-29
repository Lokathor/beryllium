use alloc::{string::String, vec::Vec};
use bytemuck::cast_slice;
use fermium::prelude::*;

use crate::{
  controller::{ControllerAxis, ControllerButton},
  Sdl,
};

impl Sdl {
  #[inline]
  pub fn poll_events(&self) -> Option<(Event, u32)> {
    let mut sdl_event: SDL_Event = SDL_Event::default();
    if unsafe { SDL_PollEvent(&mut sdl_event) } != 0 {
      Event::try_from(sdl_event).ok().map(|e| (e, unsafe { sdl_event.common.timestamp }))
    } else {
      None
    }
  }

  /// Get the number of milliseconds since the SDL library initialization.
  #[inline]
  #[must_use]
  pub fn get_ticks(&self) -> u32 {
    unsafe { SDL_GetTicks() }
  }
}

pub use fermium::prelude::{SDL_Keycode, SDL_Keymod, SDL_Scancode};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[rustfmt::skip]
pub enum Event {
  Quit,
  DisplayConnected { display_index: u32 },
  DisplayDisconnected { display_index: u32 },
  DisplayOrientationChanged { display_index: u32, new_orientation: DisplayOrientation },
  WindowShown { win_id: u32 },
  WindowHidden { win_id: u32 },
  WindowExposed { win_id: u32 },
  WindowMoved { win_id: u32, x: i32, y: i32 },
  WindowResized { win_id: u32, width: i32, height: i32 },
  WindowSizeChanged { win_id: u32, width: i32, height: i32 },
  WindowMinimized { win_id: u32 },
  WindowMaximized { win_id: u32 },
  WindowRestored { win_id: u32 },
  MouseEnteredWindow { win_id: u32 },
  MouseExitedWindow { win_id: u32 },
  WindowGainedKeyboardFocus { win_id: u32 },
  WindowLostKeyboardFocus { win_id: u32 },
  WindowCloseRequest { win_id: u32 },
  Key { win_id: u32, pressed: bool, repeat: u8, scancode: SDL_Scancode, keycode: SDL_Keycode, modifiers: SDL_Keymod },

  // * TODO: SDL_TextEditingEvent
  
  TextInput { win_id: u32, text: String },

  /// Mouse cursor motion
  /// * `x_win` and `y_win` are the window-relative mouse position.
  /// * `x_delta` and `y_delta` are the change in position since the last event.
  /// * `button_state` has bit `N` set when mouse button `N` is held down during the event.
  MouseMotion { win_id: u32, mouse_id: u32, button_state: u32, x_win: i32, y_win: i32, x_delta: i32, y_delta: i32 },

  MouseButton { win_id: u32, mouse_id: u32, button: u8, pressed: bool, clicks: u8, x: i32, y: i32 },

  /// Mouse wheel change
  /// * `x`: horizontal, with positive to the right.
  /// * `y`: vertical, with positive *away* from the user.
  MouseWheel { win_id: u32, mouse_id: u32, x: i32, y: i32 },
  JoystickAxis { joy_id: i32, axis: u8, value: i16 },
  JoystickBall { joy_id: i32, ball: u8, x_rel: i16, y_rel: i16 },
  JoystickHat { joy_id: i32, hat: u8, value: u8 },
  JoystickButton { joy_id: i32, button: u8, pressed: bool },
  JoystickAdded { index: i32 },
  JoystickRemoved { joy_id: i32 },
  ControllerAxis { ctrl_id: i32, axis: ControllerAxis, value: i16 },
  ControllerButton { ctrl_id: i32, button: ControllerButton, pressed: bool },
  ControllerAdded { index: i32 },
  ControllerRemoved { ctrl_id: i32 },
  ControllerRemapped { ctrl_id: i32 },

  //ControllerTouchpad { ?? },

  ControllerSensor { ctrl_id: i32, sensor: i32, data: [f32; 3] },
  AudioDeviceAdded { index: u32, is_capture: bool },
  AudioDeviceRemoved { audio_id: u32, is_capture: bool },
  Sensor { sensor_id: i32, data: [f32; 6] },

  // * TODO: SDL_UserEvent
  // * TODO: SDL_SysWMEvent
  // * TODO: SDL_TouchFingerEvent
  // * TODO: SDL_MultiGestureEvent
  // * TODO: SDL_DollarGestureEvent
  
  /// Marks the start of a series of files being dropped onto the window.
  DropBegin { win_id: u32 },

  /// The name of a file or directory the user dropped into the window.
  DropFile { win_id: u32, name: String },

  /// This marks the end of a group of file drops.
  DropComplete { win_id: u32 },
}

impl TryFrom<SDL_Event> for Event {
  type Error = ();
  #[inline]
  fn try_from(sdl_event: SDL_Event) -> Result<Self, Self::Error> {
    Ok(match unsafe { sdl_event.common.type_ } {
      SDL_QUIT => Event::Quit,
      SDL_DISPLAYEVENT => {
        let v = unsafe { sdl_event.display };
        match v.event {
          SDL_DISPLAYEVENT_CONNECTED => Event::DisplayConnected { display_index: v.display },
          SDL_DISPLAYEVENT_DISCONNECTED => Event::DisplayDisconnected { display_index: v.display },
          SDL_DISPLAYEVENT_ORIENTATION => {
            let new_orientation = match SDL_DisplayOrientation(v.data1 as u32) {
              SDL_ORIENTATION_PORTRAIT => DisplayOrientation::Portrait,
              SDL_ORIENTATION_LANDSCAPE => DisplayOrientation::Landscape,
              SDL_ORIENTATION_PORTRAIT_FLIPPED => DisplayOrientation::PortraitFlipped,
              SDL_ORIENTATION_LANDSCAPE_FLIPPED => DisplayOrientation::LandscapeFlipped,
              _ => DisplayOrientation::Unknown,
            };
            Event::DisplayOrientationChanged { display_index: v.display, new_orientation }
          }
          _ => return Err(()),
        }
      }
      SDL_WINDOWEVENT => {
        let v = unsafe { sdl_event.window };
        match v.event {
          SDL_WINDOWEVENT_SHOWN => Event::WindowShown { win_id: v.windowID },
          SDL_WINDOWEVENT_HIDDEN => Event::WindowHidden { win_id: v.windowID },
          SDL_WINDOWEVENT_EXPOSED => Event::WindowExposed { win_id: v.windowID },
          SDL_WINDOWEVENT_MOVED => {
            Event::WindowMoved { win_id: v.windowID, x: v.data1, y: v.data2 }
          }
          SDL_WINDOWEVENT_RESIZED => {
            Event::WindowResized { win_id: v.windowID, width: v.data1, height: v.data2 }
          }
          SDL_WINDOWEVENT_SIZE_CHANGED => {
            Event::WindowSizeChanged { win_id: v.windowID, width: v.data1, height: v.data2 }
          }
          SDL_WINDOWEVENT_MINIMIZED => Event::WindowMinimized { win_id: v.windowID },
          SDL_WINDOWEVENT_MAXIMIZED => Event::WindowMaximized { win_id: v.windowID },
          SDL_WINDOWEVENT_RESTORED => Event::WindowRestored { win_id: v.windowID },
          SDL_WINDOWEVENT_ENTER => Event::MouseEnteredWindow { win_id: v.windowID },
          SDL_WINDOWEVENT_LEAVE => Event::MouseExitedWindow { win_id: v.windowID },
          SDL_WINDOWEVENT_FOCUS_GAINED => Event::WindowGainedKeyboardFocus { win_id: v.windowID },
          SDL_WINDOWEVENT_FOCUS_LOST => Event::WindowLostKeyboardFocus { win_id: v.windowID },
          SDL_WINDOWEVENT_CLOSE => Event::WindowCloseRequest { win_id: v.windowID },
          _ => return Err(()),
        }
      }
      SDL_KEYDOWN | SDL_KEYUP => {
        let v = unsafe { sdl_event.key };
        Event::Key {
          win_id: v.windowID,
          pressed: v.state == SDL_PRESSED,
          repeat: v.repeat,
          scancode: v.keysym.scancode,
          keycode: v.keysym.sym,
          modifiers: SDL_Keymod(i32::from(v.keysym.mod_)),
        }
      }
      SDL_TEXTINPUT => {
        let v = unsafe { sdl_event.text };
        let text_slice: &[u8] = cast_slice(v.text.as_slice());
        let text_len = text_slice.iter().position(|b| *b == 0).unwrap_or(text_slice.len());
        let text = String::from_utf8_lossy(&text_slice[..text_len]).into_owned();
        Event::TextInput { win_id: v.windowID, text }
      }
      SDL_MOUSEMOTION => {
        let v = unsafe { sdl_event.motion };
        Event::MouseMotion {
          win_id: v.windowID,
          mouse_id: v.which,
          button_state: v.state,
          x_win: v.x,
          y_win: v.y,
          x_delta: v.xrel,
          y_delta: v.yrel,
        }
      }
      SDL_MOUSEBUTTONDOWN | SDL_MOUSEBUTTONUP => {
        let v = unsafe { sdl_event.button };
        Event::MouseButton {
          win_id: v.windowID,
          mouse_id: v.which,
          button: v.button,
          pressed: v.state == SDL_PRESSED,
          clicks: v.clicks,
          x: v.x,
          y: v.y,
        }
      }
      SDL_MOUSEWHEEL => {
        let v = unsafe { sdl_event.wheel };
        let x = if v.direction == SDL_MOUSEWHEEL_FLIPPED { -v.x } else { v.x };
        let y = if v.direction == SDL_MOUSEWHEEL_FLIPPED { -v.y } else { v.y };
        Event::MouseWheel { win_id: v.windowID, mouse_id: v.which, x, y }
      }
      SDL_JOYAXISMOTION => {
        let v = unsafe { sdl_event.jaxis };
        Event::JoystickAxis { joy_id: v.which.0, axis: v.axis, value: v.value }
      }
      SDL_JOYBALLMOTION => {
        let v = unsafe { sdl_event.jball };
        Event::JoystickBall { joy_id: v.which.0, ball: v.ball, x_rel: v.xrel, y_rel: v.yrel }
      }
      SDL_JOYHATMOTION => {
        let v = unsafe { sdl_event.jhat };
        Event::JoystickHat { joy_id: v.which.0, hat: v.hat, value: v.value }
      }
      SDL_JOYBUTTONDOWN | SDL_JOYBUTTONUP => {
        let v = unsafe { sdl_event.jbutton };
        Event::JoystickButton {
          joy_id: v.which.0,
          button: v.button,
          pressed: v.state == SDL_PRESSED,
        }
      }
      SDL_JOYDEVICEADDED => {
        let v = unsafe { sdl_event.jdevice };
        Event::JoystickAdded { index: v.which }
      }
      SDL_JOYDEVICEREMOVED => {
        let v = unsafe { sdl_event.jdevice };
        Event::JoystickRemoved { joy_id: v.which }
      }
      SDL_CONTROLLERAXISMOTION => {
        let v = unsafe { sdl_event.caxis };
        Event::ControllerAxis {
          ctrl_id: v.which.0,
          axis: ControllerAxis::from(v.axis),
          value: v.value,
        }
      }
      SDL_CONTROLLERBUTTONDOWN | SDL_CONTROLLERBUTTONUP => {
        let v = unsafe { sdl_event.cbutton };
        Event::ControllerButton {
          ctrl_id: v.which.0,
          button: ControllerButton::from(v.button),
          pressed: v.state == SDL_PRESSED,
        }
      }
      SDL_CONTROLLERDEVICEADDED => {
        let v = unsafe { sdl_event.cdevice };
        Event::ControllerAdded { index: v.which }
      }
      SDL_CONTROLLERDEVICEREMOVED => {
        let v = unsafe { sdl_event.cdevice };
        Event::ControllerRemoved { ctrl_id: v.which }
      }
      SDL_CONTROLLERDEVICEREMAPPED => {
        let v = unsafe { sdl_event.cdevice };
        Event::ControllerRemapped { ctrl_id: v.which }
      }
      // SDL_ControllerTouchpadEvent
      SDL_CONTROLLERSENSORUPDATE => {
        let v = unsafe { sdl_event.csensor };
        Event::ControllerSensor { ctrl_id: v.which.0, sensor: v.sensor, data: v.data }
      }
      SDL_AUDIODEVICEADDED => {
        let v = unsafe { sdl_event.adevice };
        Event::AudioDeviceAdded { index: v.which, is_capture: v.iscapture != 0 }
      }
      SDL_AUDIODEVICEREMOVED => {
        let v = unsafe { sdl_event.adevice };
        Event::AudioDeviceRemoved { audio_id: v.which, is_capture: v.iscapture != 0 }
      }
      SDL_SENSORUPDATE => {
        let v = unsafe { sdl_event.sensor };
        Event::Sensor { sensor_id: v.which, data: v.data }
      }
      SDL_DROPBEGIN => Event::DropBegin { win_id: unsafe { sdl_event.drop.windowID } },
      SDL_DROPFILE => {
        let v = unsafe { sdl_event.drop };
        if v.file.is_null() {
          return Err(());
        }
        let mut raw_bytes = Vec::new();
        let mut file = v.file;
        while unsafe { *file } != 0 {
          raw_bytes.push(unsafe { *file } as u8);
          file = unsafe { file.add(1) };
        }
        // SDL2 *should* always give us the utf8 version of the filename
        // already, so we shouldn't end up hitting the lossy path.
        let name = match String::from_utf8(raw_bytes) {
          Ok(string) => string,
          Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
        };
        unsafe { SDL_free(v.file as _) };
        Event::DropFile { win_id: v.windowID, name }
      }
      SDL_DROPTEXT => {
        let v = unsafe { sdl_event.drop };
        // Even if we don't gather up the text yet, we need to free the pointer
        // or it'll just leak memory.
        unsafe { SDL_free(v.file as _) };
        return Err(());
      }
      SDL_DROPCOMPLETE => Event::DropComplete { win_id: unsafe { sdl_event.drop.windowID } },
      _ => return Err(()),
    })
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DisplayOrientation {
  Unknown,
  Portrait,
  Landscape,
  PortraitFlipped,
  LandscapeFlipped,
}

// re-export all keycodes
pub use fermium::prelude::{
  SDLK_a, SDLK_b, SDLK_c, SDLK_d, SDLK_e, SDLK_f, SDLK_g, SDLK_h, SDLK_i, SDLK_j, SDLK_k, SDLK_l,
  SDLK_m, SDLK_n, SDLK_o, SDLK_p, SDLK_q, SDLK_r, SDLK_s, SDLK_t, SDLK_u, SDLK_v, SDLK_w, SDLK_x,
  SDLK_y, SDLK_z, SDLK_0, SDLK_1, SDLK_2, SDLK_3, SDLK_4, SDLK_5, SDLK_6, SDLK_7, SDLK_8, SDLK_9,
  SDLK_AC_BACK, SDLK_AC_BOOKMARKS, SDLK_AC_FORWARD, SDLK_AC_HOME, SDLK_AC_REFRESH, SDLK_AC_SEARCH,
  SDLK_AC_STOP, SDLK_AGAIN, SDLK_ALTERASE, SDLK_AMPERSAND, SDLK_APP1, SDLK_APP2, SDLK_APPLICATION,
  SDLK_ASTERISK, SDLK_AT, SDLK_AUDIOFASTFORWARD, SDLK_AUDIOMUTE, SDLK_AUDIONEXT, SDLK_AUDIOPLAY,
  SDLK_AUDIOPREV, SDLK_AUDIOREWIND, SDLK_AUDIOSTOP, SDLK_BACKQUOTE, SDLK_BACKSLASH, SDLK_BACKSPACE,
  SDLK_BRIGHTNESSDOWN, SDLK_BRIGHTNESSUP, SDLK_CALCULATOR, SDLK_CANCEL, SDLK_CAPSLOCK, SDLK_CARET,
  SDLK_CLEAR, SDLK_CLEARAGAIN, SDLK_COLON, SDLK_COMMA, SDLK_COMPUTER, SDLK_COPY, SDLK_CRSEL,
  SDLK_CURRENCYSUBUNIT, SDLK_CURRENCYUNIT, SDLK_CUT, SDLK_DECIMALSEPARATOR, SDLK_DELETE,
  SDLK_DISPLAYSWITCH, SDLK_DOLLAR, SDLK_DOWN, SDLK_EJECT, SDLK_END, SDLK_EQUALS, SDLK_ESCAPE,
  SDLK_EXCLAIM, SDLK_EXECUTE, SDLK_EXSEL, SDLK_F1, SDLK_F10, SDLK_F11, SDLK_F12, SDLK_F13,
  SDLK_F14, SDLK_F15, SDLK_F16, SDLK_F17, SDLK_F18, SDLK_F19, SDLK_F2, SDLK_F20, SDLK_F21,
  SDLK_F22, SDLK_F23, SDLK_F24, SDLK_F3, SDLK_F4, SDLK_F5, SDLK_F6, SDLK_F7, SDLK_F8, SDLK_F9,
  SDLK_FIND, SDLK_GREATER, SDLK_HASH, SDLK_HELP, SDLK_HOME, SDLK_INSERT, SDLK_KBDILLUMDOWN,
  SDLK_KBDILLUMTOGGLE, SDLK_KBDILLUMUP, SDLK_KP_0, SDLK_KP_00, SDLK_KP_000, SDLK_KP_1, SDLK_KP_2,
  SDLK_KP_3, SDLK_KP_4, SDLK_KP_5, SDLK_KP_6, SDLK_KP_7, SDLK_KP_8, SDLK_KP_9, SDLK_KP_A,
  SDLK_KP_AMPERSAND, SDLK_KP_AT, SDLK_KP_B, SDLK_KP_BACKSPACE, SDLK_KP_BINARY, SDLK_KP_C,
  SDLK_KP_CLEAR, SDLK_KP_CLEARENTRY, SDLK_KP_COLON, SDLK_KP_COMMA, SDLK_KP_D, SDLK_KP_DBLAMPERSAND,
  SDLK_KP_DBLVERTICALBAR, SDLK_KP_DECIMAL, SDLK_KP_DIVIDE, SDLK_KP_E, SDLK_KP_ENTER,
  SDLK_KP_EQUALS, SDLK_KP_EQUALSAS400, SDLK_KP_EXCLAM, SDLK_KP_F, SDLK_KP_GREATER, SDLK_KP_HASH,
  SDLK_KP_HEXADECIMAL, SDLK_KP_LEFTBRACE, SDLK_KP_LEFTPAREN, SDLK_KP_LESS, SDLK_KP_MEMADD,
  SDLK_KP_MEMCLEAR, SDLK_KP_MEMDIVIDE, SDLK_KP_MEMMULTIPLY, SDLK_KP_MEMRECALL, SDLK_KP_MEMSTORE,
  SDLK_KP_MEMSUBTRACT, SDLK_KP_MINUS, SDLK_KP_MULTIPLY, SDLK_KP_OCTAL, SDLK_KP_PERCENT,
  SDLK_KP_PERIOD, SDLK_KP_PLUS, SDLK_KP_PLUSMINUS, SDLK_KP_POWER, SDLK_KP_RIGHTBRACE,
  SDLK_KP_RIGHTPAREN, SDLK_KP_SPACE, SDLK_KP_TAB, SDLK_KP_VERTICALBAR, SDLK_KP_XOR, SDLK_LALT,
  SDLK_LCTRL, SDLK_LEFT, SDLK_LEFTBRACKET, SDLK_LEFTPAREN, SDLK_LESS, SDLK_LGUI, SDLK_LSHIFT,
  SDLK_MAIL, SDLK_MEDIASELECT, SDLK_MENU, SDLK_MINUS, SDLK_MODE, SDLK_MUTE, SDLK_NUMLOCKCLEAR,
  SDLK_OPER, SDLK_OUT, SDLK_PAGEDOWN, SDLK_PAGEUP, SDLK_PASTE, SDLK_PAUSE, SDLK_PERCENT,
  SDLK_PERIOD, SDLK_PLUS, SDLK_POWER, SDLK_PRINTSCREEN, SDLK_PRIOR, SDLK_QUESTION, SDLK_QUOTE,
  SDLK_QUOTEDBL, SDLK_RALT, SDLK_RCTRL, SDLK_RETURN, SDLK_RETURN2, SDLK_RGUI, SDLK_RIGHT,
  SDLK_RIGHTBRACKET, SDLK_RIGHTPAREN, SDLK_RSHIFT, SDLK_SCROLLLOCK, SDLK_SELECT, SDLK_SEMICOLON,
  SDLK_SEPARATOR, SDLK_SLASH, SDLK_SLEEP, SDLK_SPACE, SDLK_STOP, SDLK_SYSREQ, SDLK_TAB,
  SDLK_THOUSANDSSEPARATOR, SDLK_UNDERSCORE, SDLK_UNDO, SDLK_UNKNOWN, SDLK_UP, SDLK_VOLUMEDOWN,
  SDLK_VOLUMEUP, SDLK_WWW,
};

// re-export all key modifiers
pub use fermium::prelude::{
  KMOD_ALT, KMOD_CAPS, KMOD_CTRL, KMOD_GUI, KMOD_LALT, KMOD_LCTRL, KMOD_LGUI, KMOD_LSHIFT,
  KMOD_MODE, KMOD_NONE, KMOD_NUM, KMOD_RALT, KMOD_RCTRL, KMOD_RESERVED, KMOD_RGUI, KMOD_RSHIFT,
  KMOD_SHIFT,
};

// re-export all scancodes
pub use fermium::prelude::{
  SDL_SCANCODE_0, SDL_SCANCODE_1, SDL_SCANCODE_2, SDL_SCANCODE_3, SDL_SCANCODE_4, SDL_SCANCODE_5,
  SDL_SCANCODE_6, SDL_SCANCODE_7, SDL_SCANCODE_8, SDL_SCANCODE_9, SDL_SCANCODE_A,
  SDL_SCANCODE_AC_BACK, SDL_SCANCODE_AC_BOOKMARKS, SDL_SCANCODE_AC_FORWARD, SDL_SCANCODE_AC_HOME,
  SDL_SCANCODE_AC_REFRESH, SDL_SCANCODE_AC_SEARCH, SDL_SCANCODE_AC_STOP, SDL_SCANCODE_AGAIN,
  SDL_SCANCODE_ALTERASE, SDL_SCANCODE_APOSTROPHE, SDL_SCANCODE_APP1, SDL_SCANCODE_APP2,
  SDL_SCANCODE_APPLICATION, SDL_SCANCODE_AUDIOFASTFORWARD, SDL_SCANCODE_AUDIOMUTE,
  SDL_SCANCODE_AUDIONEXT, SDL_SCANCODE_AUDIOPLAY, SDL_SCANCODE_AUDIOPREV, SDL_SCANCODE_AUDIOREWIND,
  SDL_SCANCODE_AUDIOSTOP, SDL_SCANCODE_B, SDL_SCANCODE_BACKSLASH, SDL_SCANCODE_BACKSPACE,
  SDL_SCANCODE_BRIGHTNESSDOWN, SDL_SCANCODE_BRIGHTNESSUP, SDL_SCANCODE_C, SDL_SCANCODE_CALCULATOR,
  SDL_SCANCODE_CANCEL, SDL_SCANCODE_CAPSLOCK, SDL_SCANCODE_CLEAR, SDL_SCANCODE_CLEARAGAIN,
  SDL_SCANCODE_COMMA, SDL_SCANCODE_COMPUTER, SDL_SCANCODE_COPY, SDL_SCANCODE_CRSEL,
  SDL_SCANCODE_CURRENCYSUBUNIT, SDL_SCANCODE_CURRENCYUNIT, SDL_SCANCODE_CUT, SDL_SCANCODE_D,
  SDL_SCANCODE_DECIMALSEPARATOR, SDL_SCANCODE_DELETE, SDL_SCANCODE_DISPLAYSWITCH,
  SDL_SCANCODE_DOWN, SDL_SCANCODE_E, SDL_SCANCODE_EJECT, SDL_SCANCODE_END, SDL_SCANCODE_EQUALS,
  SDL_SCANCODE_ESCAPE, SDL_SCANCODE_EXECUTE, SDL_SCANCODE_EXSEL, SDL_SCANCODE_F, SDL_SCANCODE_F1,
  SDL_SCANCODE_F10, SDL_SCANCODE_F11, SDL_SCANCODE_F12, SDL_SCANCODE_F13, SDL_SCANCODE_F14,
  SDL_SCANCODE_F15, SDL_SCANCODE_F16, SDL_SCANCODE_F17, SDL_SCANCODE_F18, SDL_SCANCODE_F19,
  SDL_SCANCODE_F2, SDL_SCANCODE_F20, SDL_SCANCODE_F21, SDL_SCANCODE_F22, SDL_SCANCODE_F23,
  SDL_SCANCODE_F24, SDL_SCANCODE_F3, SDL_SCANCODE_F4, SDL_SCANCODE_F5, SDL_SCANCODE_F6,
  SDL_SCANCODE_F7, SDL_SCANCODE_F8, SDL_SCANCODE_F9, SDL_SCANCODE_FIND, SDL_SCANCODE_G,
  SDL_SCANCODE_GRAVE, SDL_SCANCODE_H, SDL_SCANCODE_HELP, SDL_SCANCODE_HOME, SDL_SCANCODE_I,
  SDL_SCANCODE_INSERT, SDL_SCANCODE_INTERNATIONAL1, SDL_SCANCODE_INTERNATIONAL2,
  SDL_SCANCODE_INTERNATIONAL3, SDL_SCANCODE_INTERNATIONAL4, SDL_SCANCODE_INTERNATIONAL5,
  SDL_SCANCODE_INTERNATIONAL6, SDL_SCANCODE_INTERNATIONAL7, SDL_SCANCODE_INTERNATIONAL8,
  SDL_SCANCODE_INTERNATIONAL9, SDL_SCANCODE_J, SDL_SCANCODE_K, SDL_SCANCODE_KBDILLUMDOWN,
  SDL_SCANCODE_KBDILLUMTOGGLE, SDL_SCANCODE_KBDILLUMUP, SDL_SCANCODE_KP_0, SDL_SCANCODE_KP_00,
  SDL_SCANCODE_KP_000, SDL_SCANCODE_KP_1, SDL_SCANCODE_KP_2, SDL_SCANCODE_KP_3, SDL_SCANCODE_KP_4,
  SDL_SCANCODE_KP_5, SDL_SCANCODE_KP_6, SDL_SCANCODE_KP_7, SDL_SCANCODE_KP_8, SDL_SCANCODE_KP_9,
  SDL_SCANCODE_KP_A, SDL_SCANCODE_KP_AMPERSAND, SDL_SCANCODE_KP_AT, SDL_SCANCODE_KP_B,
  SDL_SCANCODE_KP_BACKSPACE, SDL_SCANCODE_KP_BINARY, SDL_SCANCODE_KP_C, SDL_SCANCODE_KP_CLEAR,
  SDL_SCANCODE_KP_CLEARENTRY, SDL_SCANCODE_KP_COLON, SDL_SCANCODE_KP_COMMA, SDL_SCANCODE_KP_D,
  SDL_SCANCODE_KP_DBLAMPERSAND, SDL_SCANCODE_KP_DBLVERTICALBAR, SDL_SCANCODE_KP_DECIMAL,
  SDL_SCANCODE_KP_DIVIDE, SDL_SCANCODE_KP_E, SDL_SCANCODE_KP_ENTER, SDL_SCANCODE_KP_EQUALS,
  SDL_SCANCODE_KP_EQUALSAS400, SDL_SCANCODE_KP_EXCLAM, SDL_SCANCODE_KP_F, SDL_SCANCODE_KP_GREATER,
  SDL_SCANCODE_KP_HASH, SDL_SCANCODE_KP_HEXADECIMAL, SDL_SCANCODE_KP_LEFTBRACE,
  SDL_SCANCODE_KP_LEFTPAREN, SDL_SCANCODE_KP_LESS, SDL_SCANCODE_KP_MEMADD,
  SDL_SCANCODE_KP_MEMCLEAR, SDL_SCANCODE_KP_MEMDIVIDE, SDL_SCANCODE_KP_MEMMULTIPLY,
  SDL_SCANCODE_KP_MEMRECALL, SDL_SCANCODE_KP_MEMSTORE, SDL_SCANCODE_KP_MEMSUBTRACT,
  SDL_SCANCODE_KP_MINUS, SDL_SCANCODE_KP_MULTIPLY, SDL_SCANCODE_KP_OCTAL, SDL_SCANCODE_KP_PERCENT,
  SDL_SCANCODE_KP_PERIOD, SDL_SCANCODE_KP_PLUS, SDL_SCANCODE_KP_PLUSMINUS, SDL_SCANCODE_KP_POWER,
  SDL_SCANCODE_KP_RIGHTBRACE, SDL_SCANCODE_KP_RIGHTPAREN, SDL_SCANCODE_KP_SPACE,
  SDL_SCANCODE_KP_TAB, SDL_SCANCODE_KP_VERTICALBAR, SDL_SCANCODE_KP_XOR, SDL_SCANCODE_L,
  SDL_SCANCODE_LALT, SDL_SCANCODE_LANG1, SDL_SCANCODE_LANG2, SDL_SCANCODE_LANG3,
  SDL_SCANCODE_LANG4, SDL_SCANCODE_LANG5, SDL_SCANCODE_LANG6, SDL_SCANCODE_LANG7,
  SDL_SCANCODE_LANG8, SDL_SCANCODE_LANG9, SDL_SCANCODE_LCTRL, SDL_SCANCODE_LEFT,
  SDL_SCANCODE_LEFTBRACKET, SDL_SCANCODE_LGUI, SDL_SCANCODE_LSHIFT, SDL_SCANCODE_M,
  SDL_SCANCODE_MAIL, SDL_SCANCODE_MEDIASELECT, SDL_SCANCODE_MENU, SDL_SCANCODE_MINUS,
  SDL_SCANCODE_MODE, SDL_SCANCODE_MUTE, SDL_SCANCODE_N, SDL_SCANCODE_NONUSBACKSLASH,
  SDL_SCANCODE_NONUSHASH, SDL_SCANCODE_NUMLOCKCLEAR, SDL_SCANCODE_O, SDL_SCANCODE_OPER,
  SDL_SCANCODE_OUT, SDL_SCANCODE_P, SDL_SCANCODE_PAGEDOWN, SDL_SCANCODE_PAGEUP, SDL_SCANCODE_PASTE,
  SDL_SCANCODE_PAUSE, SDL_SCANCODE_PERIOD, SDL_SCANCODE_POWER, SDL_SCANCODE_PRINTSCREEN,
  SDL_SCANCODE_PRIOR, SDL_SCANCODE_Q, SDL_SCANCODE_R, SDL_SCANCODE_RALT, SDL_SCANCODE_RCTRL,
  SDL_SCANCODE_RETURN, SDL_SCANCODE_RETURN2, SDL_SCANCODE_RGUI, SDL_SCANCODE_RIGHT,
  SDL_SCANCODE_RIGHTBRACKET, SDL_SCANCODE_RSHIFT, SDL_SCANCODE_S, SDL_SCANCODE_SCROLLLOCK,
  SDL_SCANCODE_SELECT, SDL_SCANCODE_SEMICOLON, SDL_SCANCODE_SEPARATOR, SDL_SCANCODE_SLASH,
  SDL_SCANCODE_SLEEP, SDL_SCANCODE_SPACE, SDL_SCANCODE_STOP, SDL_SCANCODE_SYSREQ, SDL_SCANCODE_T,
  SDL_SCANCODE_TAB, SDL_SCANCODE_THOUSANDSSEPARATOR, SDL_SCANCODE_U, SDL_SCANCODE_UNDO,
  SDL_SCANCODE_UP, SDL_SCANCODE_V, SDL_SCANCODE_VOLUMEDOWN, SDL_SCANCODE_VOLUMEUP, SDL_SCANCODE_W,
  SDL_SCANCODE_WWW, SDL_SCANCODE_X, SDL_SCANCODE_Y, SDL_SCANCODE_Z,
};
