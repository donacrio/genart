use crate::utils::geometry::{WorldPoint, WorldRotation, WorldVector};
use euclid::Angle;
use std::{collections::VecDeque, f64::consts::FRAC_PI_4, fmt::Debug};

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

pub fn to_geom<T: TurtleInterpretation + Debug>(
  commands: Vec<T>,
  params: &Params,
) -> Vec<Vec<WorldPoint>> {
  let mut polygons = vec![];

  let mut position = WorldVector::zero();
  let mut rotation = WorldRotation::around_z(Angle::radians(FRAC_PI_4));
  let mut states = VecDeque::new();
  let mut points = vec![];
  let mut saved_points = VecDeque::new();
  for command in commands.iter() {
    match command.to_turtle() {
      Turtle::Vertex => points.push(position.to_point()),
      Turtle::Forward(length) => {
        position += rotation.transform_vector3d(WorldVector::one()) * length
      }
      Turtle::Left => {
        rotation = rotation.then(&WorldRotation::around_z(Angle::radians(params.angle)));
      }
      Turtle::Right => {
        rotation = rotation.then(&WorldRotation::around_z(Angle::radians(-params.angle)));
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
