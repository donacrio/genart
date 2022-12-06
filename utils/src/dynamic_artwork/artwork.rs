use std::path::PathBuf;

use super::{base_model::DynamicBaseModel, constants};

pub trait DynamicArtwork {
  fn new(model: DynamicBaseModel) -> Self;
  fn get_options() -> DynamicArtworkOptions;
  fn get_model(&self) -> &DynamicBaseModel;
  fn get_model_mut(&mut self) -> &mut DynamicBaseModel;
  fn draw_at_time(&self, t: f64);
}

pub struct DynamicArtworkOptions {
  pub texture_size: [u32; 2],
  pub render_size: [u32; 2],
  pub background_path: Option<PathBuf>,
  pub fps: u32,
  pub n_sec: u32,
}

impl Default for DynamicArtworkOptions {
  fn default() -> Self {
    Self {
      texture_size: constants::TEXTURE_SIZE,
      render_size: constants::RENDER_SIZE,
      background_path: None,
      fps: constants::FPS,
      n_sec: constants::N_SEC,
    }
  }
}
