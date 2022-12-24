use geo::{ChaikinSmoothing, Coord, EuclideanLength, LineString, Rotate};

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
  let total_length = line_string.euclidean_length();
  let mut coords: Vec<Coord> = line_string
    .lines()
    .flat_map(|line| {
      let mut coords = vec![line.start];
      let n_samples = (n_samples as f64 * line.euclidean_length() / total_length).round() as usize;
      if n_samples > 0 {
        let sample_coord = line.delta() / (n_samples + 1) as f64;
        let test = (1..=n_samples).map(|i| line.start + sample_coord * (i as f64));
        coords.extend(test);
      }
      coords
    })
    .collect();
  coords.push(*line_string.coords().last().unwrap());
  coords.into()
}

fn sample_wooble(line_string: LineString, n_samples: usize, std_dev: f64) -> LineString {
  sample_straight(line_string, n_samples)
    .lines()
    .map(|line| {
      let std_vec = line.rotate_around_centroid(90f64);
      let std_dev_x = std_dev * std_vec.dx() / line.euclidean_length();
      let std_dev_y = std_dev * std_vec.dy() / line.euclidean_length();
      sample_coords(line.start, CoordType::Slant(std_dev_x, std_dev_y))
    })
    .collect()
}

fn sample_smooth(line_string: LineString, n_iterations: usize) -> LineString {
  line_string.chaikin_smoothing(n_iterations)
}
