#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Demo of how to setup a blank window using the `glium` crate for OpenGL.
//!
//! Note that we have to implement quite a bit of glue code to make the two libs
//! interact.
//!
//! MANY THANKS TO [nukep](https://github.com/nukep)! I used their `glium-sdl2`
//! crate as a guide to figure out how to make a custom backend for `glium`.

use beryllium::*;
use glium;

use core::{ffi::c_void, marker::PhantomData};
use glium::{
  backend::{Backend, Context, Facade},
  debug::DebugCallbackBehavior,
  Frame, Surface, SwapBuffersError,
};
use std::rc::Rc;

fn main() -> Result<(), String> {
  // Init SDL2
  let sdl = unsafe { beryllium::init() }?;

  // Make a window (include the flag for OpenGL support!)
  let window = sdl.create_window(
    "Extern Crate: `glium`",                  // title
    WINDOW_POSITION_CENTERED,                 // x
    WINDOW_POSITION_CENTERED,                 // y
    800,                                      // width
    600,                                      // height
    WindowFlags::default().with_opengl(true), // flags
  )?;

  sdl.gl_set_attribute(GLattr::ContextProfileMask, CONTEXT_PROFILE_CORE);
  sdl.gl_set_attribute(GLattr::ContextMajorVersion, 3);
  sdl.gl_set_attribute(GLattr::ContextMinorVersion, 3);

  let facade = BerylliumFacade::from_window(window, &sdl)?;

  'game_loop: loop {
    while let Some(event) = sdl.poll_event() {
      #[allow(clippy::single_match)]
      match event {
        Event::Quit { timestamp } => {
          println!("Quitting the program after {} milliseconds.", timestamp);
          break 'game_loop;
        }
        _ => (),
      }
    }

    let mut frame = facade.draw();
    frame.clear_color(1.0, 0.5, 0.0, 1.0);
    frame.finish().map_err(|e| e.to_string())?;
  }

  Ok(())
}

// HERE BEGINS ALL THE LINK UP CODE TO CONNECT GLIUM AND BERYLLIUM

/// This holds the backend parts that beryllium gives you.
///
/// We keep them in their raw form and keep a phantom link to the SDLToken
/// because it makes the whole lifetime thing a lot easier to deal with.
pub struct BerylliumBackend<'sdl> {
  win_ptr: *mut beryllium::unsafe_raw_ffi::SDL_Window,
  ctx: beryllium::unsafe_raw_ffi::SDL_GLContext,
  _marker: PhantomData<&'sdl SDLToken>,
}

impl Drop for BerylliumBackend<'_> {
  /// Because the backend parts are held in raw form, we have to manually,
  /// carefully transmute them back to their "wrapped" form so that they can
  /// perform their normal drop.
  ///
  /// Note: ManuallyDrop wouldn't improve things here, since the point of using
  /// the raw forms is to fake out the lifetime system.
  fn drop(&mut self) {
    unsafe {
      // We must drop the context first so it doesn't outlive the window.
      let context: GLContext = core::mem::transmute(self.ctx);
      drop(context);
      let window: Window = core::mem::transmute(self.win_ptr);
      drop(window);
    }
  }
}

impl<'sdl> BerylliumBackend<'sdl> {
  /// Given just a window, creates a context and returns a backend.
  ///
  /// You have to have set the correct variables ahead of time so that the
  /// correct context is created.
  pub fn from_window(window: Window<'sdl>) -> Result<Self, String> {
    unsafe {
      let context: GLContext = window.gl_create_context()?;
      let ctx: beryllium::unsafe_raw_ffi::SDL_GLContext = core::mem::transmute(context);
      let win_ptr: *mut beryllium::unsafe_raw_ffi::SDL_Window = core::mem::transmute(window);
      Ok(Self {
        win_ptr,
        ctx,
        _marker: PhantomData,
      })
    }
  }

  /// Reference to the held Window.
  pub fn window(&self) -> &Window<'sdl> {
    let r: &*mut beryllium::unsafe_raw_ffi::SDL_Window = &self.win_ptr;
    unsafe { &*(r as *const *mut beryllium::unsafe_raw_ffi::SDL_Window as *const Window<'sdl>) }
  }

  /// Reference to the held GLContext
  pub fn context(&self) -> &GLContext<'sdl, 'sdl> {
    let r: &beryllium::unsafe_raw_ffi::SDL_GLContext = &self.ctx;
    unsafe {
      &*(r as *const beryllium::unsafe_raw_ffi::SDL_GLContext as *const GLContext<'sdl, 'sdl>)
    }
  }
}

/// We need to implement [glium::backend::Backend] so that we can use our
/// backend type with [glium::backend::Context].
unsafe impl Backend for BerylliumBackend<'_> {
  fn swap_buffers(&self) -> Result<(), SwapBuffersError> {
    // Note: this can't return error messages to us, so we can't pass any error
    // messages to the caller. Good luck!
    unsafe { self.window().gl_swap_window() };
    Ok(())
  }
  unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
    // Note: We can make up a &SDLToken "out of nowhere" because if this window
    // exists then naturally SDL2 is currently initialized. We choose a
    // reference instead of an owned value so that we don't drop the token.
    (&*(&() as *const () as *const beryllium::SDLToken)).gl_get_proc_address(symbol)
  }
  fn get_framebuffer_dimensions(&self) -> (u32, u32) {
    let (w, h) = self.window().gl_get_drawable_size();
    (w as u32, h as u32)
  }
  fn is_current(&self) -> bool {
    self.context().is_current()
  }
  unsafe fn make_current(&self) {
    self
      .window()
      .gl_make_current(self.context())
      .expect("Could not make the context current!")
  }
}

/// Merges a Beryllium backend with a Glium context
///
/// Once again, we just lie to the compiler _a little bit_ to make our lives
/// easier, and then fix it with phantom data.
pub struct BerylliumFacade<'sdl> {
  backend: Rc<BerylliumBackend<'static>>,
  context: Rc<Context>,
  _marker: PhantomData<&'sdl SDLToken>,
}

/// This is needed for general compatability with a lot of the glium stuff.
impl Facade for BerylliumFacade<'_> {
  fn get_context(&self) -> &Rc<Context> {
    &self.context
  }
}

impl<'sdl> BerylliumFacade<'sdl> {
  /// Reference to the deeply wrapped Window.
  pub fn window(&self) -> &Window {
    self.backend.window()
  }

  /// Starts a new draw frame.
  pub fn draw(&self) -> Frame {
    Frame::new(
      self.context.clone(),
      self.backend.get_framebuffer_dimensions(),
    )
  }

  /// Given a window, makes a facade.
  ///
  /// Because of shenanigans, you also need to pass a reference to the SDL
  /// token.
  pub fn from_window(window: Window<'_>, _t: &'sdl SDLToken) -> Result<Self, String> {
    unsafe {
      let window_static: Window<'static> = core::mem::transmute(window);
      let backend = Rc::new(BerylliumBackend::from_window(window_static)?);
      let context = Context::new(backend.clone(), true, DebugCallbackBehavior::default())
        .map_err(|e| e.to_string())?;
      Ok(Self {
        backend,
        context,
        _marker: PhantomData,
      })
    }
  }
}
