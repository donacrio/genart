use geo::{ChaikinSmoothing, LineString};

use super::{sample_coords, CoordType};

pub enum LineType {
  Straight(usize),
  Wooble(f64, f64),
  Smooth(usize),
}

pub fn sample_line(line: LineString, line_type: LineType) -> LineString {
  match line_type {
    LineType::Straight(n_sample) => sample_straight(line, n_sample),
    LineType::Wooble(std_dev_x, std_dev_y) => sample_wooble(line, std_dev_x, std_dev_y),
    LineType::Smooth(n_iterations) => sample_smooth(line, n_iterations),
  }
}

fn sample_straight(line_string: LineString, n_samples: usize) -> LineString {
  line_string
    .lines()
    .flat_map(|line| {
      let sample_coord = line.delta() / (n_samples as f64);
      (0..=n_samples).map(move |i| line.start + sample_coord * (i as f64))
    })
    .collect()
}

fn sample_wooble(line_string: LineString, std_dev_x: f64, std_dev_y: f64) -> LineString {
  // TODO: unique std_dev and orthogonal sample to line slope
  // Line not wooble enough, use less sample and bigger std_dev for slope
  // the samplke with more points and smaller std_dev
  line_string
    .into_iter()
    .map(|coord| sample_coords(coord, CoordType::Slant(std_dev_x, std_dev_y)))
    .collect()
}

fn sample_smooth(line_string: LineString, n_iterations: usize) -> LineString {
  line_string.chaikin_smoothing(n_iterations)
}
