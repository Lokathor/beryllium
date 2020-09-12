use core::ptr::NonNull;

use fermium::SDL_Surface;

pub struct Surface {
  nn: NonNull<SDL_Surface>,
}
