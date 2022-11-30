use geo::{Coord, Line, LineString};
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
    let [w_w, w_h] = self.base_model.texture.size();

    let lines: Vec<LineString> = (0..5)
      .map(|i| {
        let h = (i as f64 / 5f64 - 0.5f64) * w_h as f64 / 2f64;
        let start: Coord = (-(w_w as f64) * 0.90f64 / 2f64, h).into();
        let start = sample_coords(
          start,
          CoordType::Slant(0.01 * w_w as f64, 0.01 * w_h as f64),
        );
        let end: Coord = (w_w as f64 * 0.90f64 / 2f64, h).into();
        let end = sample_coords(end, CoordType::Slant(0.01 * w_w as f64, 0.01 * w_h as f64));

        sample_line(
          Line::new(start, end),
          LineType::Wooble(5000, 0.001 * w_w as f64, 0.001 * w_h as f64),
        )
      })
      .collect();

    lines.iter().for_each(|line| {
      line.coords().for_each(|coord| {
        draw
          .ellipse()
          .x_y(coord.x as f32, coord.y as f32)
          .w_h(3f32, 3f32)
          .color(BLACK);
      })
    });
  }
}
