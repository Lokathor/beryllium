use beryllium::*;

#[rustfmt::skip]
#[test]
pub fn test_compare_with_header_definitions() {
  fn basics_flags(format: PixelFormatEnum) -> ((PixelType, PixelOrder, PixelLayout, u32, u32), (bool, bool, bool, bool)) {
    (
      (format.pixel_type(),format.pixel_order(),format.pixel_layout(),format.bits_per_pixel(),format.bytes_per_pixel()),
      (format.is_indexed(),format.is_packed(),format.is_array(),format.is_fourcc())
    )
  }
  //
  let format = PixelFormatEnum::Unknown;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Unknown, PixelOrder::Bitmap(BitmapPixelOrder::None), PixelLayout::None, 0, 0));
  assert_eq!(flags, (false, false, false, false));
  //
  let format = PixelFormatEnum::Index1lsb;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Index1, PixelOrder::Bitmap(BitmapPixelOrder::_4321), PixelLayout::None, 1, 0));
  assert_eq!(flags, (true, false, false, false));
  //
  let format = PixelFormatEnum::Index1msb;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Index1, PixelOrder::Bitmap(BitmapPixelOrder::_1234), PixelLayout::None, 1, 0));
  assert_eq!(flags, (true, false, false, false));
  //
  let format = PixelFormatEnum::Index4lsb;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Index4, PixelOrder::Bitmap(BitmapPixelOrder::_4321), PixelLayout::None, 4, 0));
  assert_eq!(flags, (true, false, false, false));
  //
  let format = PixelFormatEnum::Index4msb;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Index4, PixelOrder::Bitmap(BitmapPixelOrder::_1234), PixelLayout::None, 4, 0));
  assert_eq!(flags, (true, false, false, false));
  //
  let format = PixelFormatEnum::Index8;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Index8, PixelOrder::Bitmap(BitmapPixelOrder::None), PixelLayout::None, 8, 1));
  assert_eq!(flags, (true, false, false, false));
  //
  let format = PixelFormatEnum::RGB332;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed8, PixelOrder::Packed(PackedPixelOrder::XRGB), PixelLayout::_332, 8, 1));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::RGB444;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed16, PixelOrder::Packed(PackedPixelOrder::XRGB), PixelLayout::_4444, 12, 2));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::RGB555;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed16, PixelOrder::Packed(PackedPixelOrder::XRGB), PixelLayout::_1555, 15, 2));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::BGR555;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed16, PixelOrder::Packed(PackedPixelOrder::XBGR), PixelLayout::_1555, 15, 2));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::ARGB4444;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed16, PixelOrder::Packed(PackedPixelOrder::ARGB), PixelLayout::_4444, 16, 2));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::RGBA4444;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed16, PixelOrder::Packed(PackedPixelOrder::RGBA), PixelLayout::_4444, 16, 2));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::ABGR4444;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed16, PixelOrder::Packed(PackedPixelOrder::ABGR), PixelLayout::_4444, 16, 2));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::BGRA4444;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed16, PixelOrder::Packed(PackedPixelOrder::BGRA), PixelLayout::_4444, 16, 2));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::ARGB1555;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed16, PixelOrder::Packed(PackedPixelOrder::ARGB), PixelLayout::_1555, 16, 2));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::RGBA5551;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed16, PixelOrder::Packed(PackedPixelOrder::RGBA), PixelLayout::_5551, 16, 2));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::ABGR1555;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed16, PixelOrder::Packed(PackedPixelOrder::ABGR), PixelLayout::_1555, 16, 2));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::BGRA5551;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed16, PixelOrder::Packed(PackedPixelOrder::BGRA), PixelLayout::_5551, 16, 2));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::RGB565;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed16, PixelOrder::Packed(PackedPixelOrder::XRGB), PixelLayout::_565, 16, 2));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::BGR565;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed16, PixelOrder::Packed(PackedPixelOrder::XBGR), PixelLayout::_565, 16, 2));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::RGB24;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::ArrayU8, PixelOrder::Array(ArrayPixelOrder::RGB), PixelLayout::None, 24, 3));
  assert_eq!(flags, (false, false, true, false));
  //
  let format = PixelFormatEnum::BGR24;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::ArrayU8, PixelOrder::Array(ArrayPixelOrder::BGR), PixelLayout::None, 24, 3));
  assert_eq!(flags, (false, false, true, false));
  //
  let format = PixelFormatEnum::RGB888;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed32, PixelOrder::Packed(PackedPixelOrder::XRGB), PixelLayout::_8888, 24, 4));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::RGBX8888;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed32, PixelOrder::Packed(PackedPixelOrder::RGBX), PixelLayout::_8888, 24, 4));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::BGR888;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed32, PixelOrder::Packed(PackedPixelOrder::XBGR), PixelLayout::_8888, 24, 4));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::BGRX8888;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed32, PixelOrder::Packed(PackedPixelOrder::BGRX), PixelLayout::_8888, 24, 4));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::ARGB8888;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed32, PixelOrder::Packed(PackedPixelOrder::ARGB), PixelLayout::_8888, 32, 4));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::RGBA8888;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed32, PixelOrder::Packed(PackedPixelOrder::RGBA), PixelLayout::_8888, 32, 4));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::ABGR8888;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed32, PixelOrder::Packed(PackedPixelOrder::ABGR), PixelLayout::_8888, 32, 4));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::BGRA8888;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed32, PixelOrder::Packed(PackedPixelOrder::BGRA), PixelLayout::_8888, 32, 4));
  assert_eq!(flags, (false, true, false, false));
  //
  let format = PixelFormatEnum::ARGB2101010;
  let (basics, flags) = basics_flags(format);
  assert_eq!(basics, (PixelType::Packed32, PixelOrder::Packed(PackedPixelOrder::ARGB), PixelLayout::_2101010, 32, 4));
  assert_eq!(flags, (false, true, false, false));
  
  // The fourcc pixel formats are standards based values that don't follow
  // SDL2's bit packing scheme, so they basically have nonsense as their
  // "basics" entries, however, the fourcc flag result must always be correct.
  assert!(PixelFormatEnum::YV12.is_fourcc());
  assert!(PixelFormatEnum::IYUV.is_fourcc());
  assert!(PixelFormatEnum::YUY2.is_fourcc());
  assert!(PixelFormatEnum::UYVY.is_fourcc());
  assert!(PixelFormatEnum::YVYU.is_fourcc());
  assert!(PixelFormatEnum::NV12.is_fourcc());
  assert!(PixelFormatEnum::NV21.is_fourcc());
  assert!(PixelFormatEnum::ExternalOES.is_fourcc());
}
