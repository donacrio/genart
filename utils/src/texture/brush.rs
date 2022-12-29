use crate::geometry::{sample_line, LineType};
use geo::{EuclideanLength, LineString};

pub enum BrushType {
  Stroke(f64),
  Pencil(usize, f64),
  Sand,
}

pub fn sample_brush(line_string: LineString, brush_type: BrushType) -> LineString {
  match brush_type {
    BrushType::Stroke(width) => stroke(line_string, width),
    BrushType::Pencil(density, width) => pencil(line_string, density, width),
    BrushType::Sand => sand(line_string),
  }
}

fn stroke(line_string: LineString, width: f64) -> LineString {
  const STROKE_FACTOR: f64 = 4f64;
  let line_string = sample_line(line_string, LineType::Wooble(10, 0.004 * width));
  let line_string = sample_line(line_string, LineType::Smooth(3));
  line_string
    .lines()
    .flat_map(|line| {
      let length = line.euclidean_length();
      let n_samples = (STROKE_FACTOR * length) as usize;
      sample_line(line.into(), LineType::Straight(n_samples))
    })
    .collect()
}

fn pencil(line_string: LineString, density: usize, width: f64) -> LineString {
  let line_length = line_string.euclidean_length();
  let line_string = sample_line(line_string, LineType::Wooble(10, 0.004 * line_length));
  let line_string = sample_line(line_string, LineType::Smooth(3));
  sample_line(line_string, LineType::Wooble(density, width))
}

fn sand(_line_string: LineString) -> LineString {
  todo!()
}
