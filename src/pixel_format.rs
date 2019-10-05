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
  Unknown = fermium::SDL_PIXELFORMAT_UNKNOWN,
  Index1lsb = fermium::SDL_PIXELFORMAT_INDEX1LSB,
  Index1msb = fermium::SDL_PIXELFORMAT_INDEX1MSB,
  Index4lsb = fermium::SDL_PIXELFORMAT_INDEX4LSB,
  Index4msb = fermium::SDL_PIXELFORMAT_INDEX4MSB,
  Index8 = fermium::SDL_PIXELFORMAT_INDEX8,
  RGB332 = fermium::SDL_PIXELFORMAT_RGB332,
  RGB444 = fermium::SDL_PIXELFORMAT_RGB444,
  RGB555 = fermium::SDL_PIXELFORMAT_RGB555,
  BGR555 = fermium::SDL_PIXELFORMAT_BGR555,
  ARGB4444 = fermium::SDL_PIXELFORMAT_ARGB4444,
  RGBA4444 = fermium::SDL_PIXELFORMAT_RGBA4444,
  ABGR4444 = fermium::SDL_PIXELFORMAT_ABGR4444,
  BGRA4444 = fermium::SDL_PIXELFORMAT_BGRA4444,
  ARGB1555 = fermium::SDL_PIXELFORMAT_ARGB1555,
  RGBA5551 = fermium::SDL_PIXELFORMAT_RGBA5551,
  ABGR1555 = fermium::SDL_PIXELFORMAT_ABGR1555,
  BGRA5551 = fermium::SDL_PIXELFORMAT_BGRA5551,
  RGB565 = fermium::SDL_PIXELFORMAT_RGB565,
  BGR565 = fermium::SDL_PIXELFORMAT_BGR565,
  RGB24 = fermium::SDL_PIXELFORMAT_RGB24,
  BGR24 = fermium::SDL_PIXELFORMAT_BGR24,
  RGB888 = fermium::SDL_PIXELFORMAT_RGB888,
  RGBX8888 = fermium::SDL_PIXELFORMAT_RGBX8888,
  BGR888 = fermium::SDL_PIXELFORMAT_BGR888,
  BGRX8888 = fermium::SDL_PIXELFORMAT_BGRX8888,
  ARGB8888 = fermium::SDL_PIXELFORMAT_ARGB8888,
  RGBA8888 = fermium::SDL_PIXELFORMAT_RGBA8888,
  ABGR8888 = fermium::SDL_PIXELFORMAT_ABGR8888,
  BGRA8888 = fermium::SDL_PIXELFORMAT_BGRA8888,
  ARGB2101010 = fermium::SDL_PIXELFORMAT_ARGB2101010,
  /// Planar mode: Y + V + U (3 planes)
  YV12 = fermium::SDL_PIXELFORMAT_YV12,
  /// Planar mode: Y + U + V (3 planes)
  IYUV = fermium::SDL_PIXELFORMAT_IYUV,
  /// Packed mode: Y0+U0+Y1+V0 (1 plane)
  YUY2 = fermium::SDL_PIXELFORMAT_YUY2,
  /// Packed mode: U0+Y0+V0+Y1 (1 plane)
  UYVY = fermium::SDL_PIXELFORMAT_UYVY,
  /// Packed mode: Y0+V0+Y1+U0 (1 plane)
  YVYU = fermium::SDL_PIXELFORMAT_YVYU,
  /// Planar mode: Y + U/V interleaved (2 planes)
  NV12 = fermium::SDL_PIXELFORMAT_NV12,
  /// Planar mode: Y + V/U interleaved (2 planes)
  NV21 = fermium::SDL_PIXELFORMAT_NV21,
  /// Android video texture format
  ExternalOES = fermium::SDL_PIXELFORMAT_EXTERNAL_OES,
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
impl From<fermium::SDL_PixelFormatEnum> for PixelFormatEnum {
  fn from(pf: fermium::SDL_PixelFormatEnum) -> Self {
    match pf {
      fermium::SDL_PIXELFORMAT_INDEX1LSB => PixelFormatEnum::Index1lsb,
      fermium::SDL_PIXELFORMAT_INDEX1MSB => PixelFormatEnum::Index1msb,
      fermium::SDL_PIXELFORMAT_INDEX4LSB => PixelFormatEnum::Index4lsb,
      fermium::SDL_PIXELFORMAT_INDEX4MSB => PixelFormatEnum::Index4msb,
      fermium::SDL_PIXELFORMAT_INDEX8 => PixelFormatEnum::Index8,
      fermium::SDL_PIXELFORMAT_RGB332 => PixelFormatEnum::RGB332,
      fermium::SDL_PIXELFORMAT_RGB444 => PixelFormatEnum::RGB444,
      fermium::SDL_PIXELFORMAT_RGB555 => PixelFormatEnum::RGB555,
      fermium::SDL_PIXELFORMAT_BGR555 => PixelFormatEnum::BGR555,
      fermium::SDL_PIXELFORMAT_ARGB4444 => PixelFormatEnum::ARGB4444,
      fermium::SDL_PIXELFORMAT_RGBA4444 => PixelFormatEnum::RGBA4444,
      fermium::SDL_PIXELFORMAT_ABGR4444 => PixelFormatEnum::ABGR4444,
      fermium::SDL_PIXELFORMAT_BGRA4444 => PixelFormatEnum::BGRA4444,
      fermium::SDL_PIXELFORMAT_ARGB1555 => PixelFormatEnum::ARGB1555,
      fermium::SDL_PIXELFORMAT_RGBA5551 => PixelFormatEnum::RGBA5551,
      fermium::SDL_PIXELFORMAT_ABGR1555 => PixelFormatEnum::ABGR1555,
      fermium::SDL_PIXELFORMAT_BGRA5551 => PixelFormatEnum::BGRA5551,
      fermium::SDL_PIXELFORMAT_RGB565 => PixelFormatEnum::RGB565,
      fermium::SDL_PIXELFORMAT_BGR565 => PixelFormatEnum::BGR565,
      fermium::SDL_PIXELFORMAT_RGB24 => PixelFormatEnum::RGB24,
      fermium::SDL_PIXELFORMAT_BGR24 => PixelFormatEnum::BGR24,
      fermium::SDL_PIXELFORMAT_RGB888 => PixelFormatEnum::RGB888,
      fermium::SDL_PIXELFORMAT_RGBX8888 => PixelFormatEnum::RGBX8888,
      fermium::SDL_PIXELFORMAT_BGR888 => PixelFormatEnum::BGR888,
      fermium::SDL_PIXELFORMAT_BGRX8888 => PixelFormatEnum::BGRX8888,
      fermium::SDL_PIXELFORMAT_ARGB8888 => PixelFormatEnum::ARGB8888,
      fermium::SDL_PIXELFORMAT_RGBA8888 => PixelFormatEnum::RGBA8888,
      fermium::SDL_PIXELFORMAT_ABGR8888 => PixelFormatEnum::ABGR8888,
      fermium::SDL_PIXELFORMAT_BGRA8888 => PixelFormatEnum::BGRA8888,
      fermium::SDL_PIXELFORMAT_ARGB2101010 => PixelFormatEnum::ARGB2101010,
      fermium::SDL_PIXELFORMAT_YV12 => PixelFormatEnum::YV12,
      fermium::SDL_PIXELFORMAT_IYUV => PixelFormatEnum::IYUV,
      fermium::SDL_PIXELFORMAT_YUY2 => PixelFormatEnum::YUY2,
      fermium::SDL_PIXELFORMAT_UYVY => PixelFormatEnum::UYVY,
      fermium::SDL_PIXELFORMAT_YVYU => PixelFormatEnum::YVYU,
      fermium::SDL_PIXELFORMAT_NV12 => PixelFormatEnum::NV12,
      fermium::SDL_PIXELFORMAT_NV21 => PixelFormatEnum::NV21,
      fermium::SDL_PIXELFORMAT_EXTERNAL_OES => PixelFormatEnum::ExternalOES,
      _ => PixelFormatEnum::Unknown,
    }
  }
}
impl PixelFormatEnum {
  /// The type of the pixel format.
  ///
  /// All unknown types convert to `BerylliumPixelType::Unknown`, of course.
  pub fn pixel_type(self) -> BerylliumPixelType {
    match ((self as u32 >> 24) & 0x0F) as fermium::PixelType {
      fermium::SDL_PIXELTYPE_INDEX1 => BerylliumPixelType::Index1,
      fermium::SDL_PIXELTYPE_INDEX4 => BerylliumPixelType::Index4,
      fermium::SDL_PIXELTYPE_INDEX8 => BerylliumPixelType::Index8,
      fermium::SDL_PIXELTYPE_PACKED8 => BerylliumPixelType::Packed8,
      fermium::SDL_PIXELTYPE_PACKED16 => BerylliumPixelType::Packed16,
      fermium::SDL_PIXELTYPE_PACKED32 => BerylliumPixelType::Packed32,
      fermium::SDL_PIXELTYPE_ARRAYU8 => BerylliumPixelType::ArrayU8,
      fermium::SDL_PIXELTYPE_ARRAYU16 => BerylliumPixelType::ArrayU16,
      fermium::SDL_PIXELTYPE_ARRAYU32 => BerylliumPixelType::ArrayU32,
      fermium::SDL_PIXELTYPE_ARRAYF16 => BerylliumPixelType::ArrayF16,
      fermium::SDL_PIXELTYPE_ARRAYF32 => BerylliumPixelType::ArrayF32,
      _ => BerylliumPixelType::Unknown,
    }
  }

