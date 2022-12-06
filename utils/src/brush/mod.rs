use crate::geometry::{sample_line, LineType};
use geo::{Line, LineString};

pub fn stroke(line: Line, size: f64) -> LineString {
  // TODO: No constant parameters
  let line = sample_line(line.into(), LineType::Straight(20));
  let line = sample_line(line, LineType::Wooble(0f64, 0.004 * size));
  let line = sample_line(line, LineType::Smooth(3));
  sample_line(line, LineType::Straight(50))
}

pub fn pencil(line: Line, size: f64) -> LineString {
  // TODO: No constant parameters
  let line = sample_line(line.into(), LineType::Straight(10));
  let line = sample_line(line, LineType::Wooble(0f64, 0.004 * size));
  let line = sample_line(line, LineType::Smooth(3));
  let line = sample_line(line, LineType::Straight(500));
  sample_line(line, LineType::Wooble(0f64, 0.0025 * size))
}

pub fn charcoal(line: Line, size: f64) -> LineString {
  // point width: 0.5
  let base_line = sample_line(line.into(), LineType::Straight(10));
  let base_line = sample_line(base_line, LineType::Wooble(0f64, 0.004 * size));
  //   let base_line = sample_line(base_line, LineType::Smooth(3));
  (0..50)
    .map(|_| sample_line(base_line.clone(), LineType::Wooble(0f64, 0.001 * size)))
    .map(|line| sample_line(line, LineType::Smooth(3)))
    .flat_map(|line| sample_line(line, LineType::Straight(50)))
    .collect()
}

pub fn sand(line: Line, size: f64) -> LineString {
  let line = sample_line(line.into(), LineType::Straight(10));
  let mut line = sample_line(line, LineType::Wooble(0f64, 0.004 * size));
  let mut lines = Vec::new();
  for _ in 0..100 {
    let current_line = sample_line(line, LineType::Wooble(0f64, 0.00075 * size));
    line = current_line.clone();
    let current_line = sample_line(current_line, LineType::Smooth(2));
    let current_line = sample_line(current_line, LineType::Straight(50));
    lines.push(current_line);
  }
  lines.into_iter().flatten().collect()
}

pub fn ink(line: Line, size: f64) -> LineString {
  let line = sample_line(line.into(), LineType::Straight(10));
  let mut line = sample_line(line, LineType::Wooble(0f64, 0.004 * size));
  let mut lines = Vec::new();
  for _ in 0..50 {
    let current_line = sample_line(line, LineType::Wooble(0f64, 0.0005 * size));
    line = current_line.clone();
    let current_line = sample_line(current_line, LineType::Smooth(3));
    let current_line = sample_line(current_line, LineType::Straight(100));
    lines.push(current_line);
  }
  lines.into_iter().flatten().collect()
}
