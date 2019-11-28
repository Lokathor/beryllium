use super::*;

pub(crate) static WINDOW_EXISTS: AtomicBool = AtomicBool::new(false);

/// A handle to the SDL API.
///
/// Having one of these is the proof that you've called `SDL_Init`.
///
/// * Most of SDL requires you to have initialized the API.
/// * SDL is generally not thread-safe, and the GUI can only be used from the
///   main thread on Mac, so basically
pub struct SDL {
  #[allow(unused)]
  init_token: Arc<Initialization>,
}
impl SDL {
  /// Initializes SDL with the flags given.
  ///
  /// ## Failure
  ///
  /// * Fails on Mac if you're not on the main thread (according to `NSThread`).
  /// * Fails if SDL is currently initialized.
  /// * Fails if `SDL_Init` fails for whatever reason.
  pub fn init(flags: InitFlags) -> Result<Self, CowStr> {
    Ok(Self { init_token: Arc::new(Initialization::new(flags)?) })
  }

  /// Obtains the current SDL error string.
  ///
  /// In practice it's unlikely that you will need to call this yourself.
  /// Essentially all APIs that can error will call this for you when an error
  /// does happen.
  pub fn get_error(&self) -> String {
    self.init_token.get_error()
  }

  /// Sets an OpenGL attribute to the given value.
  ///
  /// Make all of these calls **before** making your OpenGL-enabled Window.
  ///
  /// The final context that you get might differ from your request. Use
  /// [`gl_get_attribute`](GlWindow::gl_get_attribute) on your contest after
  /// you've made your [`GlWindow`] to check what you actually got.
  ///
  /// ## Failure
  ///
  /// The `SdlGlAttr` will only let you set valid attribute names, but there's
  /// no checking on Rust's part that the value you pass is allowed for that
  /// attribute. If you pass an invalid value SDL will generate an error.
  pub fn gl_set_attribute(
    &self,
    attr: SdlGlAttr,
    value: i32,
  ) -> Result<(), String> {
    let ret = unsafe {
      fermium::SDL_GL_SetAttribute(attr as fermium::SDL_GLattr, value)
    };
    if ret >= 0 {
      Ok(())
    } else {
      Err(self.get_error())
    }
  }

  /// Resets all GL Attributes to their default values.
  pub fn gl_reset_attributes(&self) {
    unsafe { fermium::SDL_GL_ResetAttributes() }
  }

  /// Makes a window with a GL context in one step.
  ///
  /// beryllium currently only allows one window
  pub fn create_gl_window(
    &self,
    title: &str,
    pos: WindowPosition,
    width: u32,
    height: u32,
    flags: u32,
  ) -> Result<GlWindow, String> {
    if WINDOW_EXISTS.swap(true, Ordering::SeqCst) {
      Err(String::from("beryllium: There's already a window!"))
    } else {
      // make a window
      let title_null = title.alloc_c_str();
      let (x, y) = pos.what_sdl_wants();
      let win = unsafe {
        fermium::SDL_CreateWindow(
          title_null.as_ptr(),
          x,
          y,
          width as i32,
          height as i32,
          flags | (fermium::SDL_WINDOW_OPENGL as u32),
        )
      };
      if win.is_null() {
        return Err(self.get_error());
      }
      // now it'll drop
      let win = Window { win };

      // make a context
      let ctx = unsafe { fermium::SDL_GL_CreateContext(win.win) };
      if ctx.is_null() {
        return Err(self.get_error());
      }
      // now it'll drop
      let ctx = GlContext { ctx };

      Ok(GlWindow {
        init_token: self.init_token.clone(),
        win: ManuallyDrop::new(win),
        ctx: ManuallyDrop::new(ctx),
      })
    }
  }

  /// Blocks for at least `ms` milliseconds before returning.
  ///
  /// It might be longer than that because of OS scheduling.
  pub fn delay_ms(&self, ms: u32) {
    unsafe { fermium::SDL_Delay(ms) }
  }

  /// Blocks for the given [`Duration`](core::time::Duration) with millisecond
  /// granularity.
  ///
  /// If the duration is more than `u32::max_value()` milliseconds.
  ///
  /// 1) Seriously, what the hell? Are you okay friend? Sleeping that much?
  /// 2) It uses more than one sleep in a loop because you do you.
  pub fn delay_duration(&self, duration: core::time::Duration) {
    let mut ms_remaining = duration.as_millis();
    const TIME_CHUNK: u128 = u32::max_value() as u128;
    while ms_remaining > TIME_CHUNK {
      unsafe { fermium::SDL_Delay(TIME_CHUNK as u32) }
      ms_remaining -= TIME_CHUNK;
    }
    self.delay_ms(ms_remaining as u32)
  }

