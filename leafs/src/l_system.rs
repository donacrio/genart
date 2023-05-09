use std::{collections::VecDeque, f64::consts::FRAC_PI_3};

use geo::{coord, Coord, LineString, Polygon};

pub trait Letter {
  fn update(&self) -> Vec<Box<dyn Letter>>;
  fn to_string(&self) -> String;
  fn turtle(&self, turtle: &mut Turtle);
}

pub struct LSystem {
  pub sentence: Vec<Box<dyn Letter>>,
}

impl LSystem {
  pub fn new(sentence: Vec<Box<dyn Letter>>) -> Self {
    Self { sentence }
  }

  pub fn step(&mut self) {
    self.sentence = self
      .sentence
      .iter()
      .flat_map(|letter| letter.update())
      .collect();
  }

  #[allow(dead_code)]
  pub fn to_string(&self) -> String {
    self
      .sentence
      .iter()
      .map(|letter| letter.to_string())
      .collect()
  }
}

// Turtle
pub struct Turtle {
  pub state: TurtleState,
  pub previous_states: VecDeque<TurtleState>,
  pub line: Vec<Coord>,
  pub previous_lines: VecDeque<Vec<Coord>>,
  pub polygones: Vec<Polygon>,
}

impl Default for Turtle {
  fn default() -> Self {
    Self {
      state: TurtleState::new((0.0, 0.0).into(), 0.0),
      previous_states: VecDeque::new(),
      line: vec![],
      previous_lines: VecDeque::new(),
      polygones: vec![],
    }
  }
}

#[derive(Clone)]
pub struct TurtleState {
  pub coord: Coord,
  pub angle: f64,
}

impl TurtleState {
  fn new(coord: Coord, angle: f64) -> Self {
    Self { coord, angle }
  }
}

// Constant definition
const ROTATION_ANGLE: f64 = FRAC_PI_3;
const A_LENGTH: f64 = 4.0;
const A_GROWTH_FACTOR: f64 = 1.1;
const B_LENGTH: f64 = 1.0;
const B_GROWTH_FACTOR: f64 = 1.2;
const C_LENGTH: f64 = 1.0;
const C_GROWTH_FACTOR: f64 = 1.0;
const POTENTIAL_DECREMENT: f64 = 1.0;

// Common letters
pub struct Vertex;
impl Letter for Vertex {
  fn update(&self) -> Vec<Box<dyn Letter>> {
    vec![Box::new(Self)]
  }
  fn to_string(&self) -> String {
    ".".to_string()
  }
  fn turtle(&self, turtle: &mut Turtle) {
    turtle.line.push(turtle.state.coord);
  }
}

pub struct G {
  pub length: f64,
  pub growth_rate: f64,
  pub time: Option<f64>,
}

impl Letter for G {
  fn update(&self) -> Vec<Box<dyn Letter>> {
    match self.time {
      None => vec![Box::new(Self {
        length: self.length * self.growth_rate,
        growth_rate: self.growth_rate,
        time: None,
      })],
      Some(time) if time > 1.0 => vec![Box::new(Self {
        length: self.length * self.growth_rate,
        growth_rate: self.growth_rate,
        time: Some(time - POTENTIAL_DECREMENT),
      })],
      _ => vec![],
    }
  }
  fn to_string(&self) -> String {
    match self.time {
      Some(time) => format!("G({}, {}, {})", self.length, self.growth_rate, time),
      None => format!("G({}, {})", self.length, self.growth_rate),
    }
  }
  fn turtle(&self, turtle: &mut Turtle) {
    turtle.state.coord = turtle.state.coord
      + coord! { x:turtle.state.angle.cos(), y:turtle.state.angle.sin()} * self.length;
  }
}

