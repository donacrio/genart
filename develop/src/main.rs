use geo::{Coord, Line};
use nannou::prelude::{BLACK, WHITE};
use utils::{
  geometry::{sample_coords, sample_line, CoordType, LineType},
  static_artwork::{make_static_nannou_app, StaticArtwork, StaticArtworkOptions, StaticBaseModel},
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

    let start: Coord = (-(w_w as f64) * 0.95f64 / 2f64, 0f64).into();
    let end: Coord = (w_w as f64 * 0.95f64 / 2f64, 0f64).into();
    let (start, end) = sample_coords(start, end, CoordType::Slant(0.05));
    let line = sample_line(Line::new(start, end), LineType::Straight(100));

    line.coords().for_each(|coord| {
      draw
        .ellipse()
        .x_y(coord.x as f32, coord.y as f32)
        .w_h(10f32, 10f32)
        .color(BLACK);
    });
  }
}
