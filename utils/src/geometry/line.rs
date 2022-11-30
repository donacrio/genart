use geo::{Coord, Line, LineString};

use super::{sample_coords, CoordType};

pub enum LineType {
  Straight(usize),
  Wooble(usize, f64, f64),
}

pub fn sample_line(line: Line, line_type: LineType) -> LineString {
  match line_type {
    LineType::Straight(n_sample) => sample_straight_line(line, n_sample),
    LineType::Wooble(n_sample, std_dev_x, std_dev_y) => {
      sample_wooble_line(line, n_sample, std_dev_x, std_dev_y)
    }
  }
}

fn sample_straight_line(line: Line, n_sample: usize) -> LineString {
  let sample_coord = line.delta() / (n_sample as f64);
  (0..n_sample)
    .map(|i| line.start + sample_coord * (i as f64))
    .collect::<Vec<Coord>>()
    .into()
}

fn sample_wooble_line(line: Line, n_sample: usize, std_dev_x: f64, std_dev_y: f64) -> LineString {
  sample_straight_line(line, n_sample)
    .into_iter()
    .map(|coord| sample_coords(coord, CoordType::Slant(std_dev_x, std_dev_y)))
    .collect::<Vec<Coord>>()
    .into()
}
