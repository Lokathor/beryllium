/// Obtains the current SDL2 error string.
///
/// You should never need to call this yourself, but I guess you can if you
/// really want.
pub fn get_error() -> String {
  unsafe {
    let base = fermium::SDL_GetError();
    let len = fermium::SDL_strlen(base);
    let useful_bytes = core::slice::from_raw_parts(base as *const u8, len);
    String::from_utf8_lossy(useful_bytes).into_owned()
  }
}

pub type InitFlags = u32;

#[derive(Debug)]
pub struct SDLToken;

/// Initializes SDL2 and gives you a token as proof, or an error message.
///
/// # Safety
///
/// You must not double-initialize SDL2. Other than that this is fully safe.
pub unsafe fn init(flags: InitFlags) -> Result<SDLToken, String> {
  unimplemented!()
}
