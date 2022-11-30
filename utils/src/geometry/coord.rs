use geo::Coord;
use rand_distr::{Distribution, Normal};

pub enum CoordType {
  Fixed,
  Slant(f64, f64),
}

pub fn sample_coords(coord: Coord, coord_type: CoordType) -> Coord {
  match coord_type {
    CoordType::Fixed => coord,
    CoordType::Slant(std_dev_x, std_dev_y) => sample_slant_coords(coord, std_dev_x, std_dev_y),
  }
}

fn sample_slant_coords(coord: Coord, std_dev_x: f64, std_dev_y: f64) -> Coord {
  let mut rng = rand::thread_rng();
  let normal_x = Normal::new(0f64, std_dev_x).unwrap();
  let normal_y = Normal::new(0f64, std_dev_y).unwrap();
  coord + (normal_x.sample(&mut rng), normal_y.sample(&mut rng)).into()
}
