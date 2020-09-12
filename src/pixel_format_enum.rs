pub struct PixelFormatEnum(pub(crate) u32);
impl PixelFormatEnum {
  pub const INDEX1LSB: Self = Self(fermium::SDL_PIXELFORMAT_INDEX1LSB as u32);
  pub const INDEX1MSB: Self = Self(fermium::SDL_PIXELFORMAT_INDEX1MSB as u32);
  pub const INDEX4LSB: Self = Self(fermium::SDL_PIXELFORMAT_INDEX4LSB as u32);
  pub const INDEX4MSB: Self = Self(fermium::SDL_PIXELFORMAT_INDEX4MSB as u32);
  pub const INDEX8: Self = Self(fermium::SDL_PIXELFORMAT_INDEX8 as u32);
  pub const RGB332: Self = Self(fermium::SDL_PIXELFORMAT_RGB332 as u32);
  pub const RGB444: Self = Self(fermium::SDL_PIXELFORMAT_RGB444 as u32);
  pub const RGB555: Self = Self(fermium::SDL_PIXELFORMAT_RGB555 as u32);
  pub const BGR555: Self = Self(fermium::SDL_PIXELFORMAT_BGR555 as u32);
  pub const ARGB4444: Self = Self(fermium::SDL_PIXELFORMAT_ARGB4444 as u32);
  pub const RGBA4444: Self = Self(fermium::SDL_PIXELFORMAT_RGBA4444 as u32);
  pub const ABGR4444: Self = Self(fermium::SDL_PIXELFORMAT_ABGR4444 as u32);
  pub const BGRA4444: Self = Self(fermium::SDL_PIXELFORMAT_BGRA4444 as u32);
  pub const ARGB1555: Self = Self(fermium::SDL_PIXELFORMAT_ARGB1555 as u32);
  pub const RGBA5551: Self = Self(fermium::SDL_PIXELFORMAT_RGBA5551 as u32);
  pub const ABGR1555: Self = Self(fermium::SDL_PIXELFORMAT_ABGR1555 as u32);
  pub const BGRA5551: Self = Self(fermium::SDL_PIXELFORMAT_BGRA5551 as u32);
  pub const RGB565: Self = Self(fermium::SDL_PIXELFORMAT_RGB565 as u32);
  pub const BGR565: Self = Self(fermium::SDL_PIXELFORMAT_BGR565 as u32);
  pub const RGB24: Self = Self(fermium::SDL_PIXELFORMAT_RGB24 as u32);
  pub const BGR24: Self = Self(fermium::SDL_PIXELFORMAT_BGR24 as u32);
  pub const RGB888: Self = Self(fermium::SDL_PIXELFORMAT_RGB888 as u32);
  pub const RGBX8888: Self = Self(fermium::SDL_PIXELFORMAT_RGBX8888 as u32);
  pub const BGR888: Self = Self(fermium::SDL_PIXELFORMAT_BGR888 as u32);
  pub const BGRX8888: Self = Self(fermium::SDL_PIXELFORMAT_BGRX8888 as u32);
  pub const ARGB8888: Self = Self(fermium::SDL_PIXELFORMAT_ARGB8888 as u32);
  pub const RGBA8888: Self = Self(fermium::SDL_PIXELFORMAT_RGBA8888 as u32);
  pub const ABGR8888: Self = Self(fermium::SDL_PIXELFORMAT_ABGR8888 as u32);
  pub const BGRA8888: Self = Self(fermium::SDL_PIXELFORMAT_BGRA8888 as u32);
  pub const ARGB2101010: Self = Self(fermium::SDL_PIXELFORMAT_ARGB2101010 as u32);
  ///alias for RGBA byte array of color data, for the current platform (>= SDL
  /// 2.0.5)
  pub const RGBA32: Self = Self(fermium::SDL_PIXELFORMAT_RGBA32 as u32);
  ///alias for ARGB byte array of color data, for the current platform (>= SDL
  /// 2.0.5)
  pub const ARGB32: Self = Self(fermium::SDL_PIXELFORMAT_ARGB32 as u32);
  ///alias for BGRA byte array of color data, for the current platform (>= SDL
  /// 2.0.5)
  pub const BGRA32: Self = Self(fermium::SDL_PIXELFORMAT_BGRA32 as u32);
  ///alias for ABGR byte array of color data, for the current platform (>= SDL
  /// 2.0.5)
  pub const ABGR32: Self = Self(fermium::SDL_PIXELFORMAT_ABGR32 as u32);
  ///planar mode: Y + V + U (3 planes)
  pub const YV12: Self = Self(fermium::SDL_PIXELFORMAT_YV12 as u32);
  ///planar mode: Y + U + V (3 planes)
  pub const IYUV: Self = Self(fermium::SDL_PIXELFORMAT_IYUV as u32);
  ///packed mode: Y0+U0+Y1+V0 (1 plane)
  pub const YUY2: Self = Self(fermium::SDL_PIXELFORMAT_YUY2 as u32);
  ///packed mode: U0+Y0+V0+Y1 (1 plane)
  pub const UYVY: Self = Self(fermium::SDL_PIXELFORMAT_UYVY as u32);
  ///packed mode: Y0+V0+Y1+U0 (1 plane)
  pub const YVYU: Self = Self(fermium::SDL_PIXELFORMAT_YVYU as u32);
  ///planar mode: Y + U/V interleaved (2 planes) (>= SDL 2.0.4)
  pub const NV12: Self = Self(fermium::SDL_PIXELFORMAT_NV12 as u32);
  ///planar mode: Y + V/U interleaved (2 planes) (>= SDL 2.0.4)
  pub const NV21: Self = Self(fermium::SDL_PIXELFORMAT_NV21 as u32);
}
