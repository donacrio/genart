use std::path::PathBuf;

use geo::{Coord, Line};
use nannou::prelude::{BLACK, WHITE};
use utils::{
  brush::BrushType,
  dynamic_artwork::{
    make_dynamic_nannou_app, DynamicArtwork, DynamicArtworkOptions, DynamicBaseModel,
  },
  geometry::{sample_coords, CoordType},
};

fn main() {
  make_dynamic_nannou_app::<Model>().run();
}

struct Model {
  base_model: DynamicBaseModel,
}

impl DynamicArtwork for Model {
  fn new(base_model: DynamicBaseModel) -> Self {
    Self { base_model }
  }
  fn get_options() -> DynamicArtworkOptions {
    DynamicArtworkOptions {
      background_path: Some(PathBuf::from("graph-paper.png")),
      ..DynamicArtworkOptions::default()
    }
  }
  fn get_model(&self) -> &DynamicBaseModel {
    &self.base_model
  }
  fn get_model_mut(&mut self) -> &mut DynamicBaseModel {
    &mut self.base_model
  }
  fn draw_at_time(&self, _t: f64) {
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
        let h = (i as f64 / 5f64 - 0.5f64) * w_h as f64 / 2f64;
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
