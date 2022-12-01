use crate::geometry::{sample_line, LineType};
use geo::{Line, LineString};

pub fn bold_pencil(line: Line, size: f64) -> LineString {
  // TODO: No constant parameters
  let line = sample_line(line.into(), LineType::Straight(20));
  let line = sample_line(line, LineType::Wooble(0f64, 0.004 * size));
  let line = sample_line(line, LineType::Smooth(3));
  let line = sample_line(line, LineType::Straight(50));
  sample_line(line, LineType::Wooble(0f64, 0.0025 * size))
}
