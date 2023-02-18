use geo::{Coord, Rect};
use utils::algorithm::space::SpaceTile;

// Utilities for examples at examples/spaces
pub struct Tile {
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
