use super::nannou_app::{make_base_nannou_app, NannouApp};
use nannou::LoopMode;

pub trait StaticApp: NannouApp {
  fn draw(&self);
}

pub fn update_static<T: StaticApp>(model: &mut T) {
  model.draw()
}

pub fn make_static_artwork<T: 'static + StaticApp>() -> nannou::app::Builder<T> {
  make_base_nannou_app().loop_mode(LoopMode::wait())
}
