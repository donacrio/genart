use geo::{Coord, LinesIter, Rect};
use nannou::{
  prelude::{Hsl, Key, BLACK, WHITE},
  App,
};
use rand::{rngs::StdRng, SeedableRng};
use utils::{
  algorithm::space::SpaceTile,
  app::{
    make_static_artwork, update_static, BaseModel, NannouApp, NannouAppOptions, StaticArtwork,
  },
  draw::line::LineOptions,
};

const MIN_SIZE: f32 = 50.0;

fn main() {
  make_static_artwork::<Model>().run();
}

struct Model {
  base_model: BaseModel,
  depth: u32,
  line_weight: f32,
  line_density: f32,
  filling_weight: f32,
  filling_density: f32,
  hatches_density: f32,
  hatches_degrees: f32,
  elapsed_frames: u32,
}

impl NannouApp for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      depth: 0,
      line_weight: 5.0,
      line_density: 1.0,
      filling_weight: 5.0,
      filling_density: 1.0,
      hatches_density: 0.1,
      hatches_degrees: 60.0,
      elapsed_frames: 0,
    }
  }
  fn get_options() -> NannouAppOptions {
    NannouAppOptions {
      render_size: [1080, 1080],
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
    format!("frame_{}", self.elapsed_frames)
  }
  fn key_pressed(&mut self, _app: &App, key: Key) {
    match key {
      Key::Equals => self.depth += 1,
      Key::Minus => self.depth -= 1,
      Key::Up => self.filling_weight += 1.0,
      Key::Down => self.filling_weight -= 1.0,
      Key::Left => self.filling_density -= 0.01,
      Key::Right => self.filling_density += 0.01,
      Key::W => self.line_weight += 1.0,
      Key::S => self.line_weight -= 1.0,
      Key::A => self.line_density -= 0.05,
      Key::D => self.line_density += 0.05,
      Key::I => self.hatches_density += 0.01,
      Key::K => self.hatches_density -= 0.01,
      Key::J => self.hatches_degrees -= 5.0,
      Key::L => self.hatches_degrees += 5.0,
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
    // let mut rng = StdRng::seed_from_u64(self.base_model.seed);
    let mut rng = StdRng::seed_from_u64(6236303788390788535);
    let mut space = utils::algorithm::space::compute_space(root, max_children, MIN_SIZE, &mut rng);
    let leafs = space.leafs();
    leafs.iter().for_each(|index| {
      let tile = space.get_node(*index).unwrap().content();
      let adjusted_rect = Rect::new(
        tile.rect.min() + (10.0, 10.0).into(),
        tile.rect.max() - (10.0, 10.0).into(),
      );

      adjusted_rect.lines_iter().for_each(|line| {
        utils::draw::line::stroke(
          line.start,
          line.end,
          draw,
          LineOptions {
            weight: self.line_weight,
            density: self.line_density,
            color: Hsl::from(BLACK.into_format()),
          },
        )
      });
      utils::geometry::hatch::hatch(
        adjusted_rect.to_polygon(),
        self.hatches_density,
        self.hatches_degrees,
      )
      .for_each(|(start, end)| {
        utils::draw::line::stroke(
          start,
          end,
          draw,
          LineOptions {
            weight: self.line_weight,
            density: self.line_density,
            color: Hsl::from(BLACK.into_format()),
          },
        )
      });
    });
  }
}

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
