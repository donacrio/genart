use geo::{coord, Rect};
use nannou::prelude::map_range;

#[allow(dead_code)]
pub fn tile(rect: Rect, nx: usize, ny: usize) -> Vec<Rect> {
  let (x_min, y_min) = rect.min().x_y();
  let (x_max, y_max) = rect.max().x_y();
  itertools::iproduct!(0..nx, 0..ny)
    .map(|(i, j)| {
      Rect::new(
        coord! {x:map_range(i, 0, nx, x_min, x_max), y:map_range(j, 0, ny, y_min, y_max) },
        coord! {x:map_range(i+1, 0, nx, x_min, x_max), y:map_range(j+1, 0, ny, y_min, y_max) },
      )
    })
    .collect::<Vec<Rect>>()
}
