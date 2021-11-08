use alloc::{string::String, vec::Vec};
use bytemuck::cast;
use fermium::prelude::{
  SDL_DisplayOrientation, SDL_Event, SDL_Keycode, SDL_PollEvent, SDL_Scancode, SDL_free,
  SDL_AUDIODEVICEADDED, SDL_AUDIODEVICEREMOVED, SDL_CONTROLLERAXISMOTION, SDL_CONTROLLERBUTTONDOWN,
  SDL_CONTROLLERBUTTONUP, SDL_CONTROLLERDEVICEADDED, SDL_CONTROLLERDEVICEREMAPPED,
  SDL_CONTROLLERDEVICEREMOVED, SDL_DISPLAYEVENT, SDL_DISPLAYEVENT_CONNECTED,
  SDL_DISPLAYEVENT_DISCONNECTED, SDL_DISPLAYEVENT_ORIENTATION, SDL_DROPBEGIN, SDL_DROPCOMPLETE,
  SDL_DROPFILE, SDL_DROPTEXT, SDL_FINGERDOWN, SDL_FINGERMOTION, SDL_FINGERUP, SDL_JOYAXISMOTION,
  SDL_JOYBALLMOTION, SDL_JOYBUTTONDOWN, SDL_JOYBUTTONUP, SDL_JOYDEVICEADDED, SDL_JOYDEVICEREMOVED,
  SDL_JOYHATMOTION, SDL_KEYDOWN, SDL_KEYUP, SDL_MOUSEBUTTONDOWN, SDL_MOUSEBUTTONUP,
  SDL_MOUSEMOTION, SDL_MOUSEWHEEL, SDL_MOUSEWHEEL_FLIPPED, SDL_ORIENTATION_LANDSCAPE,
  SDL_ORIENTATION_LANDSCAPE_FLIPPED, SDL_ORIENTATION_PORTRAIT, SDL_ORIENTATION_PORTRAIT_FLIPPED,
  SDL_PRESSED, SDL_QUIT, SDL_TEXTEDITING, SDL_TEXTINPUT, SDL_WINDOWEVENT, SDL_WINDOWEVENT_CLOSE,
  SDL_WINDOWEVENT_ENTER, SDL_WINDOWEVENT_EXPOSED, SDL_WINDOWEVENT_FOCUS_GAINED,
  SDL_WINDOWEVENT_FOCUS_LOST, SDL_WINDOWEVENT_HIDDEN, SDL_WINDOWEVENT_HIT_TEST,
  SDL_WINDOWEVENT_LEAVE, SDL_WINDOWEVENT_MAXIMIZED, SDL_WINDOWEVENT_MINIMIZED,
  SDL_WINDOWEVENT_MOVED, SDL_WINDOWEVENT_RESIZED, SDL_WINDOWEVENT_RESTORED, SDL_WINDOWEVENT_SHOWN,
  SDL_WINDOWEVENT_SIZE_CHANGED, SDL_WINDOWEVENT_TAKE_FOCUS,
};
use tinyvec::ArrayVec;

use crate::{init::Sdl, min_alloc_lossy_into_string};

