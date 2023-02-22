mod tile;

use geo::{Coord, Rect};
use nannou::{
  prelude::{Hsl, Key, BLACK, WHITE},
  App,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rand_distr::{Distribution, Normal, Standard};
use tile::Tile;
use utils::{
  algorithm::space::SpaceTile,
  app::{
    make_static_artwork, update_static, BaseModel, NannouApp, NannouAppOptions, StaticArtwork,
  },
  draw::line::LineOptions,
};

const RECT_MIN_SIZE: f32 = 50.0;
const PADDING: f32 = 10.0;
const HATCH_DENSITY_MEAN: f32 = 0.1;
const HATCH_DENSITY_STD: f32 = 0.05;
// const MAX_BREAKPOINTS_MEAN: f32 = 50.0;
// const MAX_BREAKPOINTS_VARIANCE: f32 = 10.0;
// const BREAK_PROPORTION_MEAN: f64 = 0.8;
// const BREAK_PROPORTION_VARIANCE: f64 = 0.05;
fn main() {
  make_static_artwork::<Model>().run();
}

struct Model {
  base_model: BaseModel,
  depth: u32,
}

impl NannouApp for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      depth: 0,
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
    format!("frame_{}_{}", self.base_model.seed, self.depth)
  }
  fn key_pressed(&mut self, _app: &App, key: Key) {
    match key {
      Key::Equals => self.depth += 1,
      Key::Minus => self.depth -= 1,
      _ => {}
    }
  }
  fn update(&mut self, _app: &App) {
    update_static(self);
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
    let mut space =
      utils::algorithm::space::compute_space(root, max_children, RECT_MIN_SIZE, &mut rng);
    let rects = space
      .leafs()
      .iter_mut()
      .map(|index| space.get_node(*index).unwrap().content())
      .map(|tile| {
        let min = tile.rect.min() + (PADDING, PADDING).into();
        let max = tile.rect.max() - (PADDING, PADDING).into();
        Rect::new(min, max)
      })
      .collect();
    squiggle(rects, &mut rng).iter().for_each(|(start, end)| {
      utils::draw::line::stroke(
        *start,
        *end,
        draw,
        LineOptions {
          weight: 5.0,
          density: 1.0,
          color: Hsl::from(BLACK.into_format()),
        },
      )
    });
  }
}

enum HatchRotation {
  NegFracPi8,
  NegFracPi4,
  NegFrac3Pi8,
  Frac3Pi8,
  FracPi4,
  FracPi8,
}

pub fn squiggle(rects: Vec<Rect<f32>>, rng: &mut StdRng) -> Vec<(Coord<f32>, Coord<f32>)> {
  let mut contours = rects
    .iter()
    .flat_map(|rect| rect.to_lines())
    .map(|line| (line.start, line.end))
    .collect::<Vec<_>>();
  //   let max_breakpoints_law = Normal::new(MAX_BREAKPOINTS_MEAN, MAX_BREAKPOINTS_VARIANCE).unwrap();
  //   let break_proportion_law = Normal::new(BREAK_PROPORTION_MEAN, BREAK_PROPORTION_VARIANCE).unwrap();
  let mut hatches = rects
    .iter()
    .flat_map(|rect| create_hatches(rect, rng))
    .collect::<Vec<_>>();
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
  contours.append(&mut hatches);
  contours
}

fn create_hatches(
  rect: &Rect<f32>,
  rng: &mut StdRng,
) -> impl Iterator<Item = (Coord<f32>, Coord<f32>)> {
  let hatch_density = Normal::new(HATCH_DENSITY_MEAN, HATCH_DENSITY_STD)
    .unwrap()
    .sample(rng);
  // TODO: use custom rng
  let hatch_degrees = rand::random::<HatchRotation>().value();
  utils::geometry::hatch::hatch(rect.to_polygon(), hatch_density, hatch_degrees)
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
