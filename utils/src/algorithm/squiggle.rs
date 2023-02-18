use super::space::Space;
use crate::geometry;
use geo::{Coord, Rect};
use rand::{rngs::StdRng, Rng};
use rand_distr::{Distribution, Normal, Standard};

// const PADDING: f32 = 50.0;
const HATCH_DENSITY_MEAN: f32 = 40.0;
const HATCH_DENSITY_STD: f32 = 10.0;
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
      let hatch_density = Normal::new(HATCH_DENSITY_MEAN, HATCH_DENSITY_STD)
        .unwrap()
        .sample(rng);
      let hatch_degrees = rand::random::<HatchRotation>().value();
      geometry::hatch::hatch(rectangle.to_polygon(), hatch_density, hatch_degrees)
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
