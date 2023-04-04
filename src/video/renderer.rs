use crate::surface::Surface;

use super::*;

macro_rules! nz_is_err {
  ($x:expr) => {{
    let ret = $x;
    if ret == 0 {
      Ok(())
    } else {
      Err(get_error())
    }
  }};
}

#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct RendererFlags(SDL_RendererFlags);
impl RendererFlags {
  pub const SOFTWARE: Self = Self(SDL_RENDERER_SOFTWARE);
  pub const ACCELERATED: Self = Self(SDL_RENDERER_ACCELERATED);
  pub const VSYNC: Self = Self(SDL_RENDERER_PRESENTVSYNC);
  pub const TARGETTEXTURE: Self = Self(SDL_RENDERER_TARGETTEXTURE);
  //
  pub const ACCELERATED_VSYNC: Self =
    Self(SDL_RendererFlags(SDL_RENDERER_ACCELERATED.0 | SDL_RENDERER_PRESENTVSYNC.0));
}
impl core::fmt::Debug for RendererFlags {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut s = f.debug_set();
    if (self.0 .0 & Self::SOFTWARE.0 .0) != 0 {
      s.entry(&"Software");
    }
    if (self.0 .0 & Self::ACCELERATED.0 .0) != 0 {
      s.entry(&"Accelerated");
    }
    if (self.0 .0 & Self::VSYNC.0 .0) != 0 {
      s.entry(&"VSync");
    }
    if (self.0 .0 & Self::TARGETTEXTURE.0 .0) != 0 {
      s.entry(&"TargetTexture");
    }
    s.finish()
  }
}

#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct PixelFormatEnum(SDL_PixelFormatEnum);
impl core::fmt::Debug for PixelFormatEnum {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self.0 {
      SDL_PIXELFORMAT_INDEX1LSB => "INDEX1LSB",
      SDL_PIXELFORMAT_INDEX1MSB => "INDEX1MSB",
      SDL_PIXELFORMAT_INDEX4LSB => "INDEX4LSB",
      SDL_PIXELFORMAT_INDEX4MSB => "INDEX4MSB",
      SDL_PIXELFORMAT_INDEX8 => "INDEX8",
      SDL_PIXELFORMAT_RGB332 => "RGB332",
      SDL_PIXELFORMAT_RGB444 => "RGB444",
      SDL_PIXELFORMAT_RGB555 => "RGB555",
      SDL_PIXELFORMAT_BGR555 => "BGR555",
      SDL_PIXELFORMAT_ARGB4444 => "ARGB4444",
      SDL_PIXELFORMAT_RGBA4444 => "RGBA4444",
      SDL_PIXELFORMAT_ABGR4444 => "ABGR4444",
      SDL_PIXELFORMAT_BGRA4444 => "BGRA4444",
      SDL_PIXELFORMAT_ARGB1555 => "ARGB1555",
      SDL_PIXELFORMAT_RGBA5551 => "RGBA5551",
      SDL_PIXELFORMAT_ABGR1555 => "ABGR1555",
      SDL_PIXELFORMAT_BGRA5551 => "BGRA5551",
      SDL_PIXELFORMAT_RGB565 => "RGB565",
      SDL_PIXELFORMAT_BGR565 => "BGR565",
      SDL_PIXELFORMAT_RGB24 => "RGB24",
      SDL_PIXELFORMAT_BGR24 => "BGR24",
      SDL_PIXELFORMAT_RGB888 => "RGB888",
      SDL_PIXELFORMAT_RGBX8888 => "RGBX8888",
      SDL_PIXELFORMAT_BGR888 => "BGR888",
      SDL_PIXELFORMAT_BGRX8888 => "BGRX8888",
      SDL_PIXELFORMAT_ARGB8888 => "ARGB8888",
      SDL_PIXELFORMAT_RGBA8888 => "RGBA8888",
      SDL_PIXELFORMAT_ABGR8888 => "ABGR8888",
      SDL_PIXELFORMAT_BGRA8888 => "BGRA8888",
      SDL_PIXELFORMAT_ARGB2101010 => "ARGB2101010",
      SDL_PIXELFORMAT_YV12 => "YV12",
      SDL_PIXELFORMAT_IYUV => "IYUV",
      SDL_PIXELFORMAT_YUY2 => "YUY2",
      SDL_PIXELFORMAT_UYVY => "UYVY",
      SDL_PIXELFORMAT_YVYU => "YVYU",
      SDL_PIXELFORMAT_NV12 => "NV12",
      SDL_PIXELFORMAT_NV21 => "NV21",
      _ => "?",
    };
    write!(f, "{s}")
  }
}

#[derive(Debug, Clone, Default)]
pub struct RendererInfo {
  pub name: String,
  pub flags: RendererFlags,
  pub texture_formats: Vec<PixelFormatEnum>,
  pub max_texture_width: i32,
  pub max_texture_height: i32,
}

