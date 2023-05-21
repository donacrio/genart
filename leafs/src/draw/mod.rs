use geo::{coord, LineString};
use nannou::{
  draw::{
    primitive::{Path, PathStroke},
    Drawing,
  },
  prelude::Point2,
};
use rand::{rngs::StdRng, Rng};
use std::f64::consts::PI;

pub trait StrokeDrawer<'a> {
  fn stroke_from_linestring(self, line: &LineString) -> Drawing<'a, Path>;
}

impl<'a> StrokeDrawer<'a> for Drawing<'a, PathStroke> {
  fn stroke_from_linestring(self, line: &LineString) -> Drawing<'a, Path> {
    self.points(
      line
        .coords()
        .map(|p| Point2::new(p.x as f32, p.y as f32))
        .collect::<Vec<Point2>>(),
    )
  }
}

pub trait BrushDrawer<'a> {
  fn brush_from_linestring(
    self,
    line: &LineString,
    radius: f64,
    rng: &mut StdRng,
  ) -> Drawing<'a, Path>;
}

impl<'a> BrushDrawer<'a> for Drawing<'a, PathStroke> {
  fn brush_from_linestring(
    self,
    line: &LineString,
    radius: f64,
    rng: &mut StdRng,
  ) -> Drawing<'a, Path> {
    self.points((0..radius as usize).flat_map(|_| {
      line
        .coords()
        .map(|p| {
          let r = radius * rng.gen::<f64>().sqrt();
          let theta = 2.0 * PI * rng.gen::<f64>().sqrt();
          *p + coord! {x:theta.cos(), y: theta.sin()} * r
        })
        .map(|p| Point2::new(p.x as f32, p.y as f32))
        .collect::<Vec<Point2>>()
    }))
  }
}
