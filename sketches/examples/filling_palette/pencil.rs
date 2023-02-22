use geo::{LineString, Polygon, Translate};
use nannou::{
  prelude::{Hsl, Key, WHITE},
  App,
};
use std::path::PathBuf;
use utils::app::{make_static_artwork, Artwork, ArtworkOptions, BaseModel, StaticArtwork};

const DEGREES: f32 = 60.0;

fn main() {
  make_static_artwork::<Model>().run();
}

struct Model {
  base_model: BaseModel,
}

impl Artwork for Model {
  fn new(base_model: BaseModel) -> Self {
    Self { base_model }
  }
  fn get_options() -> ArtworkOptions {
    ArtworkOptions {
      background_path: Some(PathBuf::from("paper.jpg")),
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
    String::from("frame")
  }
  fn key_pressed(&mut self, _app: &App, _key: Key) {}
}

impl StaticArtwork for Model {
  fn draw(&mut self) {
    let draw = &self.base_model.draw;

    draw.background().color(WHITE);
    let [w_w, w_h] = self.base_model.texture.size();
    let w = w_w as f32 * 0.9;
    let h = w_h as f32 * 0.9;
    const N_WEIGHT: usize = 4;
    const N_DENSITY: usize = 4;
    (0..N_WEIGHT)
      .flat_map(|i| {
        (0..N_DENSITY).map(move |j| {
          let x = (((i as f32 + 0.5) / N_WEIGHT as f32) - 0.5) * w;
          let y = (((j as f32 + 0.5) / N_DENSITY as f32) - 0.5) * h;
          let polygon = Polygon::new(
            LineString::from(vec![
              (-w / 16.0, -h / 10.0),
              (-w / 10.0, h / 12.0),
              (-w / 30.0, 0.0),
              (w / 12.0, h / 12.0),
              (w / 16.0, -h / 16.0),
              (w / 20.0, -h / 12.0),
            ]),
            vec![],
          );
          let polygon = polygon.translate(x, y);
          let weight = 1.0 * (i + 1) as f32;
          let density = 0.5 * (j + 2) as f32 / N_DENSITY as f32;
          (polygon, weight, density)
        })
      })
      .for_each(|(polygon, weight, density)| {
        utils::draw::filling::pencil(
          polygon,
          draw,
          DEGREES,
          utils::draw::filling::FillingOptions {
            weight,
            density,
            color: Hsl::new(0.0, 0.0, 0.0),
          },
        )
      });
  }
}