  /// Ordering of channel or bits in the pixel format.
  ///
  /// Unknown values convert to one of the `None` variants.
  pub fn pixel_order(self) -> PixelOrder {
    let bits = (self as u32 >> 20) & 0x0F;
    if self.is_packed() {
      match bits as fermium::PackedOrder {
        fermium::SDL_PACKEDORDER_ABGR => PixelOrder::Packed(PackedPixelOrder::ABGR),
        fermium::SDL_PACKEDORDER_ARGB => PixelOrder::Packed(PackedPixelOrder::ARGB),
        fermium::SDL_PACKEDORDER_BGRA => PixelOrder::Packed(PackedPixelOrder::BGRA),
        fermium::SDL_PACKEDORDER_BGRX => PixelOrder::Packed(PackedPixelOrder::BGRX),
        fermium::SDL_PACKEDORDER_RGBA => PixelOrder::Packed(PackedPixelOrder::RGBA),
        fermium::SDL_PACKEDORDER_RGBX => PixelOrder::Packed(PackedPixelOrder::RGBX),
        fermium::SDL_PACKEDORDER_XBGR => PixelOrder::Packed(PackedPixelOrder::XBGR),
        fermium::SDL_PACKEDORDER_XRGB => PixelOrder::Packed(PackedPixelOrder::XRGB),
        _ => PixelOrder::Packed(PackedPixelOrder::None),
      }
    } else if self.is_array() {
      match bits as fermium::ArrayOrder {
        fermium::SDL_ARRAYORDER_ABGR => PixelOrder::Array(ArrayPixelOrder::ABGR),
        fermium::SDL_ARRAYORDER_ARGB => PixelOrder::Array(ArrayPixelOrder::ARGB),
        fermium::SDL_ARRAYORDER_BGR => PixelOrder::Array(ArrayPixelOrder::BGR),
        fermium::SDL_ARRAYORDER_BGRA => PixelOrder::Array(ArrayPixelOrder::BGRA),
        fermium::SDL_ARRAYORDER_RGB => PixelOrder::Array(ArrayPixelOrder::RGB),
        fermium::SDL_ARRAYORDER_RGBA => PixelOrder::Array(ArrayPixelOrder::RGBA),
        _ => PixelOrder::Array(ArrayPixelOrder::None),
      }
    } else {
      match bits as fermium::BitmapOrder {
        fermium::SDL_BITMAPORDER_1234 => PixelOrder::Bitmap(BitmapPixelOrder::_1234),
        fermium::SDL_BITMAPORDER_4321 => PixelOrder::Bitmap(BitmapPixelOrder::_4321),
        _ => PixelOrder::Bitmap(BitmapPixelOrder::None),
      }
    }
  }

