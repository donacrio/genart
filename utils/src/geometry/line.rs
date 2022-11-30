use geo::{Coord, Line, LineString};

pub enum LineType {
  Straight(usize),
}

pub fn sample_line(line: Line, line_type: LineType) -> LineString {
  match line_type {
    LineType::Straight(n_sample) => sample_straight_line(line, n_sample),
  }
}

fn sample_straight_line(line: Line, n_sample: usize) -> LineString {
  let sample_coord = line.delta() / (n_sample as f64);
  (0..n_sample)
    .map(|i| line.start + sample_coord * (i as f64))
    .collect::<Vec<Coord>>()
    .into()
}
