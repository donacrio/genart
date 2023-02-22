use std::path::PathBuf;

use geo::Rect;
use nannou::{
  prelude::{Hsl, Key, WHITE},
  App,
};
use rand::{rngs::StdRng, SeedableRng};
use sketches::tile::Tile;
use utils::{
  algorithm::space::SpaceTile,
  app::{make_static_artwork, Artwork, ArtworkOptions, BaseModel, StaticArtwork},
};

const MIN_SIZE: f32 = 100.0;

fn main() {
  make_static_artwork::<Model>().run();
}

struct Model {
  base_model: BaseModel,
  depth: u32,
  weight: f32,
  density: f32,
  elapsed_frames: u32,
}

impl Artwork for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      depth: 0,
      weight: 5.0,
      density: 0.75,
      elapsed_frames: 0,
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
    // format!(
    //   "frame_{}_{}_{}_{}",
    //   self.elapsed_frames,
    //   self.get_base_model().seed,
    //   self.weight,
    //   self.density * 100.,
    // )
    format!("frame_{}", self.elapsed_frames,)
  }
  fn key_pressed(&mut self, _app: &App, key: Key) {
    match key {
      Key::Equals => self.depth += 1,
      Key::Minus => self.depth -= 1,
      Key::Up => self.weight += 1.0,
      Key::Down => self.weight -= 1.0,
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

    let w = w_w as f32 * 0.9;
    let h = w_h as f32 * 0.9;
    let min = (-w / 2.0, -h / 2.0).into();
    let max = (w / 2.0, h / 2.0).into();
    let root = Tile::new(min, max);

    let max_children = 2u32.pow(self.depth);
    let mut rng = StdRng::seed_from_u64(self.base_model.seed);
    let mut space = utils::algorithm::space::compute_space(root, max_children, MIN_SIZE, &mut rng);
    let leafs = space.leafs();
    leafs.iter().for_each(|index| {
      let tile = space.get_node(*index).unwrap().content();
      let adjusted_rect = Rect::new(
        tile.rect.min() + (10.0, 10.0).into(),
        tile.rect.max() - (10.0, 10.0).into(),
      );

      vec![
        (
          adjusted_rect.min(),
          (adjusted_rect.min().x, adjusted_rect.max().y).into(),
        ),
        (
          (adjusted_rect.min().x, adjusted_rect.max().y).into(),
          adjusted_rect.max(),
        ),
        (
          adjusted_rect.max(),
          (adjusted_rect.max().x, adjusted_rect.min().y).into(),
        ),
        (
          (adjusted_rect.max().x, adjusted_rect.min().y).into(),
          adjusted_rect.min(),
        ),
      ]
      .into_iter()
      .for_each(|(start, end)| {
        utils::draw::line::pencil(
          start,
          end,
          draw,
          utils::draw::line::LineOptions {
            weight: self.weight,
            density: self.density,
            color: Hsl::new(0.0, 0.0, 0.0),
          },
        )
      })
    });
    self.elapsed_frames += 1;
  }
}
