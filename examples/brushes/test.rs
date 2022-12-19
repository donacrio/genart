use geo::{Coord, EuclideanLength, LineString};
use nannou::{
  prelude::{Key, BLACK, WHITE},
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
  fn current_frame_name(&self) -> String {
    String::from("frame")
  }
  fn draw(&self) {
    let draw = &self.base_model.draw;

    draw.background().color(WHITE);
    let [w_w, w_h] = self.base_model.texture.size();

    let straight_line = {
      let h = (0f64 as f64 / 4f64 - 0.5f64) * w_h as f64 * 0.8;
      let start: Coord = (-(w_w as f64) * 0.90f64 / 2f64, h).into();
      let end: Coord = (w_w as f64 * 0.90f64 / 2f64, h).into();
      let line_string = LineString::from(vec![start, end]);
      utils::geometry::sample_line(line_string, utils::geometry::LineType::Straight(1))
    };

    let straight_line_bis = {
      let h = (1f64 as f64 / 4f64 - 0.5f64) * w_h as f64 * 0.8;
      let start: Coord = (-(w_w as f64) * 0.90f64 / 2f64, h).into();
      let end: Coord = (w_w as f64 * 0.90f64 / 2f64, h).into();
      let line_string = LineString::from(vec![start, end]);
      let line_string =
        utils::geometry::sample_line(line_string, utils::geometry::LineType::Straight(10));
      utils::geometry::sample_line(line_string, utils::geometry::LineType::Straight(500))
    };

    let wooble_line = {
      let h = (2f64 as f64 / 4f64 - 0.5f64) * w_h as f64 * 0.8;
      let start: Coord = (-(w_w as f64) * 0.90f64 / 2f64, h).into();
      let end: Coord = (w_w as f64 * 0.90f64 / 2f64, h).into();
      let line_string = LineString::from(vec![start, end]);
      let line_string =
        utils::geometry::sample_line(line_string, utils::geometry::LineType::Straight(10));
      let width = 0.01 * line_string.euclidean_length();

      utils::geometry::sample_line(line_string, utils::geometry::LineType::Wooble(1, width))
    };

    let wooble_line_bis = {
      let h = (3f64 as f64 / 4f64 - 0.5f64) * w_h as f64 * 0.8;
      let start: Coord = (-(w_w as f64) * 0.90f64 / 2f64, h).into();
      let end: Coord = (w_w as f64 * 0.90f64 / 2f64, h).into();
      let line_string = LineString::from(vec![start, end]);
      let line_string =
        utils::geometry::sample_line(line_string, utils::geometry::LineType::Straight(10));
      let width = 0.01 * line_string.euclidean_length();
      let line_string =
        utils::geometry::sample_line(line_string, utils::geometry::LineType::Wooble(1, width));
      utils::geometry::sample_line(line_string, utils::geometry::LineType::Straight(500))
    };

    straight_line.coords().for_each(|coord| {
      draw
        .ellipse()
        .x_y(coord.x as f32, coord.y as f32)
        .w_h(10f32, 10f32)
        .color(BLACK);
    });
    straight_line_bis.coords().for_each(|coord| {
      draw
        .ellipse()
        .x_y(coord.x as f32, coord.y as f32)
        .w_h(10f32, 10f32)
        .color(BLACK);
    });
    wooble_line.coords().for_each(|coord| {
      draw
        .ellipse()
        .x_y(coord.x as f32, coord.y as f32)
        .w_h(10f32, 10f32)
        .color(BLACK);
    });
    wooble_line_bis.coords().for_each(|coord| {
      draw
        .ellipse()
        .x_y(coord.x as f32, coord.y as f32)
        .w_h(10f32, 10f32)
        .color(BLACK);
    });
  }

  fn key_pressed(&mut self, _app: &App, _key: Key) {}
}
