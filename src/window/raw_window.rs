use super::*;

use core::ptr::null_mut;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

#[cfg(target_os = "macos")]
use raw_window_handle::macos::MacOSHandle;

#[cfg(any(
  target_os = "linux",
  target_os = "dragonfly",
  target_os = "freebsd",
  target_os = "netbsd",
  target_os = "openbsd"
))]
use raw_window_handle::unix::{WaylandHandle, XlibHandle};

#[cfg(windows)]
use raw_window_handle::windows::WindowsHandle;

/// A [`Window`] intended for the [Raw Window Handle](https://docs.rs/raw-window-handle) API.
pub struct RawWindow {
  #[allow(unused)]
  pub(crate) init_token: Arc<Initialization>,
  pub(crate) win: ManuallyDrop<Window>,
}
impl Drop for RawWindow {
  fn drop(&mut self) {
    unsafe {
      ManuallyDrop::drop(&mut self.win);
    }
    WINDOW_EXISTS.store(false, Ordering::SeqCst);
  }
}
impl core::ops::Deref for RawWindow {
  type Target = Window;
  fn deref(&self) -> &Window {
    &self.win
  }
}

unsafe impl HasRawWindowHandle for RawWindow {
  fn raw_window_handle(&self) -> RawWindowHandle {
    // 
    let mut wm_info = fermium::SDL_SysWMinfo::default();
    let b = unsafe { fermium::SDL_GetWindowWMInfo(self.win.win, &mut wm_info) };
    if b == fermium::SDL_TRUE {
      match wm_info.subsystem {
        #[cfg(windows)]
        fermium::SDL_SYSWM_WINDOWS => {
          RawWindowHandle::Windows(WindowsHandle {
            hwnd: unsafe { wm_info.info.win.window as *mut c_void },
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
            surface: unsafe { wm_info.info.wl.surface as *mut c_void },
            display: unsafe { wm_info.info.wl.display as *mut c_void },
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
            display: unsafe { wm_info.info.x11.display as *mut c_void },
            ..XlibHandle::empty()
          })
        }
        #[cfg(target_os = "macos")]
        fermium::SDL_SYSWM_COCOA => {
          RawWindowHandle::MacOS(MacOSHandle {
            ns_window: unsafe { wm_info.info.cocoa.window } as *mut c_void,
            // Note(Lokathor): the `HasRawWindowHandle` spec lets me do this, if
            // you need an `ns_view` you should get it from the OS yourself
            // using the `ns_window` I provide. Sorry, but SDL literally doesn't
            // give you an `ns_window` value for me to pass along.
            // https://wiki.libsdl.org/SDL_SysWMinfo
            ns_view: null_mut(),
            ..MacOSHandle::empty()
          })
        }
        fermium::SDL_SYSWM_UIKIT | fermium::SDL_SYSWM_ANDROID => {
          panic!("SDL2 is using a window subsystem (iOS or Android) that's supported by the raw-window-handle API but not supported yet by beryllium. Whoever did the work to make fermium/beryllium work on iOS and/or Android should have solved this but they didn't. https://github.com/Lokathor/beryllium/issues/new");
        }
        other => panic!("SDL2 is using a window subsystem that is not even supported by the raw-window-handle API, and Osspial wrote the trait to be infallible despite that clearly not always being the case, so now you got this panic. https://github.com/rust-windowing/raw-window-handle/issues/new Window Info was: {:?}",other),
      }
    } else {
      panic!("Could not retrieve any SDL2 window info, and Osspial wrote the trait to be infallible despite that clearly not always being the case, so now you got this panic. https://github.com/rust-windowing/raw-window-handle/issues/new");
    }
  }
}
