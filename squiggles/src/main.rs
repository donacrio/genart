mod tile;

use geo::{Coord, LinesIter, Rect};
use nannou::{
  prelude::{Hsl, Key, WHITE},
  App,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rand_distr::{Distribution, Normal, Standard};
use tile::Tile;
use utils::{
  algorithm::space::{Space, SpaceTile},
  app::{
    make_static_artwork, update_static, BaseModel, NannouApp, NannouAppOptions, StaticArtwork,
  },
  draw::line::LineOptions,
  geometry::hatch::hatch,
};

const MIN_SIZE: f32 = 50.0;
const COLOR_PALETTE: [[f32; 3]; 5] = [
  [201.0, 1.0, 0.14],
  [0.0, 0.69, 0.5],
  [31.0, 1.0, 0.48],
  [40.0, 0.97, 0.64],
  [51.0, 0.55, 0.82],
];

fn hsl_from_palette(color: [f32; 3]) -> Hsl {
  Hsl::new(color[0], color[1], color[2])
}

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
      line_density: 10.0,
      filling_weight: 5.0,
      filling_density: 1.0,
      hatches_density: 0.5,
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
      Key::I => self.hatches_density += 0.05,
      Key::K => self.hatches_density -= 0.05,
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
    let mut rng = StdRng::seed_from_u64(self.base_model.seed);
    let mut space = utils::algorithm::space::compute_space(root, max_children, MIN_SIZE, &mut rng);
    let leafs = space.leafs();
    leafs.iter().for_each(|index| {
      let tile = space.get_node(*index).unwrap().content();
      let adjusted_rect = Rect::new(
        tile.rect.min() + (10.0, 10.0).into(),
        tile.rect.max() - (10.0, 10.0).into(),
      );
      let index = rng.gen_range(0..COLOR_PALETTE.len());
      let color = hsl_from_palette(COLOR_PALETTE[index]);

      adjusted_rect.lines_iter().for_each(|line| {
        utils::draw::line::stroke(
          line.start,
          line.end,
          draw,
          LineOptions {
            weight: self.line_weight,
            density: self.line_density,
            color,
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
            color,
          },
        )
      });
    });
  }
}

// const PADDING: f32 = 50.0;
const HATCH_WEIGHT_MEAN: f32 = 40.0;
const HATCH_WEIGHT_STD: f32 = 10.0;
// const MAX_BREAKPOINTS_MEAN: f32 = 50.0;
// const MAX_BREAKPOINTS_VARIANCE: f32 = 10.0;
// const BREAK_PROPORTION_MEAN: f64 = 0.8;
// const BREAK_PROPORTION_VARIANCE: f64 = 0.05;

enum HatchRotation {
  NegFracPi8,
  NegFracPi4,
  NegFrac3Pi8,
  Frac3Pi8,
  FracPi4,
  FracPi8,
}

pub fn squiggle(space: &mut Space<Rect<f32>>, rng: &mut StdRng) -> Vec<(Coord<f32>, Coord<f32>)> {
  //   let max_breakpoints_law = Normal::new(MAX_BREAKPOINTS_MEAN, MAX_BREAKPOINTS_VARIANCE).unwrap();
  //   let break_proportion_law = Normal::new(BREAK_PROPORTION_MEAN, BREAK_PROPORTION_VARIANCE).unwrap();
  create_hatches(space, rng)
  // .iter()
  // .map(|hatches| {
  //   let max_breakpoint = max_breakpoints_law.sample(&mut rand::thread_rng()) as usize;
  //   let break_proportion = break_proportion_law.sample(&mut rand::thread_rng());
  //   hatches.iter().map(move |hatch| {
  //     BrokenLineBuilder::new(&hatch)
  //       .max_breakpoints(max_breakpoint)
  //       .break_proportion(break_proportion)
  //       .build()
  //       .segments
  //   })
  // })
  // .flatten()
  // .flatten()
  // .collect()
}

fn create_hatches(space: &mut Space<Rect<f32>>, rng: &mut StdRng) -> Vec<(Coord<f32>, Coord<f32>)> {
  space
    .leafs()
    .iter()
    .map(|index| space.get_node(*index).unwrap().content())
    .flat_map(|rectangle| {
      // IDEA: increment & rotation depending on rectangle size
      let hatch_weight = Normal::new(HATCH_WEIGHT_MEAN, HATCH_WEIGHT_STD)
        .unwrap()
        .sample(rng);
      let hatch_degrees = rand::random::<HatchRotation>().value();
      hatch(rectangle.to_polygon(), hatch_weight, hatch_degrees)
    })
    .collect()
}

impl HatchRotation {
  fn value(&self) -> f32 {
    match self {
      Self::NegFracPi8 => -std::f32::consts::FRAC_PI_8,
      Self::NegFracPi4 => -std::f32::consts::FRAC_PI_4,
      Self::NegFrac3Pi8 => -(std::f32::consts::FRAC_PI_8 + std::f32::consts::FRAC_PI_4),
      Self::Frac3Pi8 => std::f32::consts::FRAC_PI_8 + std::f32::consts::FRAC_PI_4,
      Self::FracPi4 => std::f32::consts::FRAC_PI_4,
      Self::FracPi8 => std::f32::consts::FRAC_PI_8,
    }
  }
}

impl Distribution<HatchRotation> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HatchRotation {
    match rng.gen_range(0..=5) {
      0 => HatchRotation::NegFracPi8,
      1 => HatchRotation::NegFracPi4,
      2 => HatchRotation::NegFrac3Pi8,
      3 => HatchRotation::Frac3Pi8,
      4 => HatchRotation::FracPi4,
      _ => HatchRotation::FracPi8,
    }
  }
}
