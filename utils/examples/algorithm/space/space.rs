use geo::{Coord, Rect};
use nannou::{
  prelude::{Key, BLACK, WHITE},
  App,
};
use rand::{rngs::StdRng, SeedableRng};
use utils::{
  algorithm::space::SpaceTile,
  app::{
    make_static_artwork, update_static, BaseModel, NannouApp, NannouAppOptions, StaticArtwork,
  },
};

const MIN_SIZE: f32 = 1.0;

fn main() {
  make_static_artwork::<Model>().run();
}

struct Tile {
  pub rect: Rect<f32>,
}

impl SpaceTile for Tile {
  fn new(min: Coord<f32>, max: Coord<f32>) -> Self {
    Tile {
      rect: Rect::new(min, max),
    }
  }

  fn width(&self) -> f32 {
    self.rect.width()
  }

  fn height(&self) -> f32 {
    self.rect.height()
  }

  fn min(&self) -> Coord<f32> {
    self.rect.min()
  }

  fn max(&self) -> Coord<f32> {
    self.rect.max()
  }
}

struct Model {
  base_model: BaseModel,
  depth: u32,
}

impl NannouApp for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      depth: 1,
    }
  }
  fn get_options() -> NannouAppOptions {
    NannouAppOptions::default()
  }
  fn get_base_model(&self) -> &BaseModel {
    &self.base_model
  }
  fn get_base_model_mut(&mut self) -> &mut BaseModel {
    &mut self.base_model
  }
  fn current_frame_name(&self) -> String {
    String::from("frame")
  }
  fn key_pressed(&mut self, _app: &App, key: Key) {
    match key {
      Key::Up => self.depth += 1,
      Key::Down => self.depth -= 1,
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
      let center = tile.rect.center();
      draw
        .rect()
        .x_y(center.x as f32, center.y as f32)
        .w_h(tile.width() as f32 - 10.0, tile.height() as f32 - 10.0)
        .color(BLACK);
    });
  }
}
