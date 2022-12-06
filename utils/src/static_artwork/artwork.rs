use std::path::PathBuf;

use super::{base_model::StaticBaseModel, constants};

pub trait StaticArtwork {
  fn new(model: StaticBaseModel) -> Self;
  fn get_options() -> StaticArtworkOptions;
  fn get_model(&self) -> &StaticBaseModel;
  fn get_model_mut(&mut self) -> &mut StaticBaseModel;
  fn draw(&self);
}

pub struct StaticArtworkOptions {
  pub texture_size: [u32; 2],
  pub render_size: [u32; 2],
  pub background_path: Option<PathBuf>,
}

impl Default for StaticArtworkOptions {
  fn default() -> Self {
    Self {
      texture_size: constants::TEXTURE_SIZE,
      render_size: constants::RENDER_SIZE,
      background_path: None,
    }
  }
}
