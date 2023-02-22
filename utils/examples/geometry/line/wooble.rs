use geo::Coord;
use nannou::{
  prelude::{Key, Vec2, BLACK, WHITE},
  App,
};
use utils::app::{make_static_artwork, Artwork, ArtworkOptions, BaseModel, StaticArtwork};

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
  fn key_pressed(&mut self, _app: &App, _key: Key) {}
}

impl StaticArtwork for Model {
  fn draw(&mut self) {
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
        utils::geometry::line::sample_wooble(start, end, 20, 10.0)
          .map(|coord| Vec2::new(coord.x, coord.y))
          .for_each(|point| {
            draw.ellipse().xy(point).w_h(10.0, 10.0).color(BLACK);
          });
      });
  }
}
