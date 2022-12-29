use geo::{Coord, Rect};
use rand_distr::{Distribution, Uniform};

pub fn fill_rectangle(rectangle: Rect<f32>, density: f32) -> impl Iterator<Item = Coord<f32>> {
  let uniform_x = Uniform::new(rectangle.min().x, rectangle.max().x);
  let uniform_y = Uniform::new(rectangle.min().y, rectangle.max().y);

  let density = density.clamp(0.0, 1.0);
  let n_points = (rectangle.width() * rectangle.height() * density) as usize;

  let x_coords = uniform_x.sample_iter(rand::thread_rng()).take(n_points);
  let y_coords = uniform_y.sample_iter(rand::thread_rng()).take(n_points);
  x_coords.zip(y_coords).map(|(x, y)| (x, y).into())
}
