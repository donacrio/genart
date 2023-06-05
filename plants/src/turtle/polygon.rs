use std::collections::VecDeque;

use nalgebra::{Point3, Rotation3, Vector3};

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

pub fn to_geom<T: TurtleInterpretation>(
  commands: Vec<T>,
  params: &Params,
) -> Vec<Vec<Point3<f64>>> {
  let mut polygons = vec![];

  let mut position = Vector3::new(0.0, 0.0, 0.0);
  let mut rotation = Rotation3::from_axis_angle(&Vector3::z_axis(), 0.0);
  let mut states = VecDeque::new();
  let mut points = vec![];
  let mut saved_points = VecDeque::new();
  for command in commands.iter() {
    match command.to_turtle() {
      Turtle::Vertex => points.push(position.into()),
      Turtle::Forward(length) => position += rotation * Vector3::identity() * length,
      Turtle::Left => {
        rotation = match rotation.axis() {
          Some(axis) => Rotation3::from_axis_angle(&axis, rotation.angle() + params.angle),
          None => Rotation3::from_axis_angle(&Vector3::z_axis(), params.angle),
        }
      }
      Turtle::Right => {
        rotation = match rotation.axis() {
          Some(axis) => Rotation3::from_axis_angle(&axis, rotation.angle() - params.angle),
          None => Rotation3::from_axis_angle(&Vector3::z_axis(), -params.angle),
        }
      }
      Turtle::Push => states.push_back((position, rotation)),
      Turtle::Pop => (position, rotation) = states.pop_back().unwrap(),
      Turtle::NewPolygon => {
        saved_points.push_back(points);
        points = vec![];
      }
      Turtle::ClosePolygon => {
        polygons.push(points);
        points = saved_points.pop_back().unwrap();
      }
      Turtle::None => {}
    }
  }
  polygons
}
