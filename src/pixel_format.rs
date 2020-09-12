use core::ptr::NonNull;

use fermium::SDL_PixelFormat;

pub struct PixelFormat {
  nn: NonNull<SDL_PixelFormat>,
}
