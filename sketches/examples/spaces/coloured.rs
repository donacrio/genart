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
  app::{
    make_static_artwork, update_static, BaseModel, NannouApp, NannouAppOptions, StaticArtwork,
  },
  draw::filling::FillingOptions,
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

impl NannouApp for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      depth: 0,
      weight: 1.0,
      density: 0.5,
      elapsed_frames: 0,
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
  fn update(&mut self, _app: &App) {
    update_static(self);
    self.elapsed_frames += 1;
  }
}

impl StaticArtwork for Model {
  fn draw(&self) {
    let draw = &self.base_model.draw;

    draw.background().color(WHITE);
    let [w_w, w_h] = self.base_model.texture.size();

    let w = w_w as f32 * 0.9;
    let h = w_h as f32 * 0.9;
    let min = (-w / 2.0, -h / 2.0).into();
    let max = (w / 2.0, h / 2.0).into();
    let root = Tile::new(min, max);

    let max_children = 2u32.pow(self.depth);
    let rng = StdRng::seed_from_u64(self.base_model.seed);
    let mut space = utils::algorithm::space::compute_space(root, max_children, MIN_SIZE, rng);
    let leafs = space.leafs();
    leafs.iter().for_each(|index| {
      let tile = space.get_node(*index).unwrap().content();
      let adjusted_rect = Rect::new(
        tile.rect.min() + (10.0, 10.0).into(),
        tile.rect.max() - (10.0, 10.0).into(),
      );
      utils::draw::filling::uniform(
        adjusted_rect.to_polygon(),
        draw,
        FillingOptions {
          weight: self.weight,
          density: self.density,
          color: Hsl::new(28.0, 0.87, 0.67),
        },
      )
    });
  }
}
