use super::*;

/// Handle to a "texture", a GPU-side image.
///
/// This is harder to directly edit, but operations are faster, and you can
/// display it in the Window.
#[derive(Debug)]
#[repr(transparent)]
pub struct Texture<'sdl, 'win, 'ren> {
  pub(crate) ptr: *mut SDL_Texture,
  pub(crate) _marker: PhantomData<&'ren Renderer<'sdl, 'win>>,
}
impl<'sdl, 'win, 'ren> Drop for Texture<'sdl, 'win, 'ren> {
  fn drop(&mut self) {
    unsafe { SDL_DestroyTexture(self.ptr) }
  }
}
