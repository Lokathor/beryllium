//! This demo plays a "Middle C" note for three seconds.

use beryllium::*;

fn main() -> Result<(), String> {
  let sdl = unsafe { init()? };

  const FREQUENCY: i32 = 48000;
  const TWO_PI: f32 = core::f32::consts::PI * 2.0;
  const ONE_HZ_OF_SOUND: f32 = TWO_PI / FREQUENCY as f32;

  let audio = sdl.open_default_audio_queue(DefaultAudioQueueRequest {
    frequency: FREQUENCY,
    format: AudioFormat::S16SYS,
    channels: 1,
    samples: 4096,
    allow_frequency_change: false,
    allow_format_change: false,
    allow_channels_change: false,
  })?;

  // We'll play for 3 seconds.
  let out_sample_count = FREQUENCY as usize * 3;
  let mut v: Vec<i16> = Vec::with_capacity(out_sample_count);
  let mut angle: f32 = 0.0;
  // this gives us a "Middle C" sound, http://pages.mtu.edu/~suits/notefreqs.html
  let angle_per_sample = 261.63 * ONE_HZ_OF_SOUND;
  for _ in 0..out_sample_count {
    v.push((angle.sin() * 3000.0) as i16);
    angle += angle_per_sample;
    if angle >= TWO_PI {
      angle -= TWO_PI;
    }
  }

  let byte_slice: &[u8] = unsafe {
    core::slice::from_raw_parts(
      v.as_ptr() as *const _,
      v.len() * core::mem::size_of::<i16>(),
    )
  };
  audio.queue_audio(byte_slice)?;
  audio.set_paused(false);
  std::thread::sleep(std::time::Duration::from_secs(3));
  Ok(())
}
