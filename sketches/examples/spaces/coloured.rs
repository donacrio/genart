use std::path::PathBuf;

use geo::{Coord, Rect};
use nannou::{
  prelude::{Key, SANDYBROWN, WHITE},
  App,
};
use utils::{
  algorithms::space::SpaceTile,
  app::{
    make_static_artwork, update_static, BaseModel, NannouApp, NannouAppOptions, StaticArtwork,
  },
};

const MIN_SIZE: f64 = 100.;

fn main() {
  make_static_artwork::<Model>().run();
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
  base_model: BaseModel,
  depth: u32,
  density: f64,
}

impl NannouApp for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      depth: 1,
      density: 0.5,
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
    String::from("frame")
  }
  fn key_pressed(&mut self, _app: &App, key: Key) {
    match key {
      Key::Up => self.depth += 1,
      Key::Down => self.depth -= 1,
      Key::Left => self.density -= 0.1,
      Key::Right => self.density += 0.1,
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
      let adjusted_rect = Rect::new(
        tile.rect.min() + (10.0, 10.0).into(),
        tile.rect.max() - (10.0, 10.0).into(),
      );
      utils::paint::fill_rectangle(adjusted_rect, self.density).for_each(|p| {
        draw
          .ellipse()
          .x_y(p.x() as f32, p.y() as f32)
          .w_h(1.0, 1.0)
          .color(SANDYBROWN);
      });
    });
  }
}