impl Sdl {
  #[inline]
  pub fn get_renderer_driver_infos(&self) -> Result<Vec<RendererInfo>, SdlError> {
    let num_drivers = unsafe { SDL_GetNumRenderDrivers() };
    if num_drivers < 0 {
      return Err(get_error());
    }
    let mut drivers = Vec::new();
    for driver_index in 0..num_drivers {
      let mut raw_info = SDL_RendererInfo::default();
      let get_result = unsafe { SDL_GetRenderDriverInfo(driver_index, &mut raw_info) };
      if get_result < 0 {
        return Err(get_error());
      } else {
        let mut info = RendererInfo::default();
        let mut p = raw_info.name.cast::<u8>();
        while !p.is_null() && unsafe { *p } != 0 {
          info.name.push(unsafe { *p } as char);
          p = unsafe { p.add(1) };
        }
        for format in
          raw_info.texture_formats.iter().copied().take(raw_info.num_texture_formats as _)
        {
          info.texture_formats.push(PixelFormatEnum(SDL_PixelFormatEnum(format)));
        }
        info.flags = RendererFlags(SDL_RendererFlags(raw_info.flags));
        info.max_texture_width = raw_info.max_texture_width;
        info.max_texture_height = raw_info.max_texture_height;
        drivers.push(info);
      }
    }
    Ok(drivers)
  }
}

#[derive(Clone)]
#[repr(C)]
struct Window {
  win: NonNull<SDL_Window>,
  parent: Arc<SdlInit>,
}
impl Deref for Window {
  type Target = CommonWindow;
  #[inline]
  fn deref(&self) -> &Self::Target {
    unsafe { &*(self as *const Self).cast::<CommonWindow>() }
  }
}
impl Drop for Window {
  #[inline]
  fn drop(&mut self) {
    unsafe { SDL_DestroyWindow(self.win.as_ptr()) };
  }
}

#[derive(Clone)]
#[repr(C)]
struct Renderer {
  rend: NonNull<SDL_Renderer>,
  win: Arc<Window>,
}
impl Deref for Renderer {
  type Target = NonNull<SDL_Renderer>;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.rend
  }
}
impl Drop for Renderer {
  #[inline]
  fn drop(&mut self) {
    unsafe { SDL_DestroyRenderer(self.rend.as_ptr()) };
  }
}

