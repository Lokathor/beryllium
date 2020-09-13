use core::{ffi::c_void, mem::MaybeUninit};

use alloc::sync::Arc;

use fermium::{SDL_AudioDeviceID, SDL_AudioSpec, SDL_OpenAudioDevice};

use tinyvec::TinyVec;

use crate::{sdl_get_error, Initialization, SdlError};

pub struct AudioDevice {
  // TODO: NonZeroUWhatever?
  device_id: SDL_AudioDeviceID,
  // Note(Lokathor): As long as the device is open, we have to also keep SDL
  // itself alive.
  #[allow(dead_code)]
  init: Arc<Initialization>,
}
impl Drop for AudioDevice {
  // Note(Lokathor): The drop for the Arc runs *after* this drop code.
  fn drop(&mut self) {
    unsafe { fermium::SDL_CloseAudioDevice(self.device_id) }
  }
}
/// The Audio subsystem is the only part of SDL that **is** thread safe.
unsafe impl Send for AudioDevice {}
unsafe impl Sync for AudioDevice {}

pub struct AudioFormat(u16);
impl AudioFormat {
  ///signed 8-bit samples
  pub const S8: Self = Self(fermium::AUDIO_S8 as _);
  ///unsigned 8-bit samples
  pub const U8: Self = Self(fermium::AUDIO_U8 as _);
  ///signed 16-bit samples in little-endian byte order
  pub const S16LSB: Self = Self(fermium::AUDIO_S16LSB as _);
  ///signed 16-bit samples in big-endian byte order
  pub const S16MSB: Self = Self(fermium::AUDIO_S16MSB as _);
  ///signed 16-bit samples in native byte order
  pub const S16SYS: Self = Self(fermium::AUDIO_S16SYS as _);
  /// AUDIO_S16LSB
  pub const S16: Self = Self(fermium::AUDIO_S16 as _);
  /// unsigned 16-bit samples in little-endian byte order
  pub const U16LSB: Self = Self(fermium::AUDIO_U16LSB as _);
  /// unsigned 16-bit samples in big-endian byte order
  pub const U16MSB: Self = Self(fermium::AUDIO_U16MSB as _);
  /// unsigned 16-bit samples in native byte order
  pub const U16SYS: Self = Self(fermium::AUDIO_U16SYS as _);
  /// AUDIO_U16LSB
  pub const U16: Self = Self(fermium::AUDIO_U16 as _);
  /// 32-bit integer samples in little-endian byte order
  pub const S32LSB: Self = Self(fermium::AUDIO_S32LSB as _);
  /// 32-bit integer samples in big-endian byte order
  pub const S32MSB: Self = Self(fermium::AUDIO_S32MSB as _);
  /// 32-bit integer samples in native byte order
  pub const S32SYS: Self = Self(fermium::AUDIO_S32SYS as _);
  /// AUDIO_S32LSB
  pub const S32: Self = Self(fermium::AUDIO_S32 as _);
  /// 32-bit floating point samples in little-endian byte order
  pub const F32LSB: Self = Self(fermium::AUDIO_F32LSB as _);
  /// 32-bit floating point samples in big-endian byte order
  pub const F32MSB: Self = Self(fermium::AUDIO_F32MSB as _);
  /// 32-bit floating point samples in native byte order
  pub const F32SYS: Self = Self(fermium::AUDIO_F32SYS as _);
  /// AUDIO_F32LSB
  pub const F32: Self = Self(fermium::AUDIO_F32 as _);
}

pub struct AllowedAudioChanges(i32);
impl AllowedAudioChanges {
  pub const FREQUENCY: Self =
    Self(fermium::SDL_AUDIO_ALLOW_FREQUENCY_CHANGE as _);
  pub const FORMAT: Self = Self(fermium::SDL_AUDIO_ALLOW_FORMAT_CHANGE as _);
  pub const CHANNELS: Self =
    Self(fermium::SDL_AUDIO_ALLOW_CHANNELS_CHANGE as _);
  pub const ANY: Self = Self(fermium::SDL_AUDIO_ALLOW_ANY_CHANGE as _);
}

pub struct AudioDeviceObtainedSpec {
  pub frequency: i32,
  pub format: AudioFormat,
  pub channels: u8,
  /// Should be a power of two (4096, etc)
  pub sample_count: u16,
  pub silence: u8,
  /// Buffer size in bytes
  pub size: usize,
}

