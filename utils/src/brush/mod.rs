use crate::geometry::{sample_line, LineType};
use geo::{EuclideanLength, LineString};

pub enum BrushType {
  Stroke,
  Pencil,
  Charcoal(f64),
  Ink(f64),
  Sand(f64),
}

pub fn sample_brush(line: LineString, brush_type: BrushType) -> LineString {
  match brush_type {
    BrushType::Stroke => stroke(line),
    BrushType::Pencil => pencil(line),
    BrushType::Charcoal(size) => charcoal(line, size),
    BrushType::Ink(size) => ink(line, size),
    BrushType::Sand(size) => sand(line, size),
  }
}

fn stroke(line_string: LineString) -> LineString {
  const STROKE_FACTOR: f64 = 4f64;
  let len = line_string.euclidean_length();
  let line_string = sample_line(line_string, LineType::Wooble(10, 0f64, 0.004 * len));
  let line_string = sample_line(line_string, LineType::Smooth(3));
  line_string
    .lines()
    .flat_map(|line| {
      let delta = line.delta();
      let length = (delta.x.powi(2) + delta.y.powi(2)).sqrt();
      let n_samples = (STROKE_FACTOR * length) as usize;
      sample_line(line.into(), LineType::Straight(n_samples))
    })
    .collect()
}

fn pencil(line_string: LineString) -> LineString {
  // TODO: No constant parameters
  let len = line_string.euclidean_length();
  let line_string = sample_line(line_string, LineType::Wooble(10, 0f64, 0.004 * len));
  let line_string = sample_line(line_string, LineType::Smooth(3));
  sample_line(line_string, LineType::Wooble(10000, 0f64, 0.0025 * len))
}

fn charcoal(line_string: LineString, size: f64) -> LineString {
  // TODO: No constant parameters
  let base_line = sample_line(line_string, LineType::Wooble(10, 0f64, 0.004 * size));
  //   let base_line = sample_line(base_line, LineType::Smooth(3));
  (0..50)
    .map(|_| sample_line(base_line.clone(), LineType::Wooble(10, 0f64, 0.001 * size)))
    .map(|line| sample_line(line, LineType::Smooth(3)))
    .flat_map(|line| sample_line(line, LineType::Straight(50)))
    .collect()
}

fn sand(line_string: LineString, size: f64) -> LineString {
  let mut base_line_string = sample_line(line_string, LineType::Wooble(10, 0f64, 0.004 * size));
  let mut line_strings = Vec::new();
  for _ in 0..100 {
    let current_line = sample_line(base_line_string, LineType::Wooble(10, 0f64, 0.00075 * size));
    base_line_string = current_line.clone();
    let current_line = sample_line(current_line, LineType::Smooth(2));
    let current_line = sample_line(current_line, LineType::Straight(50));
    line_strings.push(current_line);
  }
  line_strings.into_iter().flatten().collect()
}

fn ink(line_string: LineString, size: f64) -> LineString {
  let mut base_line_string = sample_line(line_string, LineType::Wooble(10, 0f64, 0.004 * size));
  let mut line_strings = Vec::new();
  for _ in 0..50 {
    let current_line = sample_line(base_line_string, LineType::Wooble(10, 0f64, 0.0005 * size));
    base_line_string = current_line.clone();
    let current_line = sample_line(current_line, LineType::Smooth(3));
    let current_line = sample_line(current_line, LineType::Straight(100));
    line_strings.push(current_line);
  }
  line_strings.into_iter().flatten().collect()
}
