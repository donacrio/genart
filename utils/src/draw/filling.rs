use crate::{draw, geometry};
use geo::{
  line_intersection::line_intersection, BoundingRect, Coord, Line, LinesIter, Point, Polygon, Rect,
  Rotate,
};
use nannou::{
  prelude::{Hsl, Vec2},
  Draw,
};

pub struct FillingOptions {
  pub weight: f32,
  pub density: f32,
  pub color: Hsl,
}

pub fn uniform(polygon: Polygon<f32>, draw: &Draw, options: FillingOptions) {
  geometry::filling::uniform(polygon, options.density).for_each(|coord| {
    draw
      .ellipse()
      .xy(Vec2::from(coord.x_y()))
      .w_h(options.weight, options.weight)
      .color(options.color);
  });
}

pub fn halton_23(polygon: Polygon<f32>, draw: &Draw, options: FillingOptions) {
  geometry::filling::halton_23(polygon, options.density).for_each(|coord| {
    draw
      .ellipse()
      .xy(Vec2::from(coord.x_y()))
      .w_h(options.weight, options.weight)
      .color(options.color);
  });
}

pub fn japanese_brush(polygon: Polygon<f32>, draw: &Draw, degrees: f32, options: FillingOptions) {
  // TODO: handle degrees = 0.0
  // Compute rectangle bounding the polygon for any rotation
  let bounding_rect = polygon.bounding_rect().unwrap();
  let diagonal = (bounding_rect.width().powi(2) * bounding_rect.height().powi(2)).sqrt();
  let max = bounding_rect.center() - Coord::from((diagonal, diagonal)) / 2.0;
  let min = bounding_rect.center() + Coord::from((diagonal, diagonal)) / 2.0;
  let bounding_rect = Rect::new(min, max);
  let n_lines = (bounding_rect.height() / options.weight) as usize;
  (0..n_lines)
    .map(|i| {
      let height = (i as f32 / n_lines as f32 - 0.5) * bounding_rect.height();
      let start: Point<f32> = (bounding_rect.min().x, height).into();
      let end: Point<f32> = (bounding_rect.max().x, height).into();
      let line = Line::new(start, end).rotate_around_centroid(degrees);
      let direction = line.dx();
      let mut intersections = polygon
        .lines_iter()
        .filter_map(|polygon_line| line_intersection(polygon_line, line))
        .filter_map(|intersection| match intersection {
          geo::LineIntersection::SinglePoint {
            intersection,
            is_proper: _,
          } => Some(intersection),
          _ => None,
        })
        .collect::<Vec<_>>();
      intersections.sort_by(|a, b| {
        if direction > 0.0 {
          a.x.total_cmp(&b.x)
        } else {
          b.x.total_cmp(&a.x)
        }
      });
      intersections
    })
    .for_each(|intersections| {
      intersections
        .iter()
        .step_by(2)
        .zip(intersections.iter().skip(1).step_by(2))
        .for_each(|(start, end)| {
          draw::line::japanese_brush(
            *start,
            *end,
            draw,
            draw::line::LineOptions {
              weight: options.weight,
              density: options.density,
              color: options.color,
            },
          )
        })
    })
}

// use https://docs.rs/geo/latest/geo/algorithm/sweep/struct.Intersections.html
// compute direction vector of the line then order using that direction
// Use dx sign for ordering
