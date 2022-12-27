use geo::{Coord, Rect};
use nannou::{
  prelude::{Key, BLUEVIOLET, WHITE},
  App,
};
use utils::app::{
  make_static_artwork, update_static, BaseModel, NannouApp, NannouAppOptions, StaticApp,
};

fn main() {
  make_static_artwork::<Model>().run();
}

struct Model {
  base_model: BaseModel,
  density: f64,
}

impl NannouApp for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      density: 0.5,
    }
  }
  fn get_options() -> NannouAppOptions {
    NannouAppOptions::default()
  }
  fn get_base_model(&self) -> &BaseModel {
    &self.base_model
  }
  fn get_base_model_mut(&mut self) -> &mut BaseModel {
    &mut self.base_model
  }
  fn current_frame_name(&self) -> String {
    String::from("frame")
  }

  fn key_pressed(&mut self, _app: &App, key: Key) {
    match key {
      Key::Up => self.density += 0.1,
      Key::Down => self.density -= 0.1,
      _ => {}
    }
  }
  fn update(&mut self, _app: &App) {
    update_static(self)
  }
}

impl StaticApp for Model {
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
}
