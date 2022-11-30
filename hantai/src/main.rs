use nannou::prelude::{pt2, BLACK, WHITE};
use utils::static_artwork::{
  make_static_nannou_app, StaticArtwork, StaticArtworkOptions, StaticBaseModel,
};

fn main() {
  make_static_nannou_app::<Model>().run();
}

struct Model {
  base_model: StaticBaseModel,
}

impl StaticArtwork for Model {
  fn new(base_model: StaticBaseModel) -> Self {
    Self { base_model }
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
  fn draw(&self) {
    let draw = &self.base_model.draw;

    draw.background().color(WHITE);
    let [w_w, _w_h] = self.base_model.texture.size();

    let start = pt2(-(w_w as f32) * 0.95f32 / 2f32, 0f32);
    let end = pt2(w_w as f32 * 0.95f32 / 2f32, 0f32);
    draw
      .line()
      .start(start)
      .end(end)
      .color(BLACK)
      .stroke_weight(5f32);
  }
}
