mod app;
mod draw;
mod systems;
mod turtle;

use app::{make_static_artwork, Artwork, ArtworkOptions, BaseModel, StaticArtwork};
use draw::BrushDrawer;
use geo::{AffineOps, AffineTransform, BoundingRect};
use nannou::{prelude::Key, App};
use rand::{rngs::StdRng, SeedableRng};
use std::f64::consts::FRAC_PI_3;
use systems::{
  leaf::{leaf_rule, LeafParameters, LEAF_AXIOM},
  LSystem,
};

struct Model {
  base_model: BaseModel,
  pub params: LeafParameters,
  steps: usize,
  turtle_params: turtle::polygon::Params,
}

impl Artwork for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      params: LeafParameters::new(5.0, 1.0, 0.6, 1.06, 0.0, 1.0, 0.25),
      steps: 10,
      turtle_params: turtle::polygon::Params::new(FRAC_PI_3),
    }
  }
  fn get_options() -> ArtworkOptions {
    ArtworkOptions {
      ..ArtworkOptions::default()
    }
  }
  fn get_base_model(&self) -> &BaseModel {
    &self.base_model
  }
  fn get_base_model_mut(&mut self) -> &mut BaseModel {
    &mut self.base_model
  }
  fn current_frame_name(&self) -> String {
    format!("frame_{}", self.base_model.seed)
  }
  fn key_pressed(&mut self, _app: &App, key: Key) {
    match key {
      Key::Equals => self.steps += 1,
      Key::Minus => self.steps -= 1,
      _ => {}
    }
  }
}

impl StaticArtwork for Model {
  fn draw(&mut self) {
    let mut l_system = LSystem::new(LEAF_AXIOM.to_vec(), leaf_rule, self.params.clone());
    let commands = l_system.nth(self.steps).unwrap();
    let mut polygons = turtle::polygon::to_geom(commands, &self.turtle_params);

    let draw = &self.base_model.draw;
    draw.background().color(nannou::color::WHITE);
    let [w_w, _] = self.base_model.texture.size();
    let w = w_w as f64 * 0.9;

    let bbox = polygons.bounding_rect().unwrap();
    let transform: AffineTransform<_> =
      AffineTransform::translate(-bbox.center().x, -bbox.center().y)
        .scaled(w / bbox.width(), w / bbox.width(), bbox.center())
        .rotated(90.0, bbox.center());
    polygons.affine_transform_mut(&transform);

    let mut rng = StdRng::seed_from_u64(self.base_model.seed);
    for polygon in polygons {
      draw
        .polyline()
        .stroke_weight(3.0)
        .brush_from_linestring(polygon.exterior(), 10.0, &mut rng)
        .color(nannou::color::BLACK);
    }
  }
}

fn main() {
  make_static_artwork::<Model>().run()
}
