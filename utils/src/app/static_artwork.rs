use super::nannou_app::{make_base_nannou_app, NannouApp};
use nannou::LoopMode;

pub trait StaticArtwork: NannouApp {
  fn draw(&self);
}

pub fn update_static<T: StaticArtwork>(model: &mut T) {
  model.draw()
}

pub fn make_static_artwork<T: 'static + StaticArtwork>() -> nannou::app::Builder<T> {
  make_base_nannou_app().loop_mode(LoopMode::wait())
}
