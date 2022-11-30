use geo::Coord;
use rand_distr::{Distribution, Normal};

pub enum CoordType {
  Fixed,
  Slant(f64),
}

pub fn sample_coords(start: Coord, end: Coord, coord_type: CoordType) -> (Coord, Coord) {
  match coord_type {
    CoordType::Fixed => (start, end),
    CoordType::Slant(std_dev) => sample_slant_coords(start, end, std_dev),
  }
}

fn sample_slant_coords(start: Coord, end: Coord, std_dev: f64) -> (Coord, Coord) {
  let mut rng = rand::thread_rng();
  let normal = Normal::new(0f64, std_dev).unwrap();

  let start = start + (normal.sample(&mut rng), normal.sample(&mut rng)).into();
  let end = end + (normal.sample(&mut rng), normal.sample(&mut rng)).into();
  (start, end)
}
