use super::*;

/// The various named pixel formats that SDL2 supports.
///
/// There's various checks you can perform on each pixel format.
///
/// Note that the "fourcc" formats, anything that gives `true` from the
/// [is_fourcc](PixelFormatEnum::is_fourcc) method, are industry specified special
/// values, and do not follow SDL2's bit packing scheme. In other words, the
/// output they produce for any of the other check methods is not to really be
/// trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum PixelFormatEnum {
  Unknown = SDL_PIXELFORMAT_UNKNOWN,
  Index1lsb = SDL_PIXELFORMAT_INDEX1LSB,
  Index1msb = SDL_PIXELFORMAT_INDEX1MSB,
  Index4lsb = SDL_PIXELFORMAT_INDEX4LSB,
  Index4msb = SDL_PIXELFORMAT_INDEX4MSB,
  Index8 = SDL_PIXELFORMAT_INDEX8,
  RGB332 = SDL_PIXELFORMAT_RGB332,
  RGB444 = SDL_PIXELFORMAT_RGB444,
  RGB555 = SDL_PIXELFORMAT_RGB555,
  BGR555 = SDL_PIXELFORMAT_BGR555,
  ARGB4444 = SDL_PIXELFORMAT_ARGB4444,
  RGBA4444 = SDL_PIXELFORMAT_RGBA4444,
  ABGR4444 = SDL_PIXELFORMAT_ABGR4444,
  BGRA4444 = SDL_PIXELFORMAT_BGRA4444,
  ARGB1555 = SDL_PIXELFORMAT_ARGB1555,
  RGBA5551 = SDL_PIXELFORMAT_RGBA5551,
  ABGR1555 = SDL_PIXELFORMAT_ABGR1555,
  BGRA5551 = SDL_PIXELFORMAT_BGRA5551,
  RGB565 = SDL_PIXELFORMAT_RGB565,
  BGR565 = SDL_PIXELFORMAT_BGR565,
  RGB24 = SDL_PIXELFORMAT_RGB24,
  BGR24 = SDL_PIXELFORMAT_BGR24,
  RGB888 = SDL_PIXELFORMAT_RGB888,
  RGBX8888 = SDL_PIXELFORMAT_RGBX8888,
  BGR888 = SDL_PIXELFORMAT_BGR888,
  BGRX8888 = SDL_PIXELFORMAT_BGRX8888,
  ARGB8888 = SDL_PIXELFORMAT_ARGB8888,
  RGBA8888 = SDL_PIXELFORMAT_RGBA8888,
  ABGR8888 = SDL_PIXELFORMAT_ABGR8888,
  BGRA8888 = SDL_PIXELFORMAT_BGRA8888,
  ARGB2101010 = SDL_PIXELFORMAT_ARGB2101010,
  /// Planar mode: Y + V + U (3 planes)
  YV12 = SDL_PIXELFORMAT_YV12,
  /// Planar mode: Y + U + V (3 planes)
  IYUV = SDL_PIXELFORMAT_IYUV,
  /// Packed mode: Y0+U0+Y1+V0 (1 plane)
  YUY2 = SDL_PIXELFORMAT_YUY2,
  /// Packed mode: U0+Y0+V0+Y1 (1 plane)
  UYVY = SDL_PIXELFORMAT_UYVY,
  /// Packed mode: Y0+V0+Y1+U0 (1 plane)
  YVYU = SDL_PIXELFORMAT_YVYU,
  /// Planar mode: Y + U/V interleaved (2 planes)
  NV12 = SDL_PIXELFORMAT_NV12,
  /// Planar mode: Y + V/U interleaved (2 planes)
  NV21 = SDL_PIXELFORMAT_NV21,
  /// Android video texture format
  ExternalOES = SDL_PIXELFORMAT_EXTERNAL_OES,
}
#[cfg(target_endian = "big")]
impl PixelFormatEnum {
  /// Platform specific alias for RGBA
  pub const RGBA32: Self = PixelFormatEnum::RGBA8888;
  /// Platform specific alias for ARGB
  pub const ARGB32: Self = PixelFormatEnum::ARGB8888;
  /// Platform specific alias for BGRA
  pub const BGRA32: Self = PixelFormatEnum::BGRA8888;
  /// Platform specific alias for ABGR
  pub const ABGR32: Self = PixelFormatEnum::ABGR8888;
}
#[cfg(target_endian = "little")]
impl PixelFormatEnum {
  /// Platform specific alias for RGBA
  pub const RGBA32: Self = PixelFormatEnum::ABGR8888;
  /// Platform specific alias for ARGB
  pub const ARGB32: Self = PixelFormatEnum::BGRA8888;
  /// Platform specific alias for BGRA
  pub const BGRA32: Self = PixelFormatEnum::ARGB8888;
  /// Platform specific alias for ABGR
  pub const ABGR32: Self = PixelFormatEnum::RGBA8888;
}
impl From<fermium::_bindgen_ty_6::Type> for PixelFormatEnum {
  fn from(pf: fermium::_bindgen_ty_6::Type) -> Self {
    match pf {
      SDL_PIXELFORMAT_INDEX1LSB => PixelFormatEnum::Index1lsb,
      SDL_PIXELFORMAT_INDEX1MSB => PixelFormatEnum::Index1msb,
      SDL_PIXELFORMAT_INDEX4LSB => PixelFormatEnum::Index4lsb,
      SDL_PIXELFORMAT_INDEX4MSB => PixelFormatEnum::Index4msb,
      SDL_PIXELFORMAT_INDEX8 => PixelFormatEnum::Index8,
      SDL_PIXELFORMAT_RGB332 => PixelFormatEnum::RGB332,
      SDL_PIXELFORMAT_RGB444 => PixelFormatEnum::RGB444,
      SDL_PIXELFORMAT_RGB555 => PixelFormatEnum::RGB555,
      SDL_PIXELFORMAT_BGR555 => PixelFormatEnum::BGR555,
      SDL_PIXELFORMAT_ARGB4444 => PixelFormatEnum::ARGB4444,
      SDL_PIXELFORMAT_RGBA4444 => PixelFormatEnum::RGBA4444,
      SDL_PIXELFORMAT_ABGR4444 => PixelFormatEnum::ABGR4444,
      SDL_PIXELFORMAT_BGRA4444 => PixelFormatEnum::BGRA4444,
      SDL_PIXELFORMAT_ARGB1555 => PixelFormatEnum::ARGB1555,
      SDL_PIXELFORMAT_RGBA5551 => PixelFormatEnum::RGBA5551,
      SDL_PIXELFORMAT_ABGR1555 => PixelFormatEnum::ABGR1555,
      SDL_PIXELFORMAT_BGRA5551 => PixelFormatEnum::BGRA5551,
      SDL_PIXELFORMAT_RGB565 => PixelFormatEnum::RGB565,
      SDL_PIXELFORMAT_BGR565 => PixelFormatEnum::BGR565,
      SDL_PIXELFORMAT_RGB24 => PixelFormatEnum::RGB24,
      SDL_PIXELFORMAT_BGR24 => PixelFormatEnum::BGR24,
      SDL_PIXELFORMAT_RGB888 => PixelFormatEnum::RGB888,
      SDL_PIXELFORMAT_RGBX8888 => PixelFormatEnum::RGBX8888,
      SDL_PIXELFORMAT_BGR888 => PixelFormatEnum::BGR888,
      SDL_PIXELFORMAT_BGRX8888 => PixelFormatEnum::BGRX8888,
      SDL_PIXELFORMAT_ARGB8888 => PixelFormatEnum::ARGB8888,
      SDL_PIXELFORMAT_RGBA8888 => PixelFormatEnum::RGBA8888,
      SDL_PIXELFORMAT_ABGR8888 => PixelFormatEnum::ABGR8888,
      SDL_PIXELFORMAT_BGRA8888 => PixelFormatEnum::BGRA8888,
      SDL_PIXELFORMAT_ARGB2101010 => PixelFormatEnum::ARGB2101010,
      SDL_PIXELFORMAT_YV12 => PixelFormatEnum::YV12,
      SDL_PIXELFORMAT_IYUV => PixelFormatEnum::IYUV,
      SDL_PIXELFORMAT_YUY2 => PixelFormatEnum::YUY2,
      SDL_PIXELFORMAT_UYVY => PixelFormatEnum::UYVY,
      SDL_PIXELFORMAT_YVYU => PixelFormatEnum::YVYU,
      SDL_PIXELFORMAT_NV12 => PixelFormatEnum::NV12,
      SDL_PIXELFORMAT_NV21 => PixelFormatEnum::NV21,
      SDL_PIXELFORMAT_EXTERNAL_OES => PixelFormatEnum::ExternalOES,
      _ => PixelFormatEnum::Unknown,
    }
  }
}
impl PixelFormatEnum {
  /// Gets a `PixelFormatEnum` from the bpp and mask values given.
  ///
  /// * `bpp` is bits per pixel, usually 15, 16, or 32
  /// * `?_mask` values are the `r`, `g`, `b`, and `a` mask values
  pub fn from_masks(bpp: i32, r_mask: u32, g_mask: u32, b_mask: u32, a_mask: u32) -> Self {
    PixelFormatEnum::from(
      unsafe { SDL_MasksToPixelFormatEnum(bpp, r_mask, g_mask, b_mask, a_mask) }
        as fermium::_bindgen_ty_6::Type,
    )
  }

