//! This demo plays a "Middle C" note for three seconds.

use beryllium::*;

fn main() -> Result<(), String> {
  let sdl = unsafe { init()? };

  const FREQUENCY: i32 = 48000;
  const TWO_PI: f32 = core::f32::consts::PI * 2.0;
  const ONE_HZ_OF_SOUND: f32 = TWO_PI / FREQUENCY as f32;

  // We give the API an audio queue request, and we tell it to _not_ change
  // anything in the device we get back. What this means is that if the hardware
  // cannot accommodate our request in some way, SDL2 will act as a middle man
  // and silently convert between the format we want and the closest match that
  // the sound card actually supports.
  let audio = sdl.open_default_audio_queue(DefaultAudioQueueRequest {
    frequency: FREQUENCY,
    format: AudioFormat::S16SYS,
    channels: 1,
    samples: 4096,
    allow_frequency_change: false,
    allow_format_change: false,
    allow_channels_change: false,
  })?;
  // Alternately, if your code is able to adapt, you can allow a change and
  // you'll still get the closest possible match, but perhaps not exactly what
  // you wanted. You'd want to do this if you think that you're better able to
  // cope with any format conversion considerations than SDL2 is (eg, if you've
  // written a high quality synth program that can target any sample format, you
  // might as well target the sound card's native format and avoid an extra
  // conversion).

  // We'll play for 3 seconds.
  let out_sample_count = FREQUENCY as usize * 3;
  let mut v: Vec<i16> = Vec::with_capacity(out_sample_count);

  // We fill the buffer with a basic "middle C" sound wave. Frequency taken from
  // http://pages.mtu.edu/~suits/notefreqs.html
  let angle_per_sample = 261.63 * ONE_HZ_OF_SOUND;
  let mut angle: f32 = 0.0;
  for _ in 0..out_sample_count {
    v.push((angle.sin() * 3000.0) as i16);
    angle += angle_per_sample;
    if angle >= TWO_PI {
      angle -= TWO_PI;
    }
  }

  // Regardless of the actual sample data format, you always convert your sample
  // buffer into a &[u8] and just throw the bytes into the queue. It's a C API
  // after all, they have a flexible concept of how types work.
  let byte_slice: &[u8] = unsafe {
    core::slice::from_raw_parts(
      v.as_ptr() as *const u8,
      v.len() * core::mem::size_of::<i16>(),
    )
  };
  audio.queue_audio(byte_slice)?;
  audio.set_paused(false);
  // We just sleep while that sound plays out
  std::thread::sleep(std::time::Duration::from_secs(3));
  Ok(())
}
