use std::collections::VecDeque;

use geo::{coord, LineString, MultiPolygon, Polygon};

pub trait TurtleInterpretation {
  fn to_turtle(&self) -> Turtle;
}
pub enum Turtle {
  Vertex,
  Forward(f64),
  Left,
  Right,
  Push,
  Pop,
  NewPolygon,
  ClosePolygon,
  None,
}

pub struct Params {
  pub angle: f64,
}

impl Params {
  pub fn new(angle: f64) -> Self {
    Self { angle }
  }
}

pub fn to_geom<T: TurtleInterpretation>(commands: Vec<T>, params: Params) -> MultiPolygon {
  let mut polygons = vec![];

  let mut position = coord! {x:0.0, y:0.0};
  let mut angle: f64 = 0.0;
  let mut states = VecDeque::new();
  let mut points = vec![];
  let mut saved_points = VecDeque::new();
  for command in commands.iter() {
    match command.to_turtle() {
      Turtle::Vertex => points.push(position),
      Turtle::Forward(length) => {
        position = position + coord! {x:angle.cos(), y: angle.sin()} * length
      }
      Turtle::Left => angle += params.angle,
      Turtle::Right => angle -= params.angle,
      Turtle::Push => states.push_back((position, angle)),
      Turtle::Pop => (position, angle) = states.pop_back().unwrap(),
      Turtle::NewPolygon => {
        saved_points.push_back(points);
        points = vec![];
      }
      Turtle::ClosePolygon => {
        polygons.push(Polygon::new(LineString::new(points), vec![]));
        points = saved_points.pop_back().unwrap();
      }
      Turtle::None => {}
    }
  }
  MultiPolygon::from(polygons)
}
