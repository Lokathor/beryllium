// I don't know how this works, I just followed https://sotrh.github.io/learn-wgpu

use beryllium::{event::*, init::*, vk_window::VkWindow, window::*, SdlResult};
use zstring::zstr;

fn main() -> SdlResult<()> {
  env_logger::init();
  let sdl = Sdl::init(InitFlags::EVERYTHING)?;

  let vk_win = sdl.create_vk_window(
    zstr!("WGPU Demo Window"),
    None,
    (800, 600),
    WindowFlags::ALLOW_HIGHDPI,
  )?;
  let mut state = pollster::block_on(State::new(&vk_win));

  'top: loop {
    // process all pending events
    while let Some(e) = sdl.poll_event() {
      match e {
        Event::Quit => break 'top,
        _ => (),
      }
    }
    // now update and draw

    state.render().unwrap();
  }
  Ok(())
}

pub struct State {
  surface: wgpu::Surface,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  size: (u32, u32),
}

impl State {
  // Creating some of the wgpu types requires async code
  async fn new(window: &VkWindow) -> Self {
    use core::ops::Deref;
    //
    let size = window.get_drawable_size();
    assert!(size.0 > 0);
    assert!(size.1 > 0);
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(window.deref()) };
    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await
      .unwrap();
    let (device, queue) = adapter
      .request_device(
        &wgpu::DeviceDescriptor {
          features: wgpu::Features::empty(),
          limits: wgpu::Limits::default(),
          label: None,
        },
        None, // Trace path
      )
      .await
      .unwrap();
    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface.get_preferred_format(&adapter).unwrap(),
      width: size.0,
      height: size.1,
      present_mode: wgpu::PresentMode::Fifo,
    };
    surface.configure(&device, &config);
    //
    Self { surface, device, queue, config, size }
  }

  pub fn resize(&mut self, new_size: (u32, u32)) {
    assert!(new_size.0 > 0);
    assert!(new_size.1 > 0);
    self.size = new_size;
    self.config.width = new_size.0;
    self.config.height = new_size.1;
    self.surface.configure(&self.device, &self.config);
  }

  pub fn input(&mut self, _event: Event) -> bool {
    false
  }

  pub fn update(&mut self) {
    //todo!()
  }

  pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    let output = self.surface.get_current_texture()?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Render Encoder") });
    {
      let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0 }),
            store: true,
          },
        }],
        depth_stencil_attachment: None,
      });
      // in a full program we'd draw something with our render pass, but for
      // just an example we can "do nothing" and that'll still the window to out
      // designated color, which is enough to see that the beryllium part of
      // things are all working fine.
    }
    // we have to drop all render passes before we can finish the encoder, thus
    // the above extra scope block to force the drop.
    self.queue.submit(core::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }
}
