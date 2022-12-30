use geo::{Coord, Line, Rotate};
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

fn main() {
  make_static_artwork::<Model>().run();
}

struct Model {
  base_model: BaseModel,
}

impl NannouApp for Model {
  fn new(base_model: BaseModel) -> Self {
    Self { base_model }
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
    String::from("frame")
  }
  fn key_pressed(&mut self, _app: &App, _key: Key) {}
  fn update(&mut self, _app: &App) {
    update_static(self)
  }
}

impl StaticArtwork for Model {
  fn draw(&self) {
    let draw = &self.base_model.draw;

    draw.background().color(WHITE);
    let [w_w, w_h] = self.base_model.texture.size();

    const N_WEIGHT: usize = 10;
    const N_DENSITY: usize = 5;
    const LINE_LENGTH: f32 = 400.0;
    (2..N_WEIGHT)
      .flat_map(|i| {
        (1..=N_DENSITY).map(move |j| {
          let start: Coord<f32> = (
            w_w as f32 * (i as f32 / (N_WEIGHT + 1) as f32 - 0.5) - LINE_LENGTH / 2.0,
            w_h as f32 * (j as f32 / (N_DENSITY + 1) as f32 - 0.5),
          )
            .into();
          let end = start + (LINE_LENGTH, 0.0).into();
          let line = Line::new(start, end);
          let line = line.rotate_around_centroid(45.0);
          let weight = (i * 5) as f32;
          let density = j as f32 / N_DENSITY as f32;
          println!("{density}");
          (line.start, line.end, weight, density)
        })
      })
      .for_each(|(start, end, weight, density)| {
        utils::draw::line::brush(
          start,
          end,
          draw,
          LineOptions {
            weight,
            density,
            color: Hsl::new(0.0, 0.0, 0.0),
          },
        )
      });
  }
}
