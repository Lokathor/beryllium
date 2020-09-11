use core::ptr::NonNull;

use alloc::string::String;

use tinyvec::TinyVec;

use fermium::SDL_Window;

use crate::sdl_get_error;

pub struct Window {
  nn: NonNull<SDL_Window>,
}

impl Drop for Window {
  fn drop(&mut self) {
    unsafe { fermium::SDL_DestroyWindow(self.nn.as_ptr()) }
  }
}

impl Window {
  /// Makes a new window.
  ///
  /// * If `pos` is `None` you get a centered window.
  /// * `w` and `h` can't exceed 16_384.
  pub fn new(
    title: &str, pos: Option<[i32; 2]>, [w, h]: [u32; 2], flags: u32,
  ) -> Result<Self, String> {
    let title_null: TinyVec<[u8; 64]> =
      title.as_bytes().iter().copied().chain(Some(0)).collect();
    const CENTERED: i32 = fermium::SDL_WINDOWPOS_CENTERED;
    let [p_x, p_y] = pos.unwrap_or([CENTERED, CENTERED]);
    match NonNull::new(unsafe {
      fermium::SDL_CreateWindow(
        title_null.as_ptr().cast(),
        p_x,
        p_y,
        w as i32,
        h as i32,
        flags,
      )
    }) {
      Some(nn) => Ok(Window { nn }),
      None => Err(sdl_get_error()),
    }
  }
}
