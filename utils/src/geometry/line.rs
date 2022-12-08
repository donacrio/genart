use geo::{ChaikinSmoothing, EuclideanLength, LineString};

use super::{sample_coords, CoordType};

pub enum LineType {
  Straight(usize),
  Wooble(usize, f64),
  Smooth(usize),
}

pub fn sample_line(line_string: LineString, line_type: LineType) -> LineString {
  match line_type {
    LineType::Straight(n_samples) => sample_straight(line_string, n_samples),
    LineType::Wooble(n_samples, std_dev) => sample_wooble(line_string, n_samples, std_dev),
    LineType::Smooth(n_iterations) => sample_smooth(line_string, n_iterations),
  }
}

fn sample_straight(line_string: LineString, n_samples: usize) -> LineString {
  let total_len = line_string.euclidean_length();
  line_string
    .lines()
    .flat_map(|line| {
      let n_samples = (n_samples as f64 * (line.euclidean_length() / total_len)) as usize + 1;
      let sample_coord = line.delta() / (n_samples as f64);
      (0..=n_samples + 1).map(move |i| line.start + sample_coord * (i as f64))
    })
    .collect()
}

fn sample_wooble(line_string: LineString, n_samples: usize, std_dev: f64) -> LineString {
  sample_straight(line_string, n_samples)
    .lines()
    .map(|line| {
      let slope = line.slope();
      let std_dev_x = std_dev * slope;
      let std_dev_y = std_dev * if slope == 0f64 { 1f64 } else { slope.recip() };
      sample_coords(line.start, CoordType::Slant(std_dev_x, std_dev_y))
    })
    .collect()
}

fn sample_smooth(line_string: LineString, n_iterations: usize) -> LineString {
  line_string.chaikin_smoothing(n_iterations)
}
