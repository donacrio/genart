use nannou::{geom, wgpu, App, Draw};
use std::path::PathBuf;

pub struct ImageSampler {
  texture: wgpu::Texture,
}

impl ImageSampler {
  pub fn new(app: &App, img_path: PathBuf) -> Self {
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();
    Self { texture }
  }

  pub fn draw(
    &self,
    draw: Draw,
    area: Option<geom::Rect>,
    address_mode: Option<wgpu::AddressMode>,
  ) -> Draw {
    let draw = match address_mode {
      Some(address_mode) => {
        let sampler = wgpu::SamplerBuilder::new()
          .address_mode(address_mode)
          .into_descriptor();
        draw.sampler(sampler)
      }
      None => draw,
    };
    match area {
      Some(area) => draw.texture(&self.texture).area(area),
      None => draw.texture(&self.texture),
    };

    draw
  }
}
