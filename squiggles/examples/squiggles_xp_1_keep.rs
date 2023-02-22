/// This rendering is buggy but the result looks cool, I'll keep it here for now
use geo::{Coord, Rect};
use nannou::{
  prelude::{Hsl, Key, WHITE},
  App,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rand_distr::{Distribution, Normal};
use utils::{
  algorithm::space::{Space, SpaceTile},
  app::{make_static_artwork, Artwork, ArtworkOptions, BaseModel, StaticArtwork},
  draw::line::LineOptions,
};

const MIN_SIZE: f32 = 50.0;
const COLOR_PALETTE: [[f32; 3]; 5] = [
  [201.0, 1.0, 0.14],
  [0.0, 0.69, 0.5],
  [31.0, 1.0, 0.48],
  [40.0, 0.97, 0.64],
  [51.0, 0.55, 0.82],
];

fn hsl_from_palette(color: [f32; 3]) -> Hsl {
  Hsl::new(color[0], color[1], color[2])
}

fn main() {
  make_static_artwork::<Model>().run();
}

struct Model {
  base_model: BaseModel,
  depth: u32,
}

impl Artwork for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      depth: 0,
    }
  }
  fn get_options() -> ArtworkOptions {
    ArtworkOptions {
      render_size: [1080, 1080],
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
    format!("frame_{}_{}", self.base_model.seed, self.depth)
  }
  fn key_pressed(&mut self, _app: &App, key: Key) {
    match key {
      Key::Equals => self.depth += 1,
      Key::Minus => self.depth -= 1,
      // Key::Up => self.filling_weight += 1.0,
      // Key::Down => self.filling_weight -= 1.0,
      // Key::Left => self.filling_density -= 0.01,
      // Key::Right => self.filling_density += 0.01,
      // Key::W => self.line_weight += 1.0,
      // Key::S => self.line_weight -= 1.0,
      // Key::A => self.line_density -= 0.05,
      // Key::D => self.line_density += 0.05,
      // Key::I => self.hatches_density += 0.05,
      // Key::K => self.hatches_density -= 0.05,
      // Key::J => self.hatches_degrees -= 5.0,
      // Key::L => self.hatches_degrees += 5.0,
      _ => {}
    }
  }
}

impl StaticArtwork for Model {
  fn draw(&mut self) {
    let draw = &self.base_model.draw;

    draw.background().color(WHITE);
    let [w_w, w_h] = self.base_model.texture.size();

    let w = w_w as f32 * 0.9;
    let h = w_h as f32 * 0.9;
    let min = (-w / 2.0, -h / 2.0).into();
    let max = (w / 2.0, h / 2.0).into();
    let root = Tile::new(min, max);

    let max_children = 2u32.pow(self.depth);
    let mut rng = StdRng::seed_from_u64(self.base_model.seed);
    let mut space = utils::algorithm::space::compute_space(root, max_children, MIN_SIZE, &mut rng);
    let leafs = space.leafs();
    leafs.iter().for_each(|_| {
      let index = rng.gen_range(0..COLOR_PALETTE.len());
      let color = hsl_from_palette(COLOR_PALETTE[index]);

      create_hatches(&mut space, &mut rng)
        .iter()
        .for_each(|(start, end)| {
          utils::draw::line::stroke(
            *start,
            *end,
            draw,
            LineOptions {
              weight: 5.0,
              density: 1.0,
              color,
            },
          )
        });
    });
  }
}

const HATCH_WEIGHT_MEAN: f32 = 0.1;
const HATCH_WEIGHT_STD: f32 = 0.01;

fn create_hatches(space: &mut Space<Tile>, rng: &mut StdRng) -> Vec<(Coord<f32>, Coord<f32>)> {
  space
    .leafs()
    .iter()
    .map(|index| space.get_node(*index).unwrap().content())
    .flat_map(|tile| {
      // IDEA: increment & rotation depending on rectangle size
      let hatch_density = Normal::new(HATCH_WEIGHT_MEAN, HATCH_WEIGHT_STD)
        .unwrap()
        .sample(rng);
      // TODO: use custom rng
      let hatch_degrees = 60.0;
      utils::geometry::hatch::hatch(tile.rect.to_polygon(), hatch_density, hatch_degrees)
    })
    .collect()
}

pub struct Tile {
  pub rect: Rect<f32>,
}

impl SpaceTile for Tile {
  fn new(min: Coord<f32>, max: Coord<f32>) -> Self {
    Tile {
      rect: Rect::new(min, max),
    }
  }

  fn width(&self) -> f32 {
    self.rect.width()
  }

  fn height(&self) -> f32 {
    self.rect.height()
  }

  fn min(&self) -> Coord<f32> {
    self.rect.min()
  }

  fn max(&self) -> Coord<f32> {
    self.rect.max()
  }
}
