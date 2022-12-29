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
  weight: f32,
  density: f32,
}

impl NannouApp for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      weight: 10.0,
      density: 0.75,
    }
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
  fn key_pressed(&mut self, _app: &App, key: Key) {
    match key {
      Key::Up => self.weight += 5.0,
      Key::Down => self.weight -= 5.0,
      Key::Left => self.density -= 0.05,
      Key::Right => self.density += 0.05,
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

    (0..5)
      .map(|i| {
        let h = (i as f32 / 5. - 0.5) * w_h as f32 / 2.;
        let start: Coord<f32> = (-(w_w as f32) * 0.90 / 2., h).into();
        let end: Coord<f32> = (w_w as f32 * 0.90 / 2., h).into();
        (start, end)
      })
      .for_each(|(start, end)| {
        utils::draw::line::japanese_brush(
          start,
          end,
          draw,
          LineOptions {
            weight: self.weight,
            density: self.density,
            color: Hsl::new(0.0, 0.0, 0.0),
          },
        )
      });
  }
}
