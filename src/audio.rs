
use super::*;

mod queue;
pub use queue::*;

/// A per-sample data format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AudioFormat(pub(crate) u16);
impl AudioFormat {
  /// `i8`
  pub const I8: AudioFormat = AudioFormat(fermium::AUDIO_S8 as u16);
  /// `u8`
  pub const U8: AudioFormat = AudioFormat(fermium::AUDIO_U8 as u16);
  
  /// `i16` little-endian
  pub const I16_LE: AudioFormat = AudioFormat(fermium::AUDIO_S16LSB as u16);
  /// `i16` big-endian
  pub const I16_BE: AudioFormat = AudioFormat(fermium::AUDIO_S16MSB as u16);
  /// `i16` system-endian
  pub const I16_SYS: AudioFormat = AudioFormat(fermium::AUDIO_S16SYS as u16);
  
  /// `u16` little-endian
  pub const U16_LE: AudioFormat = AudioFormat(fermium::AUDIO_U16LSB as u16);
  /// `u16` big-endian
  pub const U16_BE: AudioFormat = AudioFormat(fermium::AUDIO_U16MSB as u16);
  /// `u16` system-endian
  pub const U16_SYS: AudioFormat = AudioFormat(fermium::AUDIO_U16SYS as u16);
  
  /// `i32` little-endian
  pub const I32_LE: AudioFormat = AudioFormat(fermium::AUDIO_S32LSB as u16);
  /// `i32` big-endian
  pub const I32_BE: AudioFormat = AudioFormat(fermium::AUDIO_S32MSB as u16);
  /// `i32` system-endian
  pub const I32_SYS: AudioFormat = AudioFormat(fermium::AUDIO_S32SYS as u16);
  
  /// `f32` little-endian
  pub const F32_LE: AudioFormat = AudioFormat(fermium::AUDIO_F32LSB as u16);
  /// `f32` big-endian
  pub const F32_BE: AudioFormat = AudioFormat(fermium::AUDIO_F32MSB as u16);
  /// `f32` system-endian
  pub const F32_SYS: AudioFormat = AudioFormat(fermium::AUDIO_F32SYS as u16);
}
impl AudioFormat {
  /// If the format is a signed type.
  pub fn is_signed(self) -> bool {
    (self.0 as i16) < 0
  }

  /// If the format is big-endian
  pub fn is_big_endian(self) -> bool {
    (self.0 & 0b00010000_00000000) > 0
  }

  /// If the format is floating point
  pub fn is_floating(self) -> bool {
    (self.0 & 0b00000001_00000000) > 0
  }

  /// The number of bits per sample in this format.
  pub fn bit_size(self) -> u8 {
    self.0 as u8
  }
}

/// The number of audio output channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AudioChannels {
  /// Single sound
  Mono = 1,
  /// Two channels:
  /// * Left
  /// * Right
  Stereo = 2,
  /// Four channels:
  /// * Front-left
  /// * Front-right
  /// * Rear-left
  /// * Rear-right
  Quad = 4,
  /// Also called "5.1":
  /// * Front-left
  /// * Front-right
  /// * Center
  /// * Low-freq (the ".1")
  /// * Rear-left
  /// * Rear-right
  Surround = 6,
}

/// The status of an audio device.
#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
pub enum AudioStatus {
  /// Device is stopped.
  /// 
  /// Usually indicates a closed or error'd device.
  Stopped = fermium::SDL_AUDIO_STOPPED,
  /// Device is playing.
  Playing = fermium::SDL_AUDIO_PLAYING,
  /// Device is paused.
  Paused = fermium::SDL_AUDIO_PAUSED,
}
