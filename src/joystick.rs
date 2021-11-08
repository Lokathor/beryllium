use core::ptr::NonNull;

use crate::{get_error, init::Sdl, SdlResult};

use fermium::prelude::{
  SDL_Joystick, SDL_JoystickClose, SDL_JoystickGetAttached, SDL_JoystickGetGUID,
  SDL_JoystickGetProduct, SDL_JoystickGetSerial, SDL_JoystickGetVendor, SDL_JoystickInstanceID,
  SDL_JoystickName, SDL_JoystickNumAxes, SDL_JoystickNumBalls, SDL_JoystickNumButtons,
  SDL_JoystickNumHats, SDL_JoystickOpen, SDL_NumJoysticks, SDL_TRUE,
};
use zstring::ZStr;

impl Sdl {
  #[inline]
  pub fn get_num_joysticks(&self) -> i32 {
    unsafe { SDL_NumJoysticks() }
  }

  #[inline]
  pub fn joystick_open(&self, joystick_index: i32) -> SdlResult<Joystick> {
    match NonNull::new(unsafe { SDL_JoystickOpen(joystick_index) }) {
      Some(nn) => Ok(Joystick { nn, sdl: self.clone() }),
      None => Err(get_error()),
    }
  }
}

pub struct Joystick {
  pub(crate) nn: NonNull<SDL_Joystick>,
  #[allow(unused)]
  sdl: Sdl,
}
impl Drop for Joystick {
  fn drop(&mut self) {
    unsafe { SDL_JoystickClose(self.nn.as_ptr()) };
  }
}
impl Joystick {
  #[inline]
  pub fn name<'a>(&'a self) -> Option<ZStr<'a>> {
    unsafe {
      NonNull::new(SDL_JoystickName(self.nn.as_ptr()) as _)
        .map(|nn| ZStr::from_non_null_unchecked(nn))
    }
  }

  #[inline]
  pub fn serial<'a>(&'a self) -> Option<ZStr<'a>> {
    unsafe {
      NonNull::new(SDL_JoystickGetSerial(self.nn.as_ptr()) as _)
        .map(|nn| ZStr::from_non_null_unchecked(nn))
    }
  }

  #[inline]
  pub fn get_vendor_id(&self) -> u16 {
    unsafe { SDL_JoystickGetVendor(self.nn.as_ptr()) }
  }

  #[inline]
  pub fn get_product_id(&self) -> u16 {
    unsafe { SDL_JoystickGetProduct(self.nn.as_ptr()) }
  }

  #[inline]
  pub fn get_guid(&self) -> [u8; 16] {
    unsafe { SDL_JoystickGetGUID(self.nn.as_ptr()).data }
  }

  #[inline]
  pub fn get_num_axes(&self) -> i32 {
    unsafe { SDL_JoystickNumAxes(self.nn.as_ptr()) }
  }

  #[inline]
  pub fn get_num_hats(&self) -> i32 {
    unsafe { SDL_JoystickNumHats(self.nn.as_ptr()) }
  }

  #[inline]
  pub fn get_num_balls(&self) -> i32 {
    unsafe { SDL_JoystickNumBalls(self.nn.as_ptr()) }
  }

  #[inline]
  pub fn get_num_buttons(&self) -> i32 {
    unsafe { SDL_JoystickNumButtons(self.nn.as_ptr()) }
  }

  #[inline]
  pub fn get_instance_id(&self) -> i32 {
    unsafe { SDL_JoystickInstanceID(self.nn.as_ptr()).0 }
  }

  #[inline]
  pub fn get_attached(&self) -> bool {
    unsafe { SDL_JoystickGetAttached(self.nn.as_ptr()) == SDL_TRUE }
  }
}