  /// Converts this value into the appropriate `bpp` and mask values.
  pub fn to_masks(self) -> (i32, u32, u32, u32, u32) {
    let mut bpp = 0;
    let mut r_mask = 0;
    let mut g_mask = 0;
    let mut b_mask = 0;
    let mut a_mask = 0;
    unsafe {
      SDL_PixelFormatEnumToMasks(
        self as u32,
        &mut bpp,
        &mut r_mask,
        &mut g_mask,
        &mut b_mask,
        &mut a_mask,
      );
    }
    (bpp, r_mask, g_mask, b_mask, a_mask)
  }

  /// The type of the pixel format.
  ///
  /// All unknown types convert to `PixelType::Unknown`, of course.
  pub fn pixel_type(self) -> PixelType {
    match ((self as u32 >> 24) & 0x0F) as fermium::_bindgen_ty_1::Type {
      SDL_PIXELTYPE_INDEX1 => PixelType::Index1,
      SDL_PIXELTYPE_INDEX4 => PixelType::Index4,
      SDL_PIXELTYPE_INDEX8 => PixelType::Index8,
      SDL_PIXELTYPE_PACKED8 => PixelType::Packed8,
      SDL_PIXELTYPE_PACKED16 => PixelType::Packed16,
      SDL_PIXELTYPE_PACKED32 => PixelType::Packed32,
      SDL_PIXELTYPE_ARRAYU8 => PixelType::ArrayU8,
      SDL_PIXELTYPE_ARRAYU16 => PixelType::ArrayU16,
      SDL_PIXELTYPE_ARRAYU32 => PixelType::ArrayU32,
      SDL_PIXELTYPE_ARRAYF16 => PixelType::ArrayF16,
      SDL_PIXELTYPE_ARRAYF32 => PixelType::ArrayF32,
      _ => PixelType::Unknown,
    }
  }

