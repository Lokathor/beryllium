use super::*;

/// An `AudioQueue` lets you output audio by pushing samples to the device.
/// 
/// If your queue runs dry SDL will automatically output silence for you.
pub struct AudioQueue {
  #[allow(unused)]
  pub(crate) init_token: Arc<Initialization>,
  pub(crate) device: fermium::SDL_AudioDeviceID,
  pub(crate) spec: fermium::SDL_AudioSpec,
}
unsafe impl Send for AudioQueue {}
impl Drop for AudioQueue {
  fn drop(&mut self) {
    unsafe { fermium::SDL_CloseAudioDevice(self.device) }
  }
}

impl AudioQueue {
  /// The status of the `AudioQueue`.
  ///
  /// Normally this will be `Playing` or `Paused`. If you get `Stopped` the
  /// device might have been closed or there could be some other error.
  pub fn get_audio_status(&self) -> AudioStatus {
    match unsafe { fermium::SDL_GetAudioDeviceStatus(self.device) } {
      fermium::SDL_AUDIO_STOPPED => AudioStatus::Stopped,
      fermium::SDL_AUDIO_PLAYING => AudioStatus::Playing,
      fermium::SDL_AUDIO_PAUSED => AudioStatus::Paused,
      _ => AudioStatus::Stopped,
    }
  }

  /// The number of **bytes** (not samples!) that are queued for playback.
  pub fn get_queued_byte_count(&self) -> usize {
    (unsafe { fermium::SDL_GetQueuedAudioSize(self.device) }) as usize
  }

  /// Queues more audio data.
  ///
  /// Pro Tip: [`bytemuck::cast_slice`](https://docs.rs/bytemuck) lets you
  /// easily turn your slice of samples into a slice of bytes.
  ///
  /// ## Failure
  ///
  /// * If you try queue a slice with a length that won't fit into `u32` this
  ///   will immediately return an error without queueing the data.
  /// * If any other error happens this can't call to get the error message
  ///   itself because you might be running it in another thread. Feel free to
  ///   get the error yourself (which you can only do from the main thread).
  /// * Does this mean that SDL2 can trigger internal data races in the error
  ///   string? Yeah, probably :/ I'll investigate and fix this if it turns out
  ///   that the error messages are thread safe.
  pub fn queue_audio(&self, bytes: &[u8]) -> Result<(), ()> {
    if bytes.len() > (u32::max_value() as usize) {
      return Err(());
    }
    let ret = unsafe {
      fermium::SDL_QueueAudio(
        self.device,
        bytes.as_ptr().cast(),
        bytes.len() as u32,
      )
    };
    if ret >= 0 {
      Ok(())
    } else {
      Err(())
    }
  }

  /// Clears the queue of samples on SDL's side.
  ///
  /// If samples have been sent to the audio device but not yet played SDL can't
  /// do anything about that.
  pub fn clear_queue(&self) {
    unsafe { fermium::SDL_ClearQueuedAudio(self.device) }
  }

  /// If you want the output to be paused or not.
  pub fn set_paused(&self, paused: bool) {
    unsafe { fermium::SDL_PauseAudioDevice(self.device, i32::from(paused)) }
  }

  /// The output sample frequency.
  pub fn frequency(&self) -> i32 {
    self.spec.freq
  }

  /// The output sample format.
  pub fn format(&self) -> AudioFormat {
    AudioFormat(self.spec.format)
  }

  /// The number of output channels.
  pub fn channels(&self) -> Option<AudioChannels> {
    Some(match self.spec.channels {
      1 => AudioChannels::Mono,
      2 => AudioChannels::Stereo,
      4 => AudioChannels::Quad,
      6 => AudioChannels::Surround,
      _ => return None,
    })
  }

  /// The value to use for a "silence" sample.
  pub fn silence_value(&self) -> u8 {
    self.spec.silence
  }

  /// The audio buffer's sample count.
  pub fn buffer_sample_count(&self) -> u16 {
    self.spec.samples
  }

  /// The audio buffer's byte count.
  pub fn buffer_byte_count(&self) -> usize {
    self.spec.size as usize
  }
}

/// This lets you arrange your request for an `AudioQueue`.
///
/// Three of the fields are for "allowed changes". Whatever audio output device
/// you open might have a format other than your request. If your exact request
/// can't be attained you can "allow" a change or not.
///
/// * If you allow a change, the actual device that you get back will report its
///   real capabilities and you'll have to follow those.
/// * If you don't allow a change the device itself will still follow its real
///   capabilities but SDL will internally do all necessary conversions for you
///   so that from your side of things you don't have to change what you're
///   doing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioQueueRequest {
  /// Desired frequency
  pub frequency: u32,

  /// Desired output format.
  pub sample_format: AudioFormat,

  /// Desired output channel count.
  pub channels: AudioChannels,

  /// The intended number of samples in the audio buffer.
  /// 
  /// This is automatically rounded up to the next power of 2.
  pub sample_count: u16,

  /// If you want to allow a frequency change.
  pub allow_frequency_change: bool,

  /// If you want to allow a sample format change.
  pub allow_format_change: bool,

  /// If you want to allow a channel count change.
  pub allow_channels_change: bool,
}
