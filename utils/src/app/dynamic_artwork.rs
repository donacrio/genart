use nannou::prelude::map_range;

use super::nannou_app::{make_base_nannou_app, NannouApp};

pub trait DynamicArtwork: NannouApp {
  fn draw_at_time(&mut self, t: f64); // TODO: make immutable ref and create update function with mutable ref
  fn fps(&self) -> u32;
  fn n_sec(&self) -> u32;
  fn current_frame(&mut self) -> &mut u32;
}

// TODO: create slider to render a specific frame
pub fn update_dynamic<T: DynamicArtwork>(model: &mut T) {
  let n_frames = model.fps() * model.n_sec();
  let elapsed_frames = *model.current_frame();
  let t = map_range(elapsed_frames as f64, 0., n_frames as f64, 0., 1.);
  model.draw_at_time(t);

  *model.current_frame() = if elapsed_frames == n_frames {
    0
  } else {
    *model.current_frame() + 1
  }
}

pub fn make_dynamic_artwork<T: 'static + DynamicArtwork>() -> nannou::app::Builder<T> {
  make_base_nannou_app()
}