#[repr(C)]
pub struct RendererWindow {
  rend: Arc<Renderer>,
  win: Arc<Window>,
  init: Arc<SdlInit>,
}
impl Deref for RendererWindow {
  type Target = CommonWindow;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.win
  }
}
impl Sdl {
  /// You can only have one GL window active!
  #[inline]
  pub fn create_renderer_window(
    &self, args: CreateWinArgs<'_>, flags: RendererFlags,
  ) -> Result<RendererWindow, SdlError> {
    let title_null: String = alloc::format!("{}\0", args.title);
    let win_p: *mut SDL_Window = unsafe {
      SDL_CreateWindow(
        title_null.as_ptr().cast(),
        SDL_WINDOWPOS_CENTERED,
        SDL_WINDOWPOS_CENTERED,
        args.width,
        args.height,
        args.window_flags().0,
      )
    };
    let win = match NonNull::new(win_p) {
      Some(win) => Arc::new(Window { win, parent: self.init.clone() }),
      None => return Err(get_error()),
    };
    let rend_p: *mut SDL_Renderer = unsafe { SDL_CreateRenderer(win_p, -1, flags.0 .0) };
    let rend = match NonNull::new(rend_p) {
      Some(rend) => Arc::new(Renderer { rend, win: win.clone() }),
      None => return Err(get_error()),
    };
    Ok(RendererWindow { rend, win, init: self.init.clone() })
  }
  /// See [SDL_ComposeCustomBlendMode](https://wiki.libsdl.org/SDL2/SDL_ComposeCustomBlendMode)
  #[inline]
  pub fn compose_custom_blend_mode(
    &self, src_color: BlendFactor, dst_color: BlendFactor, color_op: BlendOperation,
    src_alpha: BlendFactor, dst_alpha: BlendFactor, alpha_op: BlendOperation,
  ) -> BlendMode {
    BlendMode(unsafe {
      SDL_ComposeCustomBlendMode(
        SDL_BlendFactor(src_color as _),
        SDL_BlendFactor(dst_color as _),
        SDL_BlendOperation(color_op as _),
        SDL_BlendFactor(src_alpha as _),
        SDL_BlendFactor(dst_alpha as _),
        SDL_BlendOperation(alpha_op as _),
      )
    })
  }
}
impl RendererWindow {
  #[inline]
  pub fn get_renderer_info(&self) -> Result<RendererInfo, SdlError> {
    let mut raw_info = SDL_RendererInfo::default();
    let get_result = unsafe { SDL_GetRendererInfo(self.rend.as_ptr(), &mut raw_info) };
    if get_result < 0 {
      Err(get_error())
    } else {
      let mut info = RendererInfo::default();
      let mut p = raw_info.name.cast::<u8>();
      while !p.is_null() && unsafe { *p } != 0 {
        info.name.push(unsafe { *p } as char);
        p = unsafe { p.add(1) };
      }
      for format in raw_info.texture_formats.iter().copied().take(raw_info.num_texture_formats as _)
      {
        info.texture_formats.push(PixelFormatEnum(SDL_PixelFormatEnum(format)));
      }
      info.flags = RendererFlags(SDL_RendererFlags(raw_info.flags));
      info.max_texture_width = raw_info.max_texture_width;
      info.max_texture_height = raw_info.max_texture_height;
      Ok(info)
    }
  }
  /// See [SDL_SetRenderDrawBlendMode](https://wiki.libsdl.org/SDL2/SDL_SetRenderDrawBlendMode)
  #[inline]
  pub fn set_draw_blend_mode(&self, mode: BlendMode) -> Result<(), SdlError> {
    nz_is_err!(unsafe { SDL_SetRenderDrawBlendMode(self.rend.as_ptr(), mode.0) })
  }
  /// See [SDL_CreateTexture](https://wiki.libsdl.org/SDL2/SDL_CreateTexture)
  #[inline]
  pub fn create_texture_from_surface(&self, surface: &Surface) -> Result<Texture, SdlError> {
    let tex_p = unsafe { SDL_CreateTextureFromSurface(self.rend.as_ptr(), surface.surf.as_ptr()) };
    match NonNull::new(tex_p) {
      Some(tex) => Ok(Texture { tex, parent: self.rend.clone() }),
      None => Err(get_error()),
    }
  }
  /// See [SDL_SetRenderDrawColor](https://wiki.libsdl.org/SDL2/SDL_SetRenderDrawColor)
  #[inline]
  pub fn set_draw_color(&self, r: u8, g: u8, b: u8, a: u8) -> Result<(), SdlError> {
    nz_is_err!(unsafe { SDL_SetRenderDrawColor(self.rend.as_ptr(), r, g, b, a) })
  }
  /// See [SDL_RenderClear](https://wiki.libsdl.org/SDL2/SDL_RenderClear)
  #[inline]
  pub fn clear(&self) -> Result<(), SdlError> {
    nz_is_err!(unsafe { SDL_RenderClear(self.rend.as_ptr()) })
  }
  /// See [SDL_RenderPresent](https://wiki.libsdl.org/SDL2/SDL_RenderPresent)
  #[inline]
  pub fn present(&self) {
    unsafe { SDL_RenderPresent(self.rend.as_ptr()) }
  }
  /// See [SDL_RenderDrawLine](https://wiki.libsdl.org/SDL2/SDL_RenderDrawLine)
  #[inline]
  pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> Result<(), SdlError> {
    nz_is_err!(unsafe { SDL_RenderDrawLine(self.rend.as_ptr(), x1, y1, x2, y2) })
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BlendOperation {
  Add = SDL_BLENDOPERATION_ADD.0,
  Subtract = SDL_BLENDOPERATION_SUBTRACT.0,
  RevSubtract = SDL_BLENDOPERATION_REV_SUBTRACT.0,
  Minimum = SDL_BLENDOPERATION_MINIMUM.0,
  Maximum = SDL_BLENDOPERATION_MAXIMUM.0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BlendFactor {
  Zero = SDL_BLENDFACTOR_ZERO.0,
  One = SDL_BLENDFACTOR_ONE.0,
  SrcColor = SDL_BLENDFACTOR_SRC_COLOR.0,
  OneMinusSrcColor = SDL_BLENDFACTOR_ONE_MINUS_SRC_COLOR.0,
  SrcAlpha = SDL_BLENDFACTOR_SRC_ALPHA.0,
  OneMinusSrcAlpha = SDL_BLENDFACTOR_ONE_MINUS_SRC_ALPHA.0,
  DstColor = SDL_BLENDFACTOR_DST_COLOR.0,
  OneMinusDstColor = SDL_BLENDFACTOR_ONE_MINUS_DST_COLOR.0,
  DstAlpha = SDL_BLENDFACTOR_DST_ALPHA.0,
  OneMinusDstAlpha = SDL_BLENDFACTOR_ONE_MINUS_DST_ALPHA.0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BlendMode(SDL_BlendMode);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TextureAccess {
  Static = SDL_TEXTUREACCESS_STATIC.0,
  Streaming = SDL_TEXTUREACCESS_STREAMING.0,
  Target = SDL_TEXTUREACCESS_TARGET.0,
}

#[repr(C)]
pub struct Texture {
  tex: NonNull<SDL_Texture>,
  parent: Arc<Renderer>,
}
impl Drop for Texture {
  #[inline]
  fn drop(&mut self) {
    unsafe { SDL_DestroyTexture(self.tex.as_ptr()) };
  }
}
