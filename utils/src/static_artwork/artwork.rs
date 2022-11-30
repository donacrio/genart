use super::{base_model::StaticBaseModel, constants};

pub trait StaticArtwork {
  fn new(model: StaticBaseModel) -> Self;
  fn get_model(&self) -> &StaticBaseModel;
  fn get_model_mut(&mut self) -> &mut StaticBaseModel;
  fn get_options() -> StaticArtworkOptions;
}

pub struct StaticArtworkOptions {
  pub texture_size: [u32; 2],
  pub render_size: [u32; 2],
}

impl Default for StaticArtworkOptions {
  fn default() -> StaticArtworkOptions {
    StaticArtworkOptions {
      texture_size: constants::TEXTURE_SIZE,
      render_size: constants::RENDER_SIZE,
    }
  }
}
