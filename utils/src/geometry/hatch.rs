use geo::{
  line_intersection::line_intersection, BoundingRect, Coord, Line, LinesIter, Point, Polygon, Rect,
  Rotate,
};

// TODO: refactor the function
// - improve performances (no collect on iterator)
pub fn hatch(
  polygon: Polygon<f32>,
  density: f32,
  degrees: f32,
) -> impl Iterator<Item = (Coord<f32>, Coord<f32>)> {
  // Create bounding square around the polygon.
  // We will create horizontal hatches on this square then rotate them.
  // Square must be at least 2 times the polygon bounding rectangle diagonal length.
  let bounding_rect = polygon.bounding_rect().unwrap();
  let diagonal = (bounding_rect.width().powi(2) + bounding_rect.height().powi(2)).sqrt();
  let max = bounding_rect.center() + Coord::from((diagonal, diagonal));
  let min = bounding_rect.center() - Coord::from((diagonal, diagonal));
  let bounding_rect = Rect::new(min, max);
  let density = density.clamp(0.0, 1.0);
  let n_lines = (bounding_rect.height() * density) as usize;
  // We create n horizontal lines on the bounding square
  (0..n_lines).flat_map(move |i| {
    // Create an horizontal line on the bounding square
    let height =
      (i as f32 / (n_lines - 1) as f32 - 0.5) * bounding_rect.height() + bounding_rect.center().y;
    let start: Point<f32> = (bounding_rect.min().x, height).into();
    let end: Point<f32> = (bounding_rect.max().x, height).into();
    let line = Line::new(start, end);
    // Rotate the line aroung the polygon center with the given angle
    let line = line.rotate_around_point(degrees, bounding_rect.center().into());
    // Find all the intersections between the line and the polygon
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
    // We sort the intersections using the line slope.
    // That will allow us to gather points into pairs to create hatches
    intersections.sort_by(|a, b| {
      if direction > 0.0 {
        a.x.total_cmp(&b.x)
      } else {
        b.x.total_cmp(&a.x)
      }
    });
    // Gather points into lines
    intersections
      .iter()
      .step_by(2)
      .zip(intersections.iter().skip(1).step_by(2))
      .map(|(a, b)| (*a, *b))
      .collect::<Vec<_>>()
  })
}
