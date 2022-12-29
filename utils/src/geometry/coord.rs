use geo::Coord;
use rand_distr::{Distribution, Normal};

pub fn slant(coord: Coord<f32>, std_dev_x: f32, std_dev_y: f32) -> Coord<f32> {
  let mut rng = rand::thread_rng();
  let normal_x = Normal::new(0.0, std_dev_x).unwrap();
  let normal_y = Normal::new(0.0, std_dev_y).unwrap();
  coord + (normal_x.sample(&mut rng), normal_y.sample(&mut rng)).into()
}
