use geo::Coord;
use nannou::{
  prelude::{Hsl, Key, WHITE},
  App,
};
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
    NannouAppOptions::default()
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

    (0..5)
      .map(|i| {
        let h = (i as f64 / 5f64 - 0.5f64) * w_h as f64 / 2f64;
        let start: Coord = (-(w_w as f64) * 0.90f64 / 2f64, h).into();
        let end: Coord = (w_w as f64 * 0.90f64 / 2f64, h).into();
        (start, end)
      })
      .for_each(|(start, end)| {
        utils::draw::line::stroke(
          start,
          end,
          draw,
          LineOptions {
            weight: 10.0,
            density: 1.0,
            color: Hsl::new(0.0, 0.0, 0.0),
          },
        )
      });
  }
}
