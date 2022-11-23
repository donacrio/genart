use nannou::wgpu;
use nannou::wgpu::Maintain;

pub struct Display {
  texture: wgpu::Texture,
  renderer: nannou::draw::Renderer,
  texture_capturer: wgpu::TextureCapturer,
  texture_reshaper: wgpu::TextureReshaper,
}

impl Display {
  pub fn new(window: &nannou::window::Window, texture_size: [u32; 2]) -> Self {
    let device = window.device();
    let texture = wgpu::TextureBuilder::new()
      .size(texture_size)
      .usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
      .sample_count(window.msaa_samples())
      .format(wgpu::TextureFormat::Rgba16Float)
      .build(device);

    let renderer = nannou::draw::RendererBuilder::new()
      .build_from_texture_descriptor(device, texture.descriptor());

    let texture_capturer = wgpu::TextureCapturer::default();

    let texture_reshaper = wgpu::TextureReshaper::new(
      window.device(),
      &texture.view().build(),
      window.msaa_samples(),
      texture.sample_type(),
      window.msaa_samples(),
      nannou::Frame::TEXTURE_FORMAT,
    );

    Display {
      texture,
      renderer,
      texture_capturer,
      texture_reshaper,
    }
  }

  pub fn create_snapshot(
    &mut self,
    window: &nannou::window::Window,
    draw: &nannou::draw::Draw,
  ) -> wgpu::TextueSnapshot {
    let device = window.device();

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: Some("Texture Renderer"),
    });

    self
      .renderer
      .render_to_texture(device, &mut encoder, draw, &self.texture);

    let snapshot = self
      .texture_capturer
      .capture(device, &mut encoder, &self.texture);

    window.queue().submit(Some(encoder.finish()));
    window.device().poll(Maintain::Wait);

    snapshot
  }

  pub fn save_snapshot(snapshot: wgpu::TextueSnapshot, path: std::path::PathBuf) {
    snapshot
      .read(move |result| {
        let image = result.expect("Failed to map texture memory").to_owned();
        image
          .save(&path)
          .expect("Failed to save texture to png image");
        println!("Saved {}", path.display());
      })
      .unwrap();
  }

  pub fn render(&self, frame: nannou::frame::Frame) {
    self
      .texture_reshaper
      .encode_render_pass(frame.texture_view(), &mut frame.command_encoder());
  }

  pub fn wait(&self, window: &nannou::window::Window) {
    self
      .texture_capturer
      .await_active_snapshots(window.device())
      .unwrap();
  }
}
