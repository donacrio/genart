use geo::{coord, AffineOps, AffineTransform, BoundingRect, MultiPolygon, Rect, Scale};
use nannou::{prelude::Key, text, App};
use plants::utils::{
  app::{make_static_artwork, Artwork, ArtworkOptions, BaseModel, StaticArtwork},
  draw::BrushDrawer,
  layout::tile,
};
use plants::{
  systems::{
    leaf::{leaf_rule, LEAF_AXIOM},
    LSystem,
  },
  turtle,
};
use rand::{distributions::Standard, prelude::Distribution, rngs::StdRng, SeedableRng};
use std::{f64::consts::FRAC_PI_3, path::Path, usize::MAX};

struct Model {
  base_model: BaseModel,
  steps: usize,
  turtle_params: turtle::polygon::Params,
}

impl Artwork for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      steps: 20,
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
      Key::Minus => self.steps = (self.steps - 1).clamp(0, MAX),
      _ => {}
    }
  }
}

impl StaticArtwork for Model {
  fn draw(&mut self) {
    let mut rng: StdRng = StdRng::seed_from_u64(self.base_model.seed);
    let draw = &self.base_model.draw;
    draw.background().color(nannou::color::WHITE);

    let [w_w, _] = self.base_model.texture.size();
    let size = w_w as f64;

    let rect = Rect::new(
      coord! {x: -(size / 2.0), y: -(size / 2.0) },
      coord! {x: size / 2.0, y: size / 2.0 },
    );

    let multipolygons = tile(rect, 3, 3)
      .iter()
      .map(|rect| rect.scale(0.8))
      .map(|rect| {
        (
          rect,
          grow_l_system(self.steps, &mut rng, &self.turtle_params, &rect),
        )
      })
      .collect::<Vec<_>>();

    let text_layout = text::layout::Builder::default()
      .font(text::font::from_file(Path::new("assets").join("fonts").join("Inkfree.ttf")).unwrap())
      .font_size(50)
      .center_justify()
      .align_middle_y()
      .build();
    multipolygons.iter().for_each(|(rect, _)| {
      let (x_min, y_min) = rect.min().x_y();
      let (x_max, _) = rect.max().x_y();
      draw
        .text(lipsum::lipsum_words_with_rng(&mut rng, 2).as_str())
        .layout(&text_layout)
        .x_y((x_min + x_max) as f32 / 2.0, y_min as f32)
        .w_h(rect.width() as f32, rect.height() as f32)
        .color(nannou::color::BLACK);
    });

    multipolygons.iter().for_each(|(_, multipolygon)| {
      multipolygon.iter().for_each(|polygon| {
        draw
          .polyline()
          .stroke_weight(2.0)
          .brush_from_linestring(polygon.exterior(), 7.0, &mut rng)
          .color(nannou::color::BLACK);
      });
    });
  }
}

fn grow_l_system(
  steps: usize,
  rng: &mut StdRng,
  turtle_params: &turtle::polygon::Params,
  rect: &Rect,
) -> MultiPolygon {
  let mut l_system = LSystem::new(LEAF_AXIOM.to_vec(), leaf_rule, Standard.sample(rng));
  let commands = l_system.nth(steps).unwrap();
  let mut polygons = turtle::polygon::to_geom(commands, turtle_params);

  let bbox = polygons.bounding_rect().unwrap();
  let transform: AffineTransform<_> = AffineTransform::translate(
    rect.center().x - bbox.center().x,
    rect.center().y - bbox.center().y,
  )
  .scaled(
    rect.width() / bbox.width(),
    rect.width() / bbox.width(),
    bbox.center(),
  )
  .rotated(45.0, bbox.center());
  polygons.affine_transform_mut(&transform);

  polygons
}
fn main() {
  make_static_artwork::<Model>().run()
}