pub struct A {
  pub time: f64,
  pub direction: bool,
}
impl Letter for A {
  fn update(&self) -> Vec<Box<dyn Letter>> {
    match self.direction {
      true => vec![
        Box::new(Vertex),
        Box::new(G {
          length: A_LENGTH,
          growth_rate: A_GROWTH_FACTOR,
          time: None,
        }),
        Box::new(Vertex),
        Box::new(Save),
        Box::new(Plus {
          angle: ROTATION_ANGLE,
        }),
        Box::new(B { time: self.time }),
        Box::new(G {
          length: C_LENGTH,
          growth_rate: C_GROWTH_FACTOR,
          time: Some(self.time),
        }),
        Box::new(Vertex),
        Box::new(Depile),
        Box::new(Load),
        Box::new(Save),
        Box::new(Plus {
          angle: ROTATION_ANGLE,
        }),
        Box::new(B { time: self.time }),
        Box::new(Pile),
        Box::new(Vertex),
        Box::new(Load),
        Box::new(Self {
          time: self.time + POTENTIAL_DECREMENT,
          direction: self.direction,
        }),
      ],
      false => vec![
        Box::new(Vertex),
        Box::new(G {
          length: A_LENGTH,
          growth_rate: A_GROWTH_FACTOR,
          time: None,
        }),
        Box::new(Vertex),
        Box::new(Save),
        Box::new(Minus {
          angle: ROTATION_ANGLE,
        }),
        Box::new(B { time: self.time }),
        Box::new(G {
          length: C_LENGTH,
          growth_rate: C_GROWTH_FACTOR,
          time: Some(self.time),
        }),
        Box::new(Vertex),
        Box::new(Depile),
        Box::new(Load),
        Box::new(Save),
        Box::new(Minus {
          angle: ROTATION_ANGLE,
        }),
        Box::new(B { time: self.time }),
        Box::new(Pile),
        Box::new(Vertex),
        Box::new(Load),
        Box::new(Self {
          time: self.time + POTENTIAL_DECREMENT,
          direction: self.direction,
        }),
      ],
    }
  }
  fn to_string(&self) -> String {
    format!("A({}, {})", self.time, self.direction)
  }
  fn turtle(&self, _: &mut Turtle) {}
}

#[derive(Clone)]
pub struct B {
  pub time: f64,
}
impl Letter for B {
  fn update(&self) -> Vec<Box<dyn Letter>> {
    if self.time > 0.0 {
      return vec![
        Box::new(G {
          length: B_LENGTH,
          growth_rate: B_GROWTH_FACTOR,
          time: None,
        }),
        Box::new(Self {
          time: self.time - POTENTIAL_DECREMENT,
        }),
      ];
    }
    vec![Box::new(self.clone())]
  }
  fn to_string(&self) -> String {
    format!("B({})", self.time)
  }
  fn turtle(&self, _: &mut Turtle) {}
}

pub struct Plus {
  pub angle: f64,
}
impl Letter for Plus {
  fn update(&self) -> Vec<Box<dyn Letter>> {
    vec![Box::new(Self { angle: self.angle })]
  }
  fn to_string(&self) -> String {
    "+".to_string()
  }
  fn turtle(&self, turtle: &mut Turtle) {
    turtle.state.angle += self.angle
  }
}

pub struct Minus {
  pub angle: f64,
}
impl Letter for Minus {
  fn update(&self) -> Vec<Box<dyn Letter>> {
    vec![Box::new(Self { angle: self.angle })]
  }
  fn to_string(&self) -> String {
    "-".to_string()
  }
  fn turtle(&self, turtle: &mut Turtle) {
    turtle.state.angle -= self.angle
  }
}

pub struct Pile;
impl Letter for Pile {
  fn update(&self) -> Vec<Box<dyn Letter>> {
    vec![Box::new(Self)]
  }
  fn to_string(&self) -> String {
    "{".to_string()
  }
  fn turtle(&self, turtle: &mut Turtle) {
    turtle.previous_lines.push_back(turtle.line.clone());
    turtle.line = vec![];
  }
}

pub struct Depile;
impl Letter for Depile {
  fn update(&self) -> Vec<Box<dyn Letter>> {
    vec![Box::new(Self)]
  }
  fn to_string(&self) -> String {
    "}".to_string()
  }
  fn turtle(&self, turtle: &mut Turtle) {
    let polygon = Polygon::new(LineString::new(turtle.line.clone()), vec![]);
    turtle.polygones.push(polygon);
    turtle.line = turtle.previous_lines.pop_back().unwrap();
  }
}

pub struct Save;
impl Letter for Save {
  fn update(&self) -> Vec<Box<dyn Letter>> {
    vec![Box::new(Self)]
  }
  fn to_string(&self) -> String {
    "[".to_string()
  }
  fn turtle(&self, turtle: &mut Turtle) {
    turtle.previous_states.push_back(turtle.state.clone());
  }
}

pub struct Load;
impl Letter for Load {
  fn update(&self) -> Vec<Box<dyn Letter>> {
    vec![Box::new(Self)]
  }
  fn to_string(&self) -> String {
    "]".to_string()
  }
  fn turtle(&self, turtle: &mut Turtle) {
    turtle.state = turtle.previous_states.pop_back().unwrap();
  }
}