  /// Channel bits pattern of the pixel format.
  ///
  /// Converts any possible unknown layout to `PixelLayout::None`.
  pub fn pixel_layout(self) -> PixelLayout {
    match ((self as u32 >> 16) & 0x0F) as fermium::PixelType {
      fermium::SDL_PACKEDLAYOUT_332 => PixelLayout::_332,
      fermium::SDL_PACKEDLAYOUT_4444 => PixelLayout::_4444,
      fermium::SDL_PACKEDLAYOUT_1555 => PixelLayout::_1555,
      fermium::SDL_PACKEDLAYOUT_5551 => PixelLayout::_5551,
      fermium::SDL_PACKEDLAYOUT_565 => PixelLayout::_565,
      fermium::SDL_PACKEDLAYOUT_8888 => PixelLayout::_8888,
      fermium::SDL_PACKEDLAYOUT_2101010 => PixelLayout::_2101010,
      fermium::SDL_PACKEDLAYOUT_1010102 => PixelLayout::_1010102,
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
        BerylliumPixelType::Index1 | BerylliumPixelType::Index4 | BerylliumPixelType::Index8 => true,
        _ => false,
      }
  }

  /// Is this format a packed format?
  pub fn is_packed(self) -> bool {
    !self.is_fourcc()
      && match self.pixel_type() {
        BerylliumPixelType::Packed8 | BerylliumPixelType::Packed16 | BerylliumPixelType::Packed32 => true,
        _ => false,
      }
  }

