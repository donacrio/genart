use std::path::PathBuf;

use geo::{Coord, Line};
use nannou::prelude::{BLACK, WHITE};
use utils::{
  brush::BrushType,
  geometry::{sample_coords, CoordType},
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
    StaticArtworkOptions {
      background_path: Some(PathBuf::from("graph-paper.png")),
      ..StaticArtworkOptions::default()
    }
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

    let brushes = vec![
      BrushType::Stroke(w_h as f64),
      BrushType::Pencil(w_h as f64),
      BrushType::Charcoal(w_h as f64),
      BrushType::Ink(w_h as f64),
      BrushType::Sand(w_h as f64),
    ];
    brushes
      .into_iter()
      .enumerate()
      .map(|(i, brush_type)| {
        let h = ((i + 1) as f64 / 5f64 - 0.5f64) * w_h as f64 / 2f64;
        let start: Coord = (-(w_w as f64) * 0.90f64 / 2f64, h).into();
        let end: Coord = (w_w as f64 * 0.90f64 / 2f64, h).into();
        ((start, end), brush_type)
      })
      .map(|((start, end), brush_type)| {
        (
          Line::new(
            sample_coords(
              start,
              CoordType::Slant(0.01 * w_w as f64, 0.01 * w_h as f64),
            ),
            sample_coords(end, CoordType::Slant(0.01 * w_w as f64, 0.01 * w_h as f64)),
          ),
          brush_type,
        )
      })
      .map(|(line, brush_type)| utils::brush::sample_brush(line, brush_type))
      .for_each(|line| {
        line.coords().for_each(|coord| {
          draw
            .ellipse()
            .x_y(coord.x as f32, coord.y as f32)
            .w_h(1f32, 1f32)
            .color(BLACK);
        })
      });
  }
}
