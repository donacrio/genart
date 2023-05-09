use std::fs;

use geo_svg::{Color, ToSvg};

use crate::systems::simple_leafs::{create_l_system, turtle};

mod l_system;
mod systems;

fn main() {
  fs::create_dir_all("out/leafs").unwrap();
  let mut l_system = create_l_system();

  for i in 0..30 {
    l_system.step();
    let polygones = turtle(&l_system.sentence);
    let svg = polygones
      .to_svg()
      .with_stroke_color(Color::Named("BLACK"))
      .with_fill_opacity(0.0);
    fs::write(format!("out/leafs/out_{}.svg", i), svg.to_string()).unwrap();
  }
}
