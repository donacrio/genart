use std::path::PathBuf;

use geo::{Coord, EuclideanLength, LineString};
use nannou::{
  prelude::{Key, BLACK, WHITE},
  App,
};
use utils::app::{
  make_static_artwork, update_static, BaseModel, NannouApp, NannouAppOptions, StaticApp,
};

const N_LINES: usize = 25;

fn main() {
  make_static_artwork::<Model>().run();
}

struct Model {
  base_model: BaseModel,
  depth: usize,
}

impl NannouApp for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      depth: 0,
    }
  }
  fn get_options() -> NannouAppOptions {
    NannouAppOptions {
      background_path: Some(PathBuf::from("paper.jpg")),
      ..NannouAppOptions::default()
    }
  }
  fn get_base_model(&self) -> &BaseModel {
    &self.base_model
  }
  fn get_base_model_mut(&mut self) -> &mut BaseModel {
    &mut self.base_model
  }
  fn current_frame_name(&self) -> String {
    format!("frame_{}_{}", self.depth, self.base_model.seed)
  }

  fn key_pressed(&mut self, _app: &App, key: Key) {
    match key {
      Key::Up => self.depth += 1,
      Key::Down => {
        if self.depth > 0 {
          self.depth -= 1
        }
      }
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

    let line_strings = (0..N_LINES)
      .map(|i| {
        let h = (i as f64 / (N_LINES - 1) as f64 - 0.5f64) * w_h as f64 * 0.8;
        let start: Coord = (-(w_w as f64) * 0.90f64 / 2f64, h).into();
        let end: Coord = (w_w as f64 * 0.90f64 / 2f64, h).into();
        (start, end)
      })
      .map(|(start, end)| LineString::from(vec![start, end]))
      .map(|line_string| {
        utils::geometry::sample_line(line_string, utils::geometry::LineType::Straight(50))
      })
      .map(|line_string| {
        line_string
          .lines()
          .flat_map(|line| {
            let factor =
              2f64 * ((line.start.x / w_w as f64) + 0.5) * ((line.start.y / w_h as f64) - 0.5);
            utils::geometry::sample_line(
              line.into(),
              utils::geometry::LineType::Wooble(1, line.euclidean_length() * factor),
            )
          })
          .collect::<LineString>()
      })
      .map(|line_string| {
        let start = line_string.coords().next().unwrap();
        let factor = 200f64 * ((start.x / w_w as f64) + 0.5) * ((start.y / w_h as f64) - 0.5);
        let width = 0.0001 * line_string.euclidean_length() * factor;
        let density = 50000;
        utils::brush::sample_brush(line_string, utils::brush::BrushType::Pencil(density, width))
      })
      .collect::<Vec<LineString>>();

    line_strings.iter().for_each(|line_string| {
      line_string.coords().for_each(|coord| {
        draw
          .ellipse()
          .x_y(coord.x as f32, coord.y as f32)
          .w_h(1f32, 1f32)
          .color(BLACK);
      })
    });
  }
}