  /// Polls for an event.
  ///
  /// This returns `Some(result)` if there was an event in the queue, otherwise
  /// it returns immediately with `None.
  ///
  /// The Result is from `Event::try_from(sdl_event)`, so if `beryllium`
  /// understands the event it'll parse it for you, otherwise you get the raw
  /// [`SDL_Event`](fermium::SDL_Event) data.
  ///
  /// Pro Tip: Use `sdl.poll_events().and_then(Result::ok)` if you only want
  /// to see parsed events.
  pub fn poll_events(&self) -> Option<Result<Event, fermium::SDL_Event>> {
    let mut sdl_event = fermium::SDL_Event::default();
    let had_event = unsafe { fermium::SDL_PollEvent(&mut sdl_event) };
    if had_event > 0 {
      Some(Event::try_from(sdl_event))
    } else {
      None
    }
  }

  /// Checks the number of audio playback devices.
  ///
  /// ## Failure
  ///
  /// It's possible that the list can't be checked. In this case, you still
  /// might be able to open the "default" device. Pass `None` instead of a
  /// device name and hope for the best.
  pub fn get_audio_playback_device_count(&self) -> Option<usize> {
    if unsafe { fermium::SDL_WasInit(InitFlags::Audio.0) == 0 } {
      return None;
    }
    let ret = unsafe { fermium::SDL_GetNumAudioDevices(i32::from(false)) };
    if ret >= 0_i32 {
      Some(ret as usize)
    } else {
      None
    }
  }

  /// Get the name of a playback device.
  ///
  /// Indexes remain consistent until the next call to
  /// [`get_audio_playback_device_count`](SDL::get_audio_playback_device_count)
  pub fn get_audio_playback_device_name(&self, index: usize) -> Option<String> {
    if unsafe { fermium::SDL_WasInit(InitFlags::Audio.0) == 0 } {
      return None;
    }
    let mut ptr = unsafe {
      fermium::SDL_GetAudioDeviceName(index as i32, i32::from(false))
    };
    if ptr.is_null() {
      None
    } else {
      let mut v = Vec::with_capacity(128);
      unsafe {
        while *ptr != 0 {
          v.push(*ptr as u8);
          ptr = ptr.offset(1);
        }
      }
      Some(String::from_utf8_lossy(&v).into_owned())
    }
  }

  /// Checks the number of audio recording devices.
  ///
  /// ## Failure
  ///
  /// It's possible that the list can't be checked. In this case, you still
  /// might be able to open the "default" device. Pass `None` instead of a
  /// device name and hope for the best.
  pub fn get_audio_recording_device_count(&self) -> Option<u32> {
    if unsafe { fermium::SDL_WasInit(InitFlags::Audio.0) == 0 } {
      return None;
    }
    let ret = unsafe { fermium::SDL_GetNumAudioDevices(i32::from(true)) };
    if ret >= 0_i32 {
      Some(ret as u32)
    } else {
      None
    }
  }

  /// Get the name of a recording device.
  ///
  /// Indexes remain consistent until the next call to
  /// [`get_audio_playback_device_count`](SDL::get_audio_playback_device_count)
  pub fn get_audio_recording_device_name(
    &self,
    index: usize,
  ) -> Option<String> {
    if unsafe { fermium::SDL_WasInit(InitFlags::Audio.0) == 0 } {
      return None;
    }
    let mut ptr =
      unsafe { fermium::SDL_GetAudioDeviceName(index as i32, i32::from(true)) };
    if ptr.is_null() {
      None
    } else {
      let mut v = Vec::with_capacity(128);
      unsafe {
        while *ptr != 0 {
          v.push(*ptr as u8);
          ptr = ptr.offset(1);
        }
      }
      Some(String::from_utf8_lossy(&v).into_owned())
    }
  }

  /// Attempts to open an audio queue.
  pub fn open_audio_queue(
    &self,
    name: Option<&str>,
    request: AudioQueueRequest,
  ) -> Result<AudioQueue, String> {
    let name_null = name.unwrap_or("").alloc_c_str();
    let mut in_spec = fermium::SDL_AudioSpec::default();
    in_spec.freq = request.frequency as i32;
    in_spec.format = request.sample_format.0;
    in_spec.channels = request.channels as u8;
    in_spec.samples = request.sample_count.next_power_of_two();
    let mut out_spec = fermium::SDL_AudioSpec::default();
    let mut changes = 0_i32;
    if request.allow_frequency_change {
      changes |= fermium::SDL_AUDIO_ALLOW_FREQUENCY_CHANGE as i32
    }
    if request.allow_format_change {
      changes |= fermium::SDL_AUDIO_ALLOW_FORMAT_CHANGE as i32
    }
    if request.allow_channels_change {
      changes |= fermium::SDL_AUDIO_ALLOW_CHANNELS_CHANGE as i32
    }
    let device = unsafe {
      fermium::SDL_OpenAudioDevice(
        name_null.as_ptr(),
        i32::from(false),
        &in_spec,
        &mut out_spec,
        changes,
      )
    };
    if device > 0 {
      Ok(AudioQueue {
        init_token: self.init_token.clone(),
        device,
        spec: out_spec,
      })
    } else {
      Err(self.get_error())
    }
  }
}
