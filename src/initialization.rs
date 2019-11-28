use super::*;

static I_THINK_THAT_SDL_IS_ACTIVE: AtomicBool = AtomicBool::new(false);
#[derive(Debug, PartialEq)]
#[repr(C)]
pub(crate) struct Initialization(PhantomData<*mut ()>);
impl Initialization {
  pub fn new(flags: InitFlags) -> Result<Self, CowStr> {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
      use objc::{class, msg_send, sel, sel_impl};
      let is_main: bool = unsafe { msg_send![class!(NSThread), isMainThread] };
      if !is_main {
        return Err(cow_str!("SDL must be initialized on the main thread."));
      }
    }
    if I_THINK_THAT_SDL_IS_ACTIVE.swap(true, Ordering::SeqCst) {
      // Reminder: swap gives the old value back.
      Err(cow_str!("SDL is currently initialized."))
    } else if unsafe { fermium::SDL_Init(flags.0) } == 0 {
      Ok(Initialization(PhantomData))
    } else {
      // Safety: Because we hold the Atomic right now no one else can be safely
      // reading the buffer.
      let out = unsafe { get_error_unchecked() };
      I_THINK_THAT_SDL_IS_ACTIVE.store(false, Ordering::SeqCst);
      Err(Cow::Owned(out))
    }
  }

  /// Gets the current SDL error message.
  pub fn get_error(&self) -> String {
    // Safety: Because you have &self, and Initialization isn't Send, no one
    // else can safely be accessing the error buffer right now.
    unsafe { get_error_unchecked() }
  }
}
impl Drop for Initialization {
  fn drop(&mut self) {
    unsafe { fermium::SDL_Quit() }
    I_THINK_THAT_SDL_IS_ACTIVE.store(false, Ordering::SeqCst);
  }
}

/// Flags of what subsystems to initialize.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct InitFlags(u32);
#[allow(non_upper_case_globals)]
impl InitFlags {
  /// Timer subsystem.
  pub const Timer: InitFlags = InitFlags(fermium::SDL_INIT_TIMER);

  /// Audio subsystem.
  pub const Audio: InitFlags = InitFlags(fermium::SDL_INIT_AUDIO);

  /// Video subsystem. Implies Events.
  pub const Video: InitFlags = InitFlags(fermium::SDL_INIT_VIDEO);

  /// Joystick subsystem. Implies Events.
  pub const Joystick: InitFlags = InitFlags(fermium::SDL_INIT_JOYSTICK);

  /// Haptic subsystem (force feedback).
  pub const Haptic: InitFlags = InitFlags(fermium::SDL_INIT_HAPTIC);

  /// Controller API on top of the Joysticks. Implies Joystick.
  pub const GameController: InitFlags =
    InitFlags(fermium::SDL_INIT_GAMECONTROLLER);

  /// Events subsystem.
  pub const Events: InitFlags = InitFlags(fermium::SDL_INIT_EVENTS);

  /// All of the subsystems.
  pub const Everything: InitFlags = InitFlags(fermium::SDL_INIT_EVERYTHING);
}
impl Default for InitFlags {
  /// ```rust
  /// use beryllium::InitFlags;
  /// assert_eq!(InitFlags::default(), InitFlags::Everything);
  /// ```
  fn default() -> Self {
    InitFlags::Everything
  }
}
impl core::ops::BitOr for InitFlags {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self {
    Self(self.0 | rhs.0)
  }
}
impl core::ops::BitOrAssign for InitFlags {
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}