  /// Ordering of channel or bits in the pixel format.
  ///
  /// Unknown values convert to one of the `None` variants.
  pub fn pixel_order(self) -> PixelOrder {
    let bits = (self as u32 >> 20) & 0x0F;
    if self.is_packed() {
      match bits as fermium::_bindgen_ty_4::Type {
        SDL_PACKEDORDER_ABGR => PixelOrder::Packed(PackedPixelOrder::ABGR),
        SDL_PACKEDORDER_ARGB => PixelOrder::Packed(PackedPixelOrder::ARGB),
        SDL_PACKEDORDER_BGRA => PixelOrder::Packed(PackedPixelOrder::BGRA),
        SDL_PACKEDORDER_BGRX => PixelOrder::Packed(PackedPixelOrder::BGRX),
        SDL_PACKEDORDER_RGBA => PixelOrder::Packed(PackedPixelOrder::RGBA),
        SDL_PACKEDORDER_RGBX => PixelOrder::Packed(PackedPixelOrder::RGBX),
        SDL_PACKEDORDER_XBGR => PixelOrder::Packed(PackedPixelOrder::XBGR),
        SDL_PACKEDORDER_XRGB => PixelOrder::Packed(PackedPixelOrder::XRGB),
        _ => PixelOrder::Packed(PackedPixelOrder::None),
      }
    } else if self.is_array() {
      match bits as fermium::_bindgen_ty_4::Type {
        SDL_ARRAYORDER_ABGR => PixelOrder::Array(ArrayPixelOrder::ABGR),
        SDL_ARRAYORDER_ARGB => PixelOrder::Array(ArrayPixelOrder::ARGB),
        SDL_ARRAYORDER_BGR => PixelOrder::Array(ArrayPixelOrder::BGR),
        SDL_ARRAYORDER_BGRA => PixelOrder::Array(ArrayPixelOrder::BGRA),
        SDL_ARRAYORDER_RGB => PixelOrder::Array(ArrayPixelOrder::RGB),
        SDL_ARRAYORDER_RGBA => PixelOrder::Array(ArrayPixelOrder::RGBA),
        _ => PixelOrder::Array(ArrayPixelOrder::None),
      }
    } else {
      match bits as fermium::_bindgen_ty_2::Type {
        SDL_BITMAPORDER_1234 => PixelOrder::Bitmap(BitmapPixelOrder::_1234),
        SDL_BITMAPORDER_4321 => PixelOrder::Bitmap(BitmapPixelOrder::_4321),
        _ => PixelOrder::Bitmap(BitmapPixelOrder::None),
      }
    }
  }

