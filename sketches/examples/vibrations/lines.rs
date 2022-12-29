use geo::{EuclideanLength, LineString};
use nannou::{
  prelude::{Hsl, Key, WHITE},
  App,
};
use std::path::PathBuf;
use utils::{
  app::{
    make_static_artwork, update_static, BaseModel, NannouApp, NannouAppOptions, StaticArtwork,
  },
  draw::line::LineOptions,
};

const N_LINES: usize = 25;

#[deprecated]
fn main() {
  make_static_artwork::<Model>().run();
}

struct Model {
  base_model: BaseModel,
  depth: usize,
}

impl NannouApp for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      depth: 0,
    }
  }
  fn get_options() -> NannouAppOptions {
    NannouAppOptions {
      background_path: Some(PathBuf::from("paper.jpg")),
      ..NannouAppOptions::default()
    }
  }
  fn get_base_model(&self) -> &BaseModel {
    &self.base_model
  }
  fn get_base_model_mut(&mut self) -> &mut BaseModel {
    &mut self.base_model
  }
  fn current_frame_name(&self) -> String {
    format!("frame_{}_{}", self.depth, self.base_model.seed)
  }

  fn key_pressed(&mut self, _app: &App, key: Key) {
    match key {
      Key::Up => self.depth += 1,
      Key::Down => {
        if self.depth > 0 {
          self.depth -= 1
        }
      }
      _ => {}
    }
  }
  fn update(&mut self, _app: &App) {
    update_static(self)
  }
}

impl StaticArtwork for Model {
  fn draw(&self) {
    let draw = &self.base_model.draw;

    draw.background().color(WHITE);

    let [w_w, w_h] = self.base_model.texture.size();

    (0..N_LINES)
      .map(|i| {
        let h = (i as f32 / (N_LINES - 1) as f32 - 0.5) * w_h as f32 * 0.8;
        let start = (-(w_w as f32) * 0.9 / 2., h).into();
        let end = (w_w as f32 * 0.9 / 2., h).into();
        (start, end)
      })
      .map(|(start, end)| {
        utils::geometry::line::sample_straight(start, end, 50).collect::<LineString<f32>>()
      })
      .map(|line_string| {
        line_string
          .lines()
          .flat_map(|line| {
            let factor =
              2.0 * ((line.start.x / w_w as f32) + 0.5) * ((line.start.y / w_h as f32) - 0.5);
            let std_dev = line.euclidean_length() * factor;
            utils::geometry::line::sample_wooble(line.start, line.end, 1, std_dev)
          })
          .collect::<LineString<f32>>()
      })
      .for_each(|line_string| {
        line_string.lines().for_each(|line| {
          let factor =
            200.0 * ((line.start.x / w_w as f32) + 0.5) * ((line.start.y / w_h as f32) - 0.5);
          let weight = 0.0001 * line_string.euclidean_length() * factor;
          let density = 0.75;
          utils::draw::line::pencil(
            line.start,
            line.end,
            draw,
            LineOptions {
              weight,
              density,
              color: Hsl::new(0.0, 0.0, 0.0),
            },
          );
        });
      });
  }
}
