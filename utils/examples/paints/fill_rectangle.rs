use geo::{Coord, Rect};
use nannou::{
  prelude::{Key, BLUEVIOLET, WHITE},
  App,
};
use utils::static_artwork::{
  make_static_nannou_app, StaticArtwork, StaticArtworkOptions, StaticBaseModel,
};

fn main() {
  make_static_nannou_app::<Model>().run();
}

struct Model {
  base_model: StaticBaseModel,
  density: f64,
}

impl StaticArtwork for Model {
  fn new(base_model: StaticBaseModel) -> Self {
    Self {
      base_model,
      density: 0.5,
    }
  }
  fn get_options() -> StaticArtworkOptions {
    StaticArtworkOptions::default()
  }
  fn get_model(&self) -> &StaticBaseModel {
    &self.base_model
  }
  fn get_model_mut(&mut self) -> &mut StaticBaseModel {
    &mut self.base_model
  }
  fn current_frame_name(&self) -> String {
    String::from("frame")
  }
  fn draw(&self) {
    let draw = &self.base_model.draw;

    draw.background().color(WHITE);
    let [w_w, w_h] = self.base_model.texture.size();

    let w = w_w as f64 * 0.9;
    let h = w_h as f64 * 0.9;
    let min: Coord = (-w / 2.0, -h / 2.0).into();
    let max: Coord = (w / 2.0, h / 2.0).into();
    let rect = Rect::new(min, max);

    utils::paint::fill_rectangle(rect, self.density).for_each(|point| {
      draw
        .ellipse()
        .x(point.x() as f32)
        .y(point.y() as f32)
        .w_h(1.0, 1.0)
        .color(BLUEVIOLET);
    });
  }

  fn key_pressed(&mut self, _app: &App, key: Key) {
    match key {
      Key::Up => self.density += 0.1,
      Key::Down => self.density -= 0.1,
      _ => {}
    }
  }
}
