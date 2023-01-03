use geo::{BoundingRect, Coord, Intersects, Polygon, Rect};
use rand_distr::{Distribution, Uniform};

pub fn uniform(polygon: Polygon<f32>, density: f32) -> impl Iterator<Item = Coord<f32>> {
  let bounding_rect = polygon.bounding_rect().unwrap();
  let density = density.clamp(0.0, 1.0);
  let n_points = (bounding_rect.width() * bounding_rect.height() * density) as usize;
  uniform_rectangle(bounding_rect)
    .filter(move |coord| coord.intersects(&polygon))
    .take(n_points)
}

fn uniform_rectangle(rectangle: Rect<f32>) -> impl Iterator<Item = Coord<f32>> {
  let uniform_x = Uniform::new(rectangle.min().x, rectangle.max().x);
  let uniform_y = Uniform::new(rectangle.min().y, rectangle.max().y);
  let x_coords = uniform_x.sample_iter(rand::thread_rng());
  let y_coords = uniform_y.sample_iter(rand::thread_rng());
  x_coords.zip(y_coords).map(|(x, y)| (x, y).into())
}

pub fn halton_23(polygon: Polygon<f32>, density: f32) -> impl Iterator<Item = Coord<f32>> {
  let bounding_rect = polygon.bounding_rect().unwrap();
  let density = density.clamp(0.0, 1.0);
  let n_points = (bounding_rect.width() * bounding_rect.height() * density) as usize;
  halton_23_rectangle(bounding_rect)
    .filter(move |coord| coord.intersects(&polygon))
    .take(n_points)
}

fn halton_23_rectangle(rectangle: Rect<f32>) -> impl Iterator<Item = Coord<f32>> {
  let center = rectangle.center();
  let width = rectangle.width();
  let height = rectangle.height();
  let halton_x = halton::Sequence::new(2);
  let halton_y = halton::Sequence::new(3);
  halton_x.zip(halton_y).map(move |(x, y)| {
    let x = (x as f32 - 0.5) * width + center.x;
    let y = (y as f32 - 0.5) * height + center.y;
    (x, y).into()
  })
}