  /// Channel bits pattern of the pixel format.
  ///
  /// Converts any possible unknown layout to `PixelLayout::None`.
  pub fn pixel_layout(self) -> PixelLayout {
    match ((self as u32 >> 16) & 0x0F) as fermium::_bindgen_ty_1::Type {
      SDL_PACKEDLAYOUT_332 => PixelLayout::_332,
      SDL_PACKEDLAYOUT_4444 => PixelLayout::_4444,
      SDL_PACKEDLAYOUT_1555 => PixelLayout::_1555,
      SDL_PACKEDLAYOUT_5551 => PixelLayout::_5551,
      SDL_PACKEDLAYOUT_565 => PixelLayout::_565,
      SDL_PACKEDLAYOUT_8888 => PixelLayout::_8888,
      SDL_PACKEDLAYOUT_2101010 => PixelLayout::_2101010,
      SDL_PACKEDLAYOUT_1010102 => PixelLayout::_1010102,
      _ => PixelLayout::None,
    }
  }

  /// Bits of color information per pixel.
  pub fn bits_per_pixel(self) -> u32 {
    (self as u32 >> 8) & 0xFF
  }

  /// Bytes used per pixel.
  ///
  /// Note: Formats with less than 8 bits per pixel give a result of 0 bytes per
  /// pixel. Weird and all, but that's how it is.
  pub fn bytes_per_pixel(self) -> u32 {
    if self.is_fourcc() {
      match self {
        PixelFormatEnum::YUY2 | PixelFormatEnum::UYVY | PixelFormatEnum::YVYU => 2,
        _ => 1,
      }
    } else {
      self as u32 & 0xFF
    }
  }

  /// Is this format an indexed format?
  pub fn is_indexed(self) -> bool {
    !self.is_fourcc()
      && match self.pixel_type() {
        PixelType::Index1 | PixelType::Index4 | PixelType::Index8 => true,
        _ => false,
      }
  }

  /// Is this format a packed format?
  pub fn is_packed(self) -> bool {
    !self.is_fourcc()
      && match self.pixel_type() {
        PixelType::Packed8 | PixelType::Packed16 | PixelType::Packed32 => true,
        _ => false,
      }
  }

  /// Is this format a packed format?
  pub fn is_array(self) -> bool {
    !self.is_fourcc()
      && match self.pixel_type() {
        PixelType::ArrayU8
        | PixelType::ArrayU16
        | PixelType::ArrayU32
        | PixelType::ArrayF16
        | PixelType::ArrayF32 => true,
        _ => false,
      }
  }

  /// Is this format a format with an alpha channel?
  pub fn is_alpha(self) -> bool {
    match self.pixel_order() {
      PixelOrder::Packed(PackedPixelOrder::ARGB)
      | PixelOrder::Packed(PackedPixelOrder::RGBA)
      | PixelOrder::Packed(PackedPixelOrder::ABGR)
      | PixelOrder::Packed(PackedPixelOrder::BGRA)
      | PixelOrder::Array(ArrayPixelOrder::ARGB)
      | PixelOrder::Array(ArrayPixelOrder::RGBA)
      | PixelOrder::Array(ArrayPixelOrder::ABGR)
      | PixelOrder::Array(ArrayPixelOrder::BGRA) => true,
      _ => false,
    }
  }

