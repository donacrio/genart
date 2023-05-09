use geo::Polygon;

use crate::l_system::{Depile, LSystem, Letter, Load, Pile, Save, Turtle, Vertex, A};

pub fn create_l_system() -> LSystem {
  LSystem::new(vec![
    Box::new(Save),
    Box::new(Pile),
    Box::new(A {
      time: 0.0,
      direction: true,
    }),
    Box::new(Vertex),
    Box::new(Depile),
    Box::new(Load),
    Box::new(Save),
    Box::new(Pile),
    Box::new(A {
      time: 0.0,
      direction: false,
    }),
    Box::new(Vertex),
    Box::new(Depile),
    Box::new(Load),
  ])
}

// Turtle definition
pub fn turtle(sentence: &[Box<dyn Letter>]) -> Vec<Polygon> {
  let mut turtle = Turtle::default();
  sentence
    .iter()
    .for_each(|letter| letter.turtle(&mut turtle));
  turtle.polygones
}
