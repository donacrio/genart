use crate::geometry::line::sample_wooble;
use geo::{ConvexHull, Coord, EuclideanDistance, Line, LineInterpolatePoint, LineString};
use nannou::{
  prelude::{Hsl, Hsla, Vec2, PI},
  Draw,
};
use rand_distr::{Distribution, Uniform};

pub struct LineOptions {
  pub weight: f32,
  pub density: f32,
  pub color: Hsl,
}

pub fn stroke(start: Coord<f32>, end: Coord<f32>, draw: &Draw, options: LineOptions) {
  let start = Vec2::from(start.x_y());
  let end = Vec2::from(end.x_y());
  let color = Hsla::new(
    options.color.hue,
    options.color.saturation,
    options.color.lightness,
    options.density.clamp(0.0, 1.0),
  );
  draw
    .line()
    .start(start)
    .end(end)
    .stroke_weight(options.weight)
    .color(color);
}

pub fn brush(start: Coord<f32>, end: Coord<f32>, draw: &Draw, options: LineOptions) {
  const N_LINES: u32 = 50;
  let stroke_weight = 10. * options.weight / N_LINES as f32;
  let color = Hsla::new(
    options.color.hue,
    options.color.saturation,
    options.color.lightness,
    options.density.clamp(0.0, 1.0),
  );
  // Generate points in the circle centered on start and end with radius equal to weight
  // Then draw a line between those points
  (0..N_LINES)
    .map(|_| {
      let start = sample_within_circle(&start, options.weight);
      let end = sample_within_circle(&end, options.weight);
      (Vec2::from(start.x_y()), Vec2::from(end.x_y()))
    })
    .for_each(|(start, end)| {
      draw
        .line()
        .start(start)
        .end(end)
        .stroke_weight(stroke_weight)
        .color(color);
    })
}

pub fn pencil(start: Coord<f32>, end: Coord<f32>, draw: &Draw, options: LineOptions) {
  const STROKE_WEIGHT: f32 = 5.0;
  let n_lines = 10.0 * options.density * options.weight;
  // Generate points in the circle centered on start and end with radius equal to weight
  // Then draw a line between those points
  (0..n_lines as u32)
    .map(|_| {
      let start = sample_within_circle(&start, options.weight);
      let end = sample_within_circle(&end, options.weight);
      Line::new(start, end)
    })
    .for_each(|line| {
      let n_points = (options.density * start.euclidean_distance(&end)) as usize / 50;
      Uniform::new(0.0, 1.0)
        .sample_iter(&mut rand::thread_rng())
        .map(|t| line.line_interpolate_point(t))
        .filter(|point| point.is_some())
        .map(|point| Vec2::from(point.unwrap().x_y()))
        .take(n_points)
        .for_each(|point| {
          draw
            .ellipse()
            .xy(point)
            .w_h(STROKE_WEIGHT, STROKE_WEIGHT)
            .color(options.color);
        })
    })
}

pub fn marker(start: Coord<f32>, end: Coord<f32>, draw: &Draw, options: LineOptions) {
  let color = Hsla::new(
    options.color.hue,
    options.color.saturation,
    options.color.lightness,
    options.density.clamp(0.0, 1.0),
  );
  let n_samples = (options.weight * start.euclidean_distance(&end)) as usize / 50;
  let std_dev = 0.5 * options.weight;
  let line_string = sample_wooble(start, end, n_samples, std_dev).collect::<LineString<f32>>();
  let polygon = line_string.convex_hull();
  let points = polygon
    .exterior()
    .coords()
    .map(|coord| (Vec2::new(coord.x, coord.y), color));
  draw.polygon().points_colored(points);
}

fn sample_within_circle(center: &Coord<f32>, radius: f32) -> Coord<f32> {
  let r = radius * rand::random::<f32>().sqrt();
  let theta = 2. * PI * rand::random::<f32>();
  *center + (r * theta.cos(), r * theta.sin()).into()
}