  /// Is this format a packed format?
  pub fn is_array(self) -> bool {
    !self.is_fourcc()
      && match self.pixel_type() {
        BerylliumPixelType::ArrayU8
        | BerylliumPixelType::ArrayU16
        | BerylliumPixelType::ArrayU32
        | BerylliumPixelType::ArrayF16
        | BerylliumPixelType::ArrayF32 => true,
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

/// The name is weird because i goofed up `fermium-0.1`.
/// 
/// This will be fixed in 0.2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum BerylliumPixelType {
  Unknown = fermium::SDL_PIXELTYPE_UNKNOWN,
  Index1 = fermium::SDL_PIXELTYPE_INDEX1,
  Index4 = fermium::SDL_PIXELTYPE_INDEX4,
  Index8 = fermium::SDL_PIXELTYPE_INDEX8,
  Packed8 = fermium::SDL_PIXELTYPE_PACKED8,
  Packed16 = fermium::SDL_PIXELTYPE_PACKED16,
  Packed32 = fermium::SDL_PIXELTYPE_PACKED32,
  ArrayU8 = fermium::SDL_PIXELTYPE_ARRAYU8,
  ArrayU16 = fermium::SDL_PIXELTYPE_ARRAYU16,
  ArrayU32 = fermium::SDL_PIXELTYPE_ARRAYU32,
  ArrayF16 = fermium::SDL_PIXELTYPE_ARRAYF16,
  ArrayF32 = fermium::SDL_PIXELTYPE_ARRAYF32,
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
  None = fermium::SDL_BITMAPORDER_NONE,
  _4321 = fermium::SDL_BITMAPORDER_4321,
  _1234 = fermium::SDL_BITMAPORDER_1234,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum PackedPixelOrder {
  None = fermium::SDL_PACKEDORDER_NONE,
  XRGB = fermium::SDL_PACKEDORDER_XRGB,
  RGBX = fermium::SDL_PACKEDORDER_RGBX,
  ARGB = fermium::SDL_PACKEDORDER_ARGB,
  RGBA = fermium::SDL_PACKEDORDER_RGBA,
  XBGR = fermium::SDL_PACKEDORDER_XBGR,
  BGRX = fermium::SDL_PACKEDORDER_BGRX,
  ABGR = fermium::SDL_PACKEDORDER_ABGR,
  BGRA = fermium::SDL_PACKEDORDER_BGRA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum ArrayPixelOrder {
  None = fermium::SDL_ARRAYORDER_NONE,
  RGB = fermium::SDL_ARRAYORDER_RGB,
  RGBA = fermium::SDL_ARRAYORDER_RGBA,
  ARGB = fermium::SDL_ARRAYORDER_ARGB,
  BGR = fermium::SDL_ARRAYORDER_BGR,
  BGRA = fermium::SDL_ARRAYORDER_BGRA,
  ABGR = fermium::SDL_ARRAYORDER_ABGR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
#[allow(missing_docs)]
pub enum PixelLayout {
  None = fermium::SDL_PACKEDLAYOUT_NONE,
  _332 = fermium::SDL_PACKEDLAYOUT_332,
  _4444 = fermium::SDL_PACKEDLAYOUT_4444,
  _1555 = fermium::SDL_PACKEDLAYOUT_1555,
  _5551 = fermium::SDL_PACKEDLAYOUT_5551,
  _565 = fermium::SDL_PACKEDLAYOUT_565,
  _8888 = fermium::SDL_PACKEDLAYOUT_8888,
  _2101010 = fermium::SDL_PACKEDLAYOUT_2101010,
  _1010102 = fermium::SDL_PACKEDLAYOUT_1010102,
}

/// A handle to the information about a particular pixel layout.
///
/// This type works similar to the [Palette](Palette) type, where many images
/// can share a single `PixelFormat` value and changes to the `PixelFormat` will
/// show up in all places. Thankfully, the only changes you can make to a
/// `PixelFormat` is changing the palette used.
///
/// Every `PixelFormat` is either "paletted" or not. If the `PixelFormat` is
/// paletted then allocating the format value also allocates a `Palette` of the
/// appropriate length. That length is either 32 (4 bits per pixel index values)
/// or 256 (8 bits per pixel index values). Formats with a bits per pixel values
/// of more than 8 don't use a palette.
#[derive(Debug)]
#[repr(transparent)]
pub struct PixelFormat<'sdl> {
  pub(crate) nn: NonNull<fermium::SDL_PixelFormat>,
  pub(crate) _marker: PhantomData<&'sdl SDLToken>,
}

impl SDLToken {
  /// Allocates a new `PixelFormat` according to the enum given.
  pub fn new_pixel_format(&self, format: PixelFormatEnum) -> Result<PixelFormat<'_>, String> {
    match NonNull::new(unsafe { fermium::SDL_AllocFormat(format as u32) }) {
      Some(nn) => Ok(PixelFormat {
        nn,
        _marker: PhantomData,
      }),
      None => Err(get_error()),
    }
  }
}

impl Drop for PixelFormat<'_> {
  fn drop(&mut self) {
    unsafe { fermium::SDL_FreeFormat(self.nn.as_ptr()) }
  }
}

impl SDLToken {
  /// Gets SDL2's textual name of a `PixelFormatEnum`.
  ///
  /// Honestly, you can probably use the `Debug` impl of that type instead, but
  /// it's here if you want to double check SDL2's opinion of things.
  pub fn get_pixel_format_name(&self, format: PixelFormatEnum) -> String {
    unsafe {
      let ptr: *const c_char = fermium::SDL_GetPixelFormatName(format as u32);
      let len = fermium::SDL_strlen(ptr);
      let useful_bytes = from_raw_parts(ptr as *const u8, len);
      String::from_utf8_lossy(useful_bytes).into_owned()
    }
  }

  /// Try to combine some `bpp` and mask values into a single `format` value.
  pub fn masks_to_pixel_format_enum(
    &self,
    bpp: i32,
    r_mask: u32,
    g_mask: u32,
    b_mask: u32,
    a_mask: u32,
  ) -> PixelFormatEnum {
    PixelFormatEnum::from(
      unsafe { fermium::SDL_MasksToPixelFormatEnum(bpp, r_mask, g_mask, b_mask, a_mask) }
        as fermium::SDL_PixelFormatEnum,
    )
  }

  /// Converts this `format` into the appropriate `bpp` and mask values.
  pub fn pixel_format_enum_to_masks(&self, format: PixelFormatEnum) -> (i32, u32, u32, u32, u32) {
    let mut bpp = 0;
    let mut r_mask = 0;
    let mut g_mask = 0;
    let mut b_mask = 0;
    let mut a_mask = 0;
    unsafe {
      fermium::SDL_PixelFormatEnumToMasks(
        format as u32,
        &mut bpp,
        &mut r_mask,
        &mut g_mask,
        &mut b_mask,
        &mut a_mask,
      );
    }
    (bpp, r_mask, g_mask, b_mask, a_mask)
  }
}

impl PixelFormat<'_> {
  /// Gets the RGB [Color] components of a pixel value in this format.
  ///
  /// * The alpha channel is always given as `0xFF`
  pub fn get_rgb(&self, pixel: u32) -> Color {
    let mut out = Color {
      r: 0,
      g: 0,
      b: 0,
      a: 0xFF,
    };
    unsafe { fermium::SDL_GetRGB(pixel, self.nn.as_ptr(), &mut out.r, &mut out.g, &mut out.b) };
    out
  }

  /// Gets the RGBA [Color] components of a pixel value in this format.
  ///
  /// * The alpha channel is always given as `0xFF` if the format has no alpha
  ///   channel.
  pub fn get_rgba(&self, pixel: u32) -> Color {
    let mut out = Color::default();
    unsafe {
      fermium::SDL_GetRGBA(
        pixel,
        self.nn.as_ptr(),
        &mut out.r,
        &mut out.g,
        &mut out.b,
        &mut out.a,
      )
    };
    out
  }

  /// Maps a [Color] value into an RGB pixel value in this format.
  ///
  /// * If the format is paletted the closest index is returned.
  /// * If the format supports alpha it will be a fully opaque pixel.
  /// * The pixel format data is always in the lowest bits, so you can safely
  ///   downcast pixel values to `u16` and `u8` as appropriate.
  pub fn map_rgb(&self, color: Color) -> u32 {
    unsafe { fermium::SDL_MapRGB(self.nn.as_ptr(), color.r, color.g, color.b) }
  }

  /// Maps a [Color] value into an RGBA pixel value in this format.
  ///
  /// * If the format is paletted the closest index is returned.
  /// * If the format has no alpha channel or is paletted then the input alpha
  ///   value is simply ignored.
  /// * The pixel format data is always in the lowest bits, so you can safely
  ///   downcast pixel values to `u16` and `u8` as appropriate.
  pub fn map_rgba(&self, color: Color) -> u32 {
    unsafe { fermium::SDL_MapRGBA(self.nn.as_ptr(), color.r, color.g, color.b, color.a) }
  }

  /// Reassigns the [Palette] for this `PixelFormat`
  pub fn set_palette(&mut self, palette: &Palette) -> Result<(), String> {
    // Note(Lokathor): This must take `&mut self` to ensure you don't have an
    // active reference to the palette.
    let out = unsafe { fermium::SDL_SetPixelFormatPalette(self.nn.as_ptr(), palette.nn.as_ptr()) };
    if out == 0 {
      Ok(())
    } else {
      Err(get_error())
    }
  }

  /// The enum value of this pixel format.
  pub fn format(&self) -> PixelFormatEnum {
    PixelFormatEnum::from(unsafe { (*self.nn.as_ptr()).format } as fermium::SDL_PixelFormatEnum)
  }

  /// Obtains the palette of this format, if any.
  pub fn palette(&self) -> Option<&Palette> {
    unsafe {
      match NonNull::new((*self.nn.as_ptr()).palette) {
        // Note(Lokathor): Hey can't you use map here? Naw, the lifetimes get weird.
        Some(nn) => Some(
          &*(&nn as *const std::ptr::NonNull<fermium::SDL_Palette> as *const palette::Palette<'_>),
        ),
        None => None,
      }
    }
  }

  /// Significant bits in a pixel value: probably 8, 15, 16, 24, or 32.
  pub fn bits_per_pixel(&self) -> u8 {
    unsafe { (*self.nn.as_ptr()).BitsPerPixel }
  }

  /// The bytes required to hold a pixel value: probably 1, 2, 3, or 4.
  pub fn bytes_per_pixel(&self) -> u8 {
    unsafe { (*self.nn.as_ptr()).BytesPerPixel }
  }

  /// Mask for the location of the red component within a pixel value.
  pub fn r_mask(&self) -> u32 {
    unsafe { (*self.nn.as_ptr()).Rmask }
  }
  /// Mask for the location of the green component within a pixel value.
  pub fn g_mask(&self) -> u32 {
    unsafe { (*self.nn.as_ptr()).Gmask }
  }

  /// Mask for the location of the blue component within a pixel value.
  pub fn b_mask(&self) -> u32 {
    unsafe { (*self.nn.as_ptr()).Bmask }
  }

  /// Mask for the location of the alpha component within a pixel value.
  pub fn a_mask(&self) -> u32 {
    unsafe { (*self.nn.as_ptr()).Amask }
  }
}
