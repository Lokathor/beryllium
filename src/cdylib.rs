use super::*;

/// Handle to a C ABI dynamic library that has been loaded.
///
/// You can make your own libs that will work with this using the `cdylib` crate
/// type
/// ([here](https://rust-embedded.github.io/book/interoperability/rust-with-c.html)
/// is a short tutorial).
///
/// Do not attempt to mix this with the `dylib` crate type. That's a crate type
/// you should not use, it's basically for `rustc` internal usage only.
#[derive(Debug)]
#[repr(transparent)]
pub struct CDyLib<'sdl> {
  pub(crate) nn: NonNull<c_void>,
  pub(crate) _marker: PhantomData<&'sdl SDLToken>,
}
impl<'sdl> Drop for CDyLib<'sdl> {
  fn drop(&mut self) {
    unsafe { SDL_UnloadObject(self.nn.as_ptr()) }
  }
}
impl<'sdl> CDyLib<'sdl> {
  /// Attempts to look up a function by name, getting its pointer.
  ///
  /// Once this function returns, you will have to
  /// [transmute](core::mem::transmute) the optional NonNull value you get into
  /// an optional `fn` value of some sort.
  ///
  /// You _probably_ want to transmute it into `Option<unsafe extern "C"
  /// fn(INPUTS) -> OUTPUTS>`, but it's possible that you might need to use some
  /// other ABI for example. This whole thing is obviously not at all safe. You
  /// absolutely must get the `fn` type correct when doing this `transmute`.
  ///
  /// # Safety
  ///
  /// * The returned value _does not_ have a lifetime linking it back to this
  ///   shared library. Making sure that the function pointer is not used after
  ///   this library unloads is up to you.
  pub unsafe fn find_function(&self, name: &str) -> Option<NonNull<c_void>> {
    let name_null: Vec<u8> = name.bytes().chain(Some(0)).collect();
    let name_ptr: *const c_char = name_null.as_ptr() as *const c_char;
    NonNull::new(SDL_LoadFunction(self.nn.as_ptr(), name_ptr))
  }
}
