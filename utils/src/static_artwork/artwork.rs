use super::{base_model::StaticBaseModel, constants};
use nannou::{prelude::Key, App};
use std::path::PathBuf;

pub trait StaticArtwork {
  fn new(model: StaticBaseModel) -> Self;
  fn get_options() -> StaticArtworkOptions;
  fn get_model(&self) -> &StaticBaseModel;
  fn get_model_mut(&mut self) -> &mut StaticBaseModel;
  fn current_frame_name(&self) -> String;
  fn draw(&self);
  fn key_pressed(&mut self, app: &App, key: Key);
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
