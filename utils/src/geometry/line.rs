use geo::{Coord, EuclideanLength, Line, LineInterpolatePoint, Rotate};

use super::coord::slant;

pub fn sample_straight(
  start: Coord<f32>,
  end: Coord<f32>,
  n_samples: usize,
) -> impl Iterator<Item = Coord<f32>> {
  let line = Line::new(start, end);
  (0..=n_samples)
    .map(move |i| line.line_interpolate_point(i as f32 / n_samples as f32))
    .filter(|point| point.is_some())
    .map(|point| point.unwrap().into())
}

pub fn sample_wooble(
  start: Coord<f32>,
  end: Coord<f32>,
  n_samples: usize,
  std_dev: f32,
) -> impl Iterator<Item = Coord<f32>> {
  let line = Line::new(start, end);
  let std_vec = line.rotate_around_centroid(90.0);
  let std_dev_x = std_dev * std_vec.dx() / line.euclidean_length();
  let std_dev_y = std_dev * std_vec.dy() / line.euclidean_length();
  sample_straight(start, end, n_samples).map(move |coord| slant(coord, std_dev_x, std_dev_y))
}
