use geo::{LineString, Polygon};
use nannou::{
  prelude::{Hsl, Key, WHITE},
  App,
};
use utils::app::{make_static_artwork, Artwork, ArtworkOptions, BaseModel, StaticArtwork};

fn main() {
  make_static_artwork::<Model>().run();
}

struct Model {
  base_model: BaseModel,
  weight: f32,
  density: f32,
}

impl Artwork for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      weight: 1.0,
      density: 0.1,
    }
  }
  fn get_options() -> ArtworkOptions {
    ArtworkOptions::default()
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
      Key::Up => self.weight += 0.5,
      Key::Down => self.weight -= 0.5,
      Key::Left => self.density -= 0.01,
      Key::Right => self.density += 0.01,
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
    let polygon = Polygon::new(
      LineString::from(vec![
        (-w / 4.0, -h / 4.0),
        (-w / 3.0, h / 3.0),
        (-w / 7.5, 0.0),
        (w / 3.0, h / 3.0),
        (w / 4.0, -h / 4.0),
        (w / 5.0, -h / 3.0),
      ]),
      vec![],
    );

    utils::draw::filling::halton_23(
      polygon,
      draw,
      utils::draw::filling::FillingOptions {
        weight: self.weight,
        density: self.density,
        color: Hsl::new(0.0, 0.0, 0.0),
      },
    );
  }
}
