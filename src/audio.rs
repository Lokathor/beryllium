use super::*;

/// Specifies a particular sample data format.
#[derive(Debug, Clone, Copy)]
pub struct AudioFormat(pub(crate) u16);
#[allow(missing_docs)]
impl AudioFormat {
  phantom_fields! {
    self.0: u16,
    bit_size: 0-7,
    floating: 8,
    big_endian: 12,
    signed: 15,
  }
  pub const S8: AudioFormat = AudioFormat(fermium::AUDIO_S8 as u16);
  pub const U8: AudioFormat = AudioFormat(fermium::AUDIO_U8 as u16);
  pub const S16LSB: AudioFormat = AudioFormat(fermium::AUDIO_S16LSB as u16);
  pub const S16MSB: AudioFormat = AudioFormat(fermium::AUDIO_S16MSB as u16);
  pub const S16SYS: AudioFormat = AudioFormat(fermium::AUDIO_S16SYS as u16);
  pub const S16: AudioFormat = AudioFormat(fermium::AUDIO_S16 as u16);
  pub const U16LSB: AudioFormat = AudioFormat(fermium::AUDIO_U16LSB as u16);
  pub const U16MSB: AudioFormat = AudioFormat(fermium::AUDIO_U16MSB as u16);
  pub const U16SYS: AudioFormat = AudioFormat(fermium::AUDIO_U16SYS as u16);
  pub const U16: AudioFormat = AudioFormat(fermium::AUDIO_U16 as u16);
  pub const S32LSB: AudioFormat = AudioFormat(fermium::AUDIO_S32LSB as u16);
  pub const S32MSB: AudioFormat = AudioFormat(fermium::AUDIO_S32MSB as u16);
  pub const S32SYS: AudioFormat = AudioFormat(fermium::AUDIO_S32SYS as u16);
  pub const S32: AudioFormat = AudioFormat(fermium::AUDIO_S32 as u16);
  pub const F32LSB: AudioFormat = AudioFormat(fermium::AUDIO_F32LSB as u16);
  pub const F32MSB: AudioFormat = AudioFormat(fermium::AUDIO_F32MSB as u16);
  pub const F32SYS: AudioFormat = AudioFormat(fermium::AUDIO_F32SYS as u16);
  pub const F32: AudioFormat = AudioFormat(fermium::AUDIO_F32 as u16);
}
impl From<fermium::SDL_AudioFormat> for AudioFormat {
  fn from(format: fermium::SDL_AudioFormat) -> Self {
    Self(format)
  }
}

/// Specifies a request to open an audio queue.
#[derive(Debug, Clone, Copy)]
pub struct DefaultAudioQueueRequest {
  /// Samples per second
  pub frequency: i32,
  /// Sample data format.
  pub format: AudioFormat,
  /// Number of channels. Supported values are 1, 2, 4, or 6.
  pub channels: u8,
  /// Must be a power of 2.
  pub samples: u16,
  /// Allow the audio device you get to have a different frequency
  pub allow_frequency_change: bool,
  /// Allow the audio device you get to have a different format
  pub allow_format_change: bool,
  /// Allow the audio device you get to have a different channel count
  pub allow_channels_change: bool,
}

/// Handle to an audio device in "queue" mode, and info about its settings.
#[derive(Debug)]
pub struct AudioQueue<'sdl> {
  pub(crate) dev: fermium::SDL_AudioDeviceID,
  pub(crate) frequency: i32,
  pub(crate) format: AudioFormat,
  pub(crate) channels: u8,
  pub(crate) silence: u8,
  pub(crate) sample_count: usize,
  pub(crate) buffer_size: usize,
  pub(crate) _marker: PhantomData<&'sdl SDLToken>,
}
impl<'sdl> Drop for AudioQueue<'sdl> {
  fn drop(&mut self) {
    unsafe { fermium::SDL_CloseAudioDevice(self.dev) }
  }
}
impl<'sdl> AudioQueue<'sdl> {
  /// Samples per second
  pub fn frequency(&self) -> i32 {
    self.frequency
  }
  /// Sample data format
  pub fn format(&self) -> AudioFormat {
    self.format
  }
  /// Channel count
  pub fn channels(&self) -> u8 {
    self.channels
  }
  /// Silence value
  pub fn silence(&self) -> u8 {
    self.silence
  }
  /// Samples in the buffer
  pub fn sample_count(&self) -> usize {
    self.sample_count
  }
  /// Size (in bytes) of the buffer
  pub fn buffer_size(&self) -> usize {
    self.buffer_size
  }
  /// Sets the device into paused state or not.
  pub fn set_paused(&self, pause_on: bool) {
    unsafe { fermium::SDL_PauseAudioDevice(self.dev, pause_on as i32) }
  }
  /// Gets the current number of bytes of queued audio.
  ///
  /// NOTE: this seems to be a somewhat unreliable metric. When the queue runs
  /// low SDL2 seems to automatically re-queue some silence for you, but that
  /// silence time counts into the queue size. In other words, if you just leave
  /// the queue playing without pushing any new audio the queued byte size never
  /// hits 0. The queue size will go to 0 as expected if you clear it while
  /// playback it paused.
  pub fn queued_audio_size(&self) -> usize {
    unsafe { fermium::SDL_GetQueuedAudioSize(self.dev) as usize }
  }
  /// Clears any queued data that has not yet been sent to the sound card.
  pub fn clear(&self) {
    unsafe { fermium::SDL_ClearQueuedAudio(self.dev) }
  }
  /// Pushes audio data into the queue.
  ///
  /// The size of the queue has no particular limit, but you can't queue more
  /// than `u32::MAX` bytes at once.
  pub fn queue_audio(&self, data: &[u8]) -> Result<(), String> {
    assert!(data.len() < core::u32::MAX as usize);
    let ptr = data.as_ptr() as *const c_void;
    let len = data.len() as u32;
    let err = unsafe { fermium::SDL_QueueAudio(self.dev, ptr, len) };
    if err == 0 {
      Ok(())
    } else {
      Err(get_error())
    }
  }
}
