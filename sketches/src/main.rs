use std::path::PathBuf;

use geo::{coord, LineString, Rect};
use nannou::{
  prelude::{Key, BLACK, WHITE},
  App,
};
use rand::{rngs::StdRng, SeedableRng};
use rand_distr::{Bernoulli, Distribution};
use utils::static_artwork::{
  make_static_nannou_app, StaticArtwork, StaticArtworkOptions, StaticBaseModel,
};

fn main() {
  make_static_nannou_app::<Model>().run();
}

struct Model {
  base_model: StaticBaseModel,
  depth: usize,
}

impl StaticArtwork for Model {
  fn new(base_model: StaticBaseModel) -> Self {
    Self {
      base_model,
      depth: 0,
    }
  }
  fn get_options() -> StaticArtworkOptions {
    StaticArtworkOptions {
      background_path: Some(PathBuf::from("paper.jpg")),
      ..StaticArtworkOptions::default()
    }
  }
  fn get_model(&self) -> &StaticBaseModel {
    &self.base_model
  }
  fn get_model_mut(&mut self) -> &mut StaticBaseModel {
    &mut self.base_model
  }
  fn current_frame_name(&self) -> String {
    format!("frame_{}_{}", self.depth, self.base_model.seed)
  }
  fn draw(&self) {
    let draw = &self.base_model.draw;

    draw.background().color(WHITE);

    let [w_w, w_h] = self.base_model.texture.size();

    let rect = Rect::new(
      coord! {x:-(w_w as f64 / 2f64), y:-(w_h as f64 / 2f64) },
      coord! {x:w_w as f64 / 2f64, y:w_h as f64 / 2f64 },
    );

    let line_width = (rect.width().powi(2) + rect.height().powi(2)).sqrt() * 0.9;
    println!("{line_width}");
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
      let width = 0.004 * line_width;
      let density = 50000 / (self.depth + 1);
      let line_string = LineString::from(vec![start, end]);
      let line_string =
        utils::brush::sample_brush(line_string, utils::brush::BrushType::Pencil(width, density));
      line_string.coords().for_each(|coord| {
        draw
          .ellipse()
          .x_y(coord.x as f32, coord.y as f32)
          .w_h(1f32, 1f32)
          .color(BLACK);
      });
    });
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
}

fn tile(tiles: Vec<Rect>) -> Vec<Rect> {
  tiles
    .iter()
    .flat_map(|rect| rect.split_x().map(|x_rect| x_rect.split_y()))
    .flatten()
    .collect()
}
