use geo::{CoordNum, LineString};
use nannou::{
  draw::{
    primitive::{Path, PathStroke},
    Drawing,
  },
  prelude::{real::Real, Point2},
};

pub trait NannouDrawer<'a, T> {
  fn polyline_from_linestring(self, line: &LineString<T>) -> Drawing<'a, Path>
  where
    T: CoordNum + Real;
}

impl<'a, T> NannouDrawer<'a, T> for Drawing<'a, PathStroke> {
  fn polyline_from_linestring(self, line: &LineString<T>) -> Drawing<'a, Path>
  where
    T: CoordNum,
  {
    self.points(
      line
        .coords()
        .map(|p| Point2::new(p.x.to_f32().unwrap(), p.y.to_f32().unwrap()))
        .collect::<Vec<Point2>>(),
    )
  }
}
