use crate::{
  draw,
  geometry::{self, hatch::hatch},
};
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

pub fn stroke(polygon: Polygon<f32>, draw: &Draw, degrees: f32, options: FillingOptions) {
  hatch(polygon, options.density, degrees).for_each(|(start, end)| {
    draw::line::stroke(
      start,
      end,
      draw,
      draw::line::LineOptions {
        weight: options.weight,
        density: options.density,
        color: options.color,
      },
    )
  })
}

pub fn brush(polygon: Polygon<f32>, draw: &Draw, degrees: f32, options: FillingOptions) {
  hatch(polygon, options.density, degrees).for_each(|(start, end)| {
    draw::line::brush(
      start,
      end,
      draw,
      draw::line::LineOptions {
        weight: options.weight,
        density: options.density,
        color: options.color,
      },
    )
  })
}

pub fn pencil(polygon: Polygon<f32>, draw: &Draw, degrees: f32, options: FillingOptions) {
  hatch(polygon, options.density, degrees).for_each(|(start, end)| {
    draw::line::pencil(
      start,
      end,
      draw,
      draw::line::LineOptions {
        weight: options.weight,
        density: options.density,
        color: options.color,
      },
    )
  })
}

pub fn marker(polygon: Polygon<f32>, draw: &Draw, degrees: f32, options: FillingOptions) {
  hatch(polygon, options.density, degrees).for_each(|(start, end)| {
    draw::line::marker(
      start,
      end,
      draw,
      draw::line::LineOptions {
        weight: options.weight,
        density: options.density,
        color: options.color,
      },
    )
  })
}
