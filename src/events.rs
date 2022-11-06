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
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[rustfmt::skip]
pub enum Event {
  Quit,
  DisplayConnected { display_index: u32 },
  DisplayDisconnected { display_index: u32 },
  DisplayOrientationChanged { display_index: u32 },
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
  /// Mouse cursor change
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
  //ControllerTouchpad { ctrl_id: i32, touchpad: i32, finger: i32, x: f32, y: f32, pressure: f32 },
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
            Event::DisplayOrientationChanged { display_index: v.display }
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
        println!("DropText: {}, {:p}", v.windowID, v.file);
        return Err(());
      }
      SDL_DROPCOMPLETE => Event::DropComplete { win_id: unsafe { sdl_event.drop.windowID } },
      _ => return Err(()),
    })
  }
}
