mod l_system;
mod systems;
mod turtle;

use geo_svg::{Color, ToSvg};
use l_system::LSystem;
use std::{f64::consts::FRAC_PI_3, fs};
use systems::leaf::{leaf_rule, LeafParameters, LEAF_AXIOM};
use turtle::polygon::to_geom;

fn main() {
  fs::create_dir_all("out/leafs").unwrap();
  let l_system_params = LeafParameters::new(4.0, 1.1, 1.0, 1.2, 1.0, 1.0, 1.0);
  let mut l_system = LSystem::new(LEAF_AXIOM.to_vec(), leaf_rule, l_system_params);

  let commands = l_system.nth(50).unwrap();

  let draw_params = turtle::polygon::Params::new(FRAC_PI_3);
  let polygons = to_geom(commands, draw_params);

  let svg = polygons
    .0
    .to_svg()
    .with_stroke_color(Color::Named("BLACK"))
    .with_fill_opacity(0.0);
  fs::write("out/leafs/out.svg", svg.to_string()).unwrap();
}
