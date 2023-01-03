use crate::geometry::filling;
use geo::Polygon;
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
  filling::uniform(polygon, options.density).for_each(|coord| {
    draw
      .ellipse()
      .xy(Vec2::from(coord.x_y()))
      .w_h(options.weight, options.weight)
      .color(options.color);
  });
}