  /// Is this a [four-character code](https://en.wikipedia.org/wiki/FourCC) format?
  ///
  /// True for pixel formats representing unique formats, for example YUV formats.
  pub fn is_fourcc(self) -> bool {
    (self as u32 > 0) && (((self as u32 >> 28) & 0x0F) != 1)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum PixelType {
  Unknown = SDL_PIXELTYPE_UNKNOWN,
  Index1 = SDL_PIXELTYPE_INDEX1,
  Index4 = SDL_PIXELTYPE_INDEX4,
  Index8 = SDL_PIXELTYPE_INDEX8,
  Packed8 = SDL_PIXELTYPE_PACKED8,
  Packed16 = SDL_PIXELTYPE_PACKED16,
  Packed32 = SDL_PIXELTYPE_PACKED32,
  ArrayU8 = SDL_PIXELTYPE_ARRAYU8,
  ArrayU16 = SDL_PIXELTYPE_ARRAYU16,
  ArrayU32 = SDL_PIXELTYPE_ARRAYU32,
  ArrayF16 = SDL_PIXELTYPE_ARRAYF16,
  ArrayF32 = SDL_PIXELTYPE_ARRAYF32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum PixelOrder {
  Bitmap(BitmapPixelOrder),
  Packed(PackedPixelOrder),
  Array(ArrayPixelOrder),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum BitmapPixelOrder {
  None = SDL_BITMAPORDER_NONE,
  _4321 = SDL_BITMAPORDER_4321,
  _1234 = SDL_BITMAPORDER_1234,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum PackedPixelOrder {
  None = SDL_PACKEDORDER_NONE,
  XRGB = SDL_PACKEDORDER_XRGB,
  RGBX = SDL_PACKEDORDER_RGBX,
  ARGB = SDL_PACKEDORDER_ARGB,
  RGBA = SDL_PACKEDORDER_RGBA,
  XBGR = SDL_PACKEDORDER_XBGR,
  BGRX = SDL_PACKEDORDER_BGRX,
  ABGR = SDL_PACKEDORDER_ABGR,
  BGRA = SDL_PACKEDORDER_BGRA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum ArrayPixelOrder {
  None = SDL_ARRAYORDER_NONE,
  RGB = SDL_ARRAYORDER_RGB,
  RGBA = SDL_ARRAYORDER_RGBA,
  ARGB = SDL_ARRAYORDER_ARGB,
  BGR = SDL_ARRAYORDER_BGR,
  BGRA = SDL_ARRAYORDER_BGRA,
  ABGR = SDL_ARRAYORDER_ABGR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum PixelLayout {
  None = SDL_PACKEDLAYOUT_NONE,
  _332 = SDL_PACKEDLAYOUT_332,
  _4444 = SDL_PACKEDLAYOUT_4444,
  _1555 = SDL_PACKEDLAYOUT_1555,
  _5551 = SDL_PACKEDLAYOUT_5551,
  _565 = SDL_PACKEDLAYOUT_565,
  _8888 = SDL_PACKEDLAYOUT_8888,
  _2101010 = SDL_PACKEDLAYOUT_2101010,
  _1010102 = SDL_PACKEDLAYOUT_1010102,
}

/// Data about a particular pixel layout.
///
/// A `PixelFormat` value is a handle to a heap allocated `SDL_PixelFormat`.
/// That `SDL_PixelFormat` then has all sorts of info, including a reference
/// count value. So you can think of it as being _similar to_ `*mut
/// Rc<PixelFormatData>` or something like that.
///
/// A `PixelFormat` is for either a paletted format or masks format. The mask
/// values of a paletted format will all be 0.
#[derive(Debug)]
#[repr(transparent)]
pub struct PixelFormat<'sdl> {
  pub(crate) ptr: *mut SDL_PixelFormat,
  pub(crate) _marker: PhantomData<&'sdl SDLToken>,
}
impl<'sdl> Clone for PixelFormat<'sdl> {
  fn clone(&self) -> Self {
    let ptr = unsafe { SDL_AllocFormat((*self.ptr).format) };
    if ptr.is_null() {
      panic!("Couldn't allocate a new PixelFormat!")
    } else {
      Self {
        ptr,
        _marker: PhantomData,
      }
      // TODO: also copy over palette data?
    }
  }
}
impl<'sdl> Drop for PixelFormat<'sdl> {
  fn drop(&mut self) {
    unsafe { SDL_FreeFormat(self.ptr) }
  }
}
impl SDLToken {
  /// Allocates a new `PixelFormat` value.
  pub fn new_pixel_format<'sdl>(
    &'sdl self, format: PixelFormatEnum,
  ) -> Result<PixelFormat<'sdl>, String> {
    let ptr = unsafe { SDL_AllocFormat(format as u32) };
    if ptr.is_null() {
      Err(get_error())
    } else {
      Ok(PixelFormat {
        ptr,
        _marker: PhantomData,
      })
    }
  }
}

impl<'sdl> PixelFormat<'sdl> {
  /// Gets the RGB [Color](Color) components of a pixel value in this format.
  ///
  /// The alpha channel is always given as `0xFF`
  pub fn get_rgb(&self, pixel: u32) -> Color {
    let mut out = Color {
      r: 0,
      g: 0,
      b: 0,
      a: 0xFF,
    };
    unsafe { SDL_GetRGB(pixel, self.ptr, &mut out.r, &mut out.g, &mut out.b) };
    out
  }

  /// Gets the RGBA [Color](Color) components of a pixel value in this format.
  ///
  /// The alpha channel is always given as `0xFF` if the format has no alpha
  /// channel.
  pub fn get_rgba(&self, pixel: u32) -> Color {
    let mut out = Color::default();
    unsafe {
      SDL_GetRGBA(
        pixel, self.ptr, &mut out.r, &mut out.g, &mut out.b, &mut out.a,
      )
    };
    out
  }

  /// Maps a [Color](Color) value into an RGB pixel value in this format.
  ///
  /// * If the format is paletted the closest index is returned.
  /// * If the format supports alpha it will be a fully opaque pixel.
  /// * The pixel format data is always in the lowest bits, so you can safely
  ///   downcast pixel values to `u16` and `u8` as appropriate.
  pub fn map_rgb(&self, color: Color) -> u32 {
    unsafe { SDL_MapRGB(self.ptr, color.r, color.g, color.b) }
  }

  /// Maps a [Color](Color) value into an RGBA pixel value in this format.
  ///
  /// * If the format is paletted the closest index is returned.
  /// * If the format has no alpha channel or is paletted then the input alpha
  ///   value is simply ignored.
  /// * The pixel format data is always in the lowest bits, so you can safely
  ///   downcast pixel values to `u16` and `u8` as appropriate.
  pub fn map_rgba(&self, color: Color) -> u32 {
    unsafe { SDL_MapRGBA(self.ptr, color.r, color.g, color.b, color.a) }
  }

  /// Reassigns the [Palette](Palette) for this `PixelFormat`
  pub fn set_palette(&mut self, palette: &Palette) -> Result<(), String> {
    let out = unsafe { SDL_SetPixelFormatPalette(self.ptr, palette.nn.as_ptr()) };
    if out == 0 {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  /// The enum value of this pixel format.
  pub fn format(&self) -> PixelFormatEnum {
    PixelFormatEnum::from(unsafe { (*self.ptr).format } as fermium::_bindgen_ty_6::Type)
  }

  /// Significant bits in a pixel value: probably 8, 15, 16, 24, or 32.
  pub fn bits_per_pixel(&self) -> u8 {
    unsafe { (*self.ptr).BitsPerPixel }
  }

  /// The bytes required to hold a pixel value: probably 1, 2, 3, or 4.
  pub fn bytes_per_pixel(&self) -> u8 {
    unsafe { (*self.ptr).BytesPerPixel }
  }

  /// Mask for the location of the red component within a pixel value.
  pub fn r_mask(&self) -> u32 {
    unsafe { (*self.ptr).Rmask }
  }
  /// Mask for the location of the green component within a pixel value.
  pub fn g_mask(&self) -> u32 {
    unsafe { (*self.ptr).Gmask }
  }

  /// Mask for the location of the blue component within a pixel value.
  pub fn b_mask(&self) -> u32 {
    unsafe { (*self.ptr).Bmask }
  }

  /// Mask for the location of the alpha component within a pixel value.
  pub fn a_mask(&self) -> u32 {
    unsafe { (*self.ptr).Amask }
  }
}
