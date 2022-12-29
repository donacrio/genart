use std::path::PathBuf;

use geo::{coord, LineString, Rect};
use nannou::{
  prelude::{Key, BLACK, WHITE},
  App,
};
use rand::{rngs::StdRng, SeedableRng};
use rand_distr::{Bernoulli, Distribution};
use utils::app::{
  make_static_artwork, update_static, BaseModel, NannouApp, NannouAppOptions, StaticArtwork,
};

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

impl StaticArtwork for Model {
  fn draw(&self) {
    let draw = &self.base_model.draw;

    draw.background().color(WHITE);

    let [w_w, w_h] = self.base_model.texture.size();

    let rect = Rect::new(
      coord! {x:-(w_w as f64 / 2f64), y:-(w_h as f64 / 2f64) },
      coord! {x:w_w as f64 / 2f64, y:w_h as f64 / 2f64 },
    );

    let line_width = (rect.width().powi(2) + rect.height().powi(2)).sqrt() * 0.9;
    let mut tiles = vec![rect];
    for _ in 0..self.depth {
      tiles = tile(tiles);
    }
    let mut rng = StdRng::seed_from_u64(self.base_model.seed);
    tiles.iter().for_each(|tile| {
      let axis = Bernoulli::new(0.5).unwrap().sample(&mut rng);
      let (start, end) = match axis {
        true => (tile.min(), tile.max()),
        false => (
          (tile.min().x, tile.max().y).into(),
          (tile.max().x, tile.min().y).into(),
        ),
      };
      let density = 50000 / (self.depth + 1);
      let width = 0.004 * line_width;
      let line_string = LineString::from(vec![start, end]);
      let line_string =
        utils::brush::sample_brush(line_string, utils::brush::BrushType::Pencil(density, width));
      line_string.coords().for_each(|coord| {
        draw
          .ellipse()
          .x_y(coord.x as f32, coord.y as f32)
          .w_h(1f32, 1f32)
          .color(BLACK);
      });
    });
  }
}

fn tile(tiles: Vec<Rect>) -> Vec<Rect> {
  tiles
    .iter()
    .flat_map(|rect| rect.split_x().map(|x_rect| x_rect.split_y()))
    .flatten()
    .collect()
}