// // // // //
// Audio Queue
// // // // //

pub struct AudioQueueRequestSpec {
  pub frequency: i32,
  pub format: AudioFormat,
  pub channels: u8,
  /// Should be a power of two (4096, etc)
  pub sample_count: u16,
}

pub struct AudioQueueDevice(AudioDevice);
impl AudioQueueDevice {
  pub(crate) fn open(
    init: Arc<Initialization>, device_name: Option<&str>, capture: bool,
    spec: &AudioQueueRequestSpec, changes: AllowedAudioChanges,
  ) -> Result<(Self, AudioDeviceObtainedSpec), SdlError> {
    let opt_device_null = device_name.map(|s| {
      s.as_bytes().iter().copied().chain(Some(0)).collect::<TinyVec<[u8; 64]>>()
    });
    let device_null: *const u8 = match opt_device_null.as_ref() {
      Some(device_null_ref) => device_null_ref.as_ptr(),
      None => core::ptr::null(),
    };
    let desired = SDL_AudioSpec {
      freq: spec.frequency,
      format: spec.format.0,
      channels: spec.channels,
      silence: /* calculated */ 0,
      samples: spec.sample_count,
      size: /* calculated */ 0,
      callback: None,
      userdata: core::ptr::null_mut(),
      padding: 0,
    };
    let mut obtained = SDL_AudioSpec::default();
    let device_id = unsafe {
      SDL_OpenAudioDevice(
        device_null.cast(),
        capture as _,
        &desired,
        &mut obtained,
        changes.0,
      )
    };
    if device_id > 0 {
      let queue = AudioQueueDevice(AudioDevice { device_id, init });
      let obtained_spec = AudioDeviceObtainedSpec {
        frequency: obtained.freq,
        format: AudioFormat(obtained.format),
        channels: obtained.channels,
        sample_count: obtained.samples,
        silence: obtained.silence,
        size: obtained.size as usize,
      };
      Ok((queue, obtained_spec))
    } else {
      Err(sdl_get_error())
    }
  }
}

// // // // //
// Audio Callback
// // // // //

pub struct AudioCallbackRequestSpec {
  pub frequency: i32,
  pub format: AudioFormat,
  pub channels: u8,
  /// Should be a power of two (4096, etc)
  pub sample_count: u16,
  /// Usually runs in a **separate** thread from the main thread.
  pub callback: unsafe extern "C" fn(*mut c_void, *mut MaybeUninit<u8>, i32),
  pub userdata: *mut c_void,
}

pub struct AudioCallbackDevice(AudioDevice);
impl AudioCallbackDevice {
  pub(crate) unsafe fn open(
    init: Arc<Initialization>, device_name: Option<&str>, capture: bool,
    spec: &AudioCallbackRequestSpec, changes: AllowedAudioChanges,
  ) -> Result<(Self, AudioDeviceObtainedSpec), SdlError> {
    let opt_device_null = device_name.map(|s| {
      s.as_bytes().iter().copied().chain(Some(0)).collect::<TinyVec<[u8; 64]>>()
    });
    let device_null: *const u8 = match opt_device_null.as_ref() {
      Some(device_null_ref) => device_null_ref.as_ptr(),
      None => core::ptr::null(),
    };
    let desired = SDL_AudioSpec {
      freq: spec.frequency,
      format: spec.format.0,
      channels: spec.channels,
      silence: /* calculated */ 0,
      samples: spec.sample_count,
      size: /* calculated */ 0,
      callback: Some(core::mem::transmute(spec.callback)),
      userdata: spec.userdata,
      padding: 0,
    };
    let mut obtained = SDL_AudioSpec::default();
    let device_id = SDL_OpenAudioDevice(
      device_null.cast(),
      capture as _,
      &desired,
      &mut obtained,
      changes.0,
    );
    if device_id > 0 {
      let callback = AudioCallbackDevice(AudioDevice { device_id, init });
      let obtained_spec = AudioDeviceObtainedSpec {
        frequency: obtained.freq,
        format: AudioFormat(obtained.format),
        channels: obtained.channels,
        sample_count: obtained.samples,
        silence: obtained.silence,
        size: obtained.size as usize,
      };
      Ok((callback, obtained_spec))
    } else {
      Err(sdl_get_error())
    }
  }
}
