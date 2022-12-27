use nannou::{prelude::map_range, App};

use super::nannou_app::{make_base_nannou_app, NannouApp};

pub trait DynamicArtwork: NannouApp {
  fn draw_at_time(&self, t: f64);
  fn fps(&self) -> u32;
  fn n_sec(&self) -> u32;
  fn recording(&self) -> bool;
  fn current_frame(&self) -> u32;
}

pub fn update_dynamic<T: DynamicArtwork>(model: &mut T, app: &App) {
  let [w, _] = model.get_base_model().texture.size();
  let n_frames = model.fps() * model.n_sec();
  let elapsed_frames = if model.recording() {
    model.current_frame()
  } else {
    let pos = 2. * (4. * app.mouse.x + (w as f32)) / w as f32;
    (pos * n_frames as f32) as u32 % n_frames
  };
  let t = map_range(elapsed_frames as f64, 0., n_frames as f64, 0., 1.);
  model.draw_at_time(t)
}

pub fn make_dynamic_artwork<T: 'static + DynamicArtwork>() -> nannou::app::Builder<T> {
  make_base_nannou_app()
}
