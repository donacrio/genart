use std::path::PathBuf;

use geo::{coord, Rect};
use nannou::{
  prelude::{Hsl, Key, WHITE},
  App,
};
use rand::{rngs::StdRng, SeedableRng};
use rand_distr::{Bernoulli, Distribution};
use utils::app::{make_static_artwork, Artwork, ArtworkOptions, BaseModel, StaticArtwork};

fn main() {
  make_static_artwork::<Model>().run();
}

struct Model {
  base_model: BaseModel,
  depth: usize,
  density: f32,
}

impl Artwork for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      depth: 0,
      density: 0.75,
    }
  }
  fn get_options() -> ArtworkOptions {
    ArtworkOptions {
      background_path: Some(PathBuf::from("paper.jpg")),
      ..ArtworkOptions::default()
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
      Key::Left => self.density -= 0.05,
      Key::Right => self.density += 0.05,
      _ => {}
    }
  }
}

impl StaticArtwork for Model {
  fn draw(&mut self) {
    let draw = &self.base_model.draw;

    draw.background().color(WHITE);

    let [w_w, w_h] = self.base_model.texture.size();

    let rect = Rect::new(
      coord! {x:-(w_w as f32 / 2.), y:-(w_h as f32 / 2.) },
      coord! {x:w_w as f32 / 2., y:w_h as f32 / 2. },
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
      let weight = 0.004 * line_width as f32;
      utils::draw::line::brush(
        start,
        end,
        draw,
        utils::draw::line::LineOptions {
          weight,
          density: self.density,
          color: Hsl::new(0.0, 0.0, 0.0),
        },
      );
    });
  }
}

fn tile(tiles: Vec<Rect<f32>>) -> Vec<Rect<f32>> {
  tiles
    .iter()
    .flat_map(|rect| rect.split_x().map(|x_rect| x_rect.split_y()))
    .flatten()
    .collect()
}
