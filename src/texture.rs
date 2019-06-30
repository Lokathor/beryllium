use super::*;

/// Handle to a "texture", a GPU-side image.
///
/// This is harder to directly edit, but operations are faster, and you can
/// display it in the Window.
#[derive(Debug)]
#[repr(transparent)]
pub struct Texture<'sdl, 'ren> {
  pub(crate) ptr: *mut SDL_Texture,
  pub(crate) _marker: PhantomData<&'ren RendererWindow<'sdl>>,
}
impl<'sdl, 'ren> Drop for Texture<'sdl, 'ren> {
  fn drop(&mut self) {
    unsafe { SDL_DestroyTexture(self.ptr) }
  }
}