impl Sdl {
  #[inline]
  #[must_use]
  pub fn poll_event(&self) -> Option<Event> {
    let mut e = SDL_Event::default();
    if unsafe { SDL_PollEvent(&mut e) } == 0 {
      None
    } else {
      Event::try_from(e).ok()
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum DisplayOrientation {
  Portrait,
  Landscape,
  PortraitFlipped,
  LandscapeFlipped,
  Unknown,
}

// TODO: the various ID types should probably be newtypes.

#[derive(Debug)]
pub enum Event {
  Quit,
  Keyboard {
    timestamp: u32,
    window_id: u32,
    is_pressed: bool,
    repeat: u8,
    // TODO: better types here
    scancode: SDL_Scancode,
    keycode: SDL_Keycode,
    modifiers: u16,
  },
  MouseButton {
    timestamp: u32,
    window_id: u32,
    mouse_id: u32,
    button: u8,
    is_pressed: bool,
    click_count: u8,
    win_x: i32,
    win_y: i32,
  },
  MouseMotion {
    timestamp: u32,
    window_id: u32,
    mouse_id: u32,
    button_state: u32,
    win_x: i32,
    win_y: i32,
    delta_x: i32,
    delta_y: i32,
  },
  MouseWheel {
    timestamp: u32,
    window_id: u32,
    mouse_id: u32,
    /// positive to the right and negative to the left
    delta_x: i32,
    /// positive away from the user and negative toward the user
    delta_y: i32,
  },
  /// A new audio device was added.
  AudioDeviceAdded {
    timestamp: u32,
    /// Valid until the next call to
    /// [get_num_audio_devices](Sdl::get_num_audio_devices).
    ///
    /// Note that the the "removed" events only come in for opened audio
    /// devices, so if the device is not opened immediately then trying to open
    /// this index later on might fail just because the device is no longer
    /// connected.
    audio_device_index: u32,
    is_capture: bool,
  },
  /// An opened audio device was removed.
  AudioDeviceRemoved {
    timestamp: u32,
    audio_device_id: u32,
  },
  DisplayConnected {
    timestamp: u32,
    display_id: u32,
  },
  DisplayDisconnected {
    timestamp: u32,
    display_id: u32,
  },
  DisplayOrientationChange {
    timestamp: u32,
    display_id: u32,
    orientation: DisplayOrientation,
  },
  /// Text and/or filenames are about to be dropped into a window.
  DropBegin {
    timestamp: u32,
    window_id: u32,
  },
  /// The drop process is completed.
  DropComplete {
    timestamp: u32,
    window_id: u32,
  },
  /// A filename dropped into the window.
  DropFilename {
    timestamp: u32,
    window_id: u32,
    name: String,
  },
  /// Text dropped into the window.
  ///
  /// **Note:** Only works with X11 on Linux!
  DropText {
    timestamp: u32,
    window_id: u32,
    text: String,
  },
  TextInput {
    timestamp: u32,
    window_id: u32,
    /// The text input bytes.
    ///
    /// These *should* be UTF-8 encoded, so
    /// [str::from_utf8](core::str::from_utf8) *should* return `Ok` on the
    /// slice of the array data.
    text: ArrayVec<[u8; 32]>,
  },
  TextEditing {
    timestamp: u32,
    window_id: u32,
    text: ArrayVec<[u8; 32]>,
    cursor: i32,
  },
  WindowShown {
    timestamp: u32,
    window_id: u32,
  },
  WindowHidden {
    timestamp: u32,
    window_id: u32,
  },
  WindowExposed {
    timestamp: u32,
    window_id: u32,
  },
  WindowMoved {
    timestamp: u32,
    window_id: u32,
    x: i32,
    y: i32,
  },
  /// The window has been resized by an external event.
  ///
  /// The dimensions given are in screen coordinates (I think).
  WindowResized {
    timestamp: u32,
    window_id: u32,
    width: u32,
    height: u32,
  },
  /// The window's size has changed.
  ///
  /// If the size was changed by an external event, then this event will be
  /// followed by a `WindowResized` event.
  WindowSizeChanged {
    timestamp: u32,
    window_id: u32,
  },
  WindowMinimized {
    timestamp: u32,
    window_id: u32,
  },
  WindowMaximized {
    timestamp: u32,
    window_id: u32,
  },
  WindowRestored {
    timestamp: u32,
    window_id: u32,
  },
  WindowMouseEnter {
    timestamp: u32,
    window_id: u32,
  },
  WindowMouseLeave {
    timestamp: u32,
    window_id: u32,
  },
  WindowKeyboardFocusGained {
    timestamp: u32,
    window_id: u32,
  },
  WindowKeyboardFocusLost {
    timestamp: u32,
    window_id: u32,
  },
  WindowClose {
    timestamp: u32,
    window_id: u32,
  },
  WindowTakeFocus {
    timestamp: u32,
    window_id: u32,
  },
  WindowHitTest {
    timestamp: u32,
    window_id: u32,
  },
  TouchFingerDown {
    timestamp: u32,
    touch_id: i64,
    finger_id: i64,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    pressure: f32,
    window_id: u32,
  },
  TouchFingerUp {
    timestamp: u32,
    touch_id: i64,
    finger_id: i64,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    pressure: f32,
    window_id: u32,
  },
  TouchFingerMotion {
    timestamp: u32,
    touch_id: i64,
    finger_id: i64,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    pressure: f32,
    window_id: u32,
  },
  JoystickAxis {
    timestamp: u32,
    joystick_id: i32,
    axis: u8,
    value: i16,
  },
  JoystickBall {
    timestamp: u32,
    joystick_id: i32,
    ball: u8,
    dx: i16,
    dy: i16,
  },
  JoystickButton {
    timestamp: u32,
    joystick_id: i32,
    button: u8,
    is_pressed: bool,
  },
  JoystickAdded {
    timestamp: u32,
    index: i32,
  },
  JoystickRemoved {
    timestamp: u32,
    joystick_id: i32,
  },
  JoystickHat {
    timestamp: u32,
    joystick_id: i32,
    hat: u8,
    value: u8,
  },
  ControllerAxis {
    timestamp: u32,
    controller_id: i32,
    axis: u8,
    value: i16,
  },
  ControllerButton {
    timestamp: u32,
    controller_id: i32,
    button: u8,
    is_pressed: bool,
  },
  ControllerAdded {
    timestamp: u32,
    joystick_index: i32,
  },
  ControllerRemoved {
    timestamp: u32,
    controller_id: i32,
  },
  ControllerRemapped {
    timestamp: u32,
    controller_id: i32,
  },
  /* rustfmt i hate you */

  /*SDL_ControllerSensorEvent
   *SDL_ControllerTouchpadEvent
   *SDL_DollarGestureEvent
   *SDL_MultiGestureEvent
   *SDL_OSEvent
   *SDL_SensorEvent
   *SDL_SysWMEvent
   *SDL_UserEvent
   */
}
impl TryFrom<SDL_Event> for Event {
  type Error = ();
  #[inline]
  fn try_from(event: SDL_Event) -> Result<Self, Self::Error> {
    unsafe {
      Ok(match event.type_ {
        SDL_QUIT => Self::Quit,
        SDL_KEYDOWN | SDL_KEYUP => Self::Keyboard {
          timestamp: event.key.timestamp,
          window_id: event.key.windowID,
          is_pressed: event.key.state == SDL_PRESSED,
          repeat: event.key.repeat,
          scancode: event.key.keysym.scancode,
          keycode: event.key.keysym.sym,
          modifiers: event.key.keysym.mod_,
        },
        SDL_MOUSEBUTTONDOWN | SDL_MOUSEBUTTONUP => Self::MouseButton {
          timestamp: event.button.timestamp,
          window_id: event.button.windowID,
          mouse_id: event.button.which,
          button: event.button.button,
          is_pressed: event.button.state == SDL_PRESSED,
          click_count: event.button.clicks,
          win_x: event.button.x,
          win_y: event.button.y,
        },
        SDL_MOUSEMOTION => Self::MouseMotion {
          timestamp: event.motion.timestamp,
          window_id: event.motion.windowID,
          mouse_id: event.motion.which,
          button_state: event.motion.state,
          win_x: event.motion.x,
          win_y: event.motion.y,
          delta_x: event.motion.xrel,
          delta_y: event.motion.yrel,
        },
        SDL_MOUSEWHEEL => Self::MouseWheel {
          timestamp: event.wheel.timestamp,
          window_id: event.wheel.windowID,
          mouse_id: event.wheel.which,
          delta_x: event.wheel.x
            * if event.wheel.direction == SDL_MOUSEWHEEL_FLIPPED { -1 } else { 1 },
          delta_y: event.wheel.y
            * if event.wheel.direction == SDL_MOUSEWHEEL_FLIPPED { -1 } else { 1 },
        },
        SDL_AUDIODEVICEADDED => Self::AudioDeviceAdded {
          timestamp: event.adevice.timestamp,
          audio_device_index: event.adevice.which,
          is_capture: event.adevice.iscapture != 0,
        },
        SDL_AUDIODEVICEREMOVED => Self::AudioDeviceRemoved {
          timestamp: event.adevice.timestamp,
          audio_device_id: event.adevice.which,
        },
        SDL_DISPLAYEVENT => match event.display.event {
          SDL_DISPLAYEVENT_CONNECTED => Self::DisplayConnected {
            timestamp: event.display.timestamp,
            display_id: event.display.display,
          },
          SDL_DISPLAYEVENT_DISCONNECTED => Self::DisplayDisconnected {
            timestamp: event.display.timestamp,
            display_id: event.display.display,
          },
          SDL_DISPLAYEVENT_ORIENTATION => Self::DisplayOrientationChange {
            timestamp: event.display.timestamp,
            display_id: event.display.display,
            orientation: match SDL_DisplayOrientation(event.display.data1 as u32) {
              SDL_ORIENTATION_PORTRAIT => DisplayOrientation::Portrait,
              SDL_ORIENTATION_LANDSCAPE => DisplayOrientation::Landscape,
              SDL_ORIENTATION_PORTRAIT_FLIPPED => DisplayOrientation::PortraitFlipped,
              SDL_ORIENTATION_LANDSCAPE_FLIPPED => DisplayOrientation::LandscapeFlipped,
              _ => DisplayOrientation::Unknown,
            },
          },
          _ => return Err(()),
        },
        SDL_DROPBEGIN => {
          Self::DropBegin { timestamp: event.drop.timestamp, window_id: event.drop.windowID }
        }
        SDL_DROPCOMPLETE => {
          Self::DropComplete { timestamp: event.drop.timestamp, window_id: event.drop.windowID }
        }
        SDL_DROPFILE => Self::DropFilename {
          timestamp: event.drop.timestamp,
          window_id: event.drop.windowID,
          name: {
            let mut v = Vec::new();
            let mut p = event.drop.file as *const u8;
            let mut len = 0;
            while *p != 0 {
              v.push(*p);
              p = p.add(1);
              len += 1;
            }
            v.set_len(len);
            SDL_free(event.drop.file as _);
            min_alloc_lossy_into_string(v)
          },
        },
        SDL_DROPTEXT => Self::DropText {
          timestamp: event.drop.timestamp,
          window_id: event.drop.windowID,
          text: {
            let mut v = Vec::new();
            let mut p = event.drop.file as *const u8;
            let mut len = 0;
            while *p != 0 {
              v.push(*p);
              p = p.add(1);
              len += 1;
            }
            v.set_len(len);
            SDL_free(event.drop.file as _);
            min_alloc_lossy_into_string(v)
          },
        },
        SDL_TEXTINPUT => Self::TextInput {
          timestamp: event.text.timestamp,
          window_id: event.text.windowID,
          text: ArrayVec::from_array_len(
            cast(event.text.text),
            event.text.text.iter().copied().position(|u| u == 0).unwrap_or(32),
          ),
        },
        SDL_TEXTEDITING => Self::TextEditing {
          timestamp: event.edit.timestamp,
          window_id: event.edit.windowID,
          text: ArrayVec::from_array_len(
            cast(event.edit.text),
            event.edit.length.try_into().unwrap_or(32),
          ),
          cursor: event.edit.start,
        },
        SDL_WINDOWEVENT => match event.window.event {
          SDL_WINDOWEVENT_SHOWN => Self::WindowShown {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_HIDDEN => Self::WindowHidden {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_EXPOSED => Self::WindowExposed {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_MOVED => Self::WindowMoved {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
            x: event.window.data1,
            y: event.window.data2,
          },
          SDL_WINDOWEVENT_RESIZED => Self::WindowResized {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
            width: event.window.data1 as _,
            height: event.window.data2 as _,
          },
          SDL_WINDOWEVENT_SIZE_CHANGED => Self::WindowSizeChanged {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_MINIMIZED => Self::WindowMinimized {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_MAXIMIZED => Self::WindowMaximized {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_RESTORED => Self::WindowRestored {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_ENTER => Self::WindowMouseEnter {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_LEAVE => Self::WindowMouseLeave {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_FOCUS_GAINED => Self::WindowKeyboardFocusGained {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_FOCUS_LOST => Self::WindowKeyboardFocusLost {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_CLOSE => Self::WindowClose {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_TAKE_FOCUS => Self::WindowTakeFocus {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          SDL_WINDOWEVENT_HIT_TEST => Self::WindowHitTest {
            timestamp: event.window.timestamp,
            window_id: event.window.windowID,
          },
          _ => return Err(()),
        },
        SDL_FINGERDOWN => Self::TouchFingerDown {
          timestamp: event.tfinger.timestamp,
          touch_id: event.tfinger.touchId.0,
          finger_id: event.tfinger.fingerId.0,
          x: event.tfinger.x,
          y: event.tfinger.y,
          dx: event.tfinger.dx,
          dy: event.tfinger.dy,
          pressure: event.tfinger.pressure,
          window_id: event.tfinger.windowID,
        },
        SDL_FINGERUP => Self::TouchFingerUp {
          timestamp: event.tfinger.timestamp,
          touch_id: event.tfinger.touchId.0,
          finger_id: event.tfinger.fingerId.0,
          x: event.tfinger.x,
          y: event.tfinger.y,
          dx: event.tfinger.dx,
          dy: event.tfinger.dy,
          pressure: event.tfinger.pressure,
          window_id: event.tfinger.windowID,
        },
        SDL_FINGERMOTION => Self::TouchFingerUp {
          timestamp: event.tfinger.timestamp,
          touch_id: event.tfinger.touchId.0,
          finger_id: event.tfinger.fingerId.0,
          x: event.tfinger.x,
          y: event.tfinger.y,
          dx: event.tfinger.dx,
          dy: event.tfinger.dy,
          pressure: event.tfinger.pressure,
          window_id: event.tfinger.windowID,
        },
        SDL_JOYAXISMOTION => Self::JoystickAxis {
          timestamp: event.jaxis.timestamp,
          joystick_id: event.jaxis.which.0,
          axis: event.jaxis.axis,
          value: event.jaxis.value,
        },
        SDL_JOYBALLMOTION => Self::JoystickBall {
          timestamp: event.jball.timestamp,
          joystick_id: event.jball.which.0,
          ball: event.jball.ball,
          dx: event.jball.xrel,
          dy: event.jball.yrel,
        },
        SDL_JOYBUTTONDOWN | SDL_JOYBUTTONUP => Self::JoystickButton {
          timestamp: event.jbutton.timestamp,
          joystick_id: event.jbutton.which.0,
          button: event.jbutton.button,
          is_pressed: event.jbutton.state == SDL_PRESSED,
        },
        SDL_JOYDEVICEADDED => {
          Self::JoystickAdded { timestamp: event.jdevice.timestamp, index: event.jdevice.which }
        }
        SDL_JOYDEVICEREMOVED => Self::JoystickRemoved {
          timestamp: event.jdevice.timestamp,
          joystick_id: event.jdevice.which,
        },
        SDL_JOYHATMOTION => Self::JoystickHat {
          timestamp: event.jhat.timestamp,
          joystick_id: event.jhat.which.0,
          hat: event.jhat.hat,
          value: event.jhat.value,
        },
        SDL_CONTROLLERAXISMOTION => Self::ControllerAxis {
          timestamp: event.caxis.timestamp,
          controller_id: event.caxis.which.0,
          axis: event.caxis.axis,
          value: event.caxis.value,
        },
        SDL_CONTROLLERBUTTONDOWN | SDL_CONTROLLERBUTTONUP => Self::ControllerButton {
          timestamp: event.cbutton.timestamp,
          controller_id: event.cbutton.which.0,
          button: event.cbutton.button,
          is_pressed: event.cbutton.state == SDL_PRESSED,
        },
        SDL_CONTROLLERDEVICEADDED => Self::ControllerAdded {
          timestamp: event.cdevice.timestamp,
          joystick_index: event.cdevice.which,
        },
        SDL_CONTROLLERDEVICEREMOVED => Self::ControllerRemoved {
          timestamp: event.cdevice.timestamp,
          controller_id: event.cdevice.which,
        },
        SDL_CONTROLLERDEVICEREMAPPED => Self::ControllerRemapped {
          timestamp: event.cdevice.timestamp,
          controller_id: event.cdevice.which,
        },
        _other => return Err(()),
      })
    }
  }
}
