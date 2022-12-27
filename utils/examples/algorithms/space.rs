use geo::{Coord, Rect};
use nannou::{
  prelude::{Key, BLACK, WHITE},
  App,
};
use utils::{
  algorithms::space::SpaceTile,
  static_artwork::{make_static_nannou_app, StaticArtwork, StaticArtworkOptions, StaticBaseModel},
};

const MIN_SIZE: f64 = 1f64;

fn main() {
  make_static_nannou_app::<Model>().run();
}

struct Tile {
  rect: Rect,
}

impl SpaceTile for Tile {
  fn new(min: Coord, max: Coord) -> Self {
    Tile {
      rect: Rect::new(min, max),
    }
  }

  fn width(&self) -> f64 {
    self.rect.width()
  }

  fn height(&self) -> f64 {
    self.rect.height()
  }

  fn min(&self) -> Coord {
    self.rect.min()
  }

  fn max(&self) -> Coord {
    self.rect.max()
  }
}

struct Model {
  base_model: StaticBaseModel,
  depth: u32,
}

impl StaticArtwork for Model {
  fn new(base_model: StaticBaseModel) -> Self {
    Self {
      base_model,
      depth: 1,
    }
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

    let w = w_w as f64 * 0.9;
    let h = w_h as f64 * 0.9;
    let min: Coord = (-w / 2.0, -h / 2.0).into();
    let max: Coord = (w / 2.0, h / 2.0).into();
    let root = Tile::new(min, max);

    let max_children = 2u32.pow(self.depth);
    let mut space = utils::algorithms::space::compute_space(root, max_children, MIN_SIZE);
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

  fn key_pressed(&mut self, _app: &App, key: Key) {
    match key {
      Key::Up => self.depth += 1,
      Key::Down => self.depth -= 1,
      _ => {}
    }
  }
}
