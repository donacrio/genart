//! A simple as possible example demonstrating how to use the `draw` API to display a texture.

use nannou::prelude::*;

fn main() {
  nannou::app(model).run();
}

struct Model {
  _window_id: WindowId,
  background_sampler: utils::texture::ImageSampler,
  test_sampler: utils::texture::ImageSampler,
}

fn model(app: &App) -> Model {
  let _window_id = app.new_window().size(512, 512).view(view).build().unwrap();

  // Load the image from disk and upload it to a GPU texture.
  let images = app.project_path().unwrap().join("hantai").join("images");
  let background_path = images.join("paper-texture.jpg");
  let background_sampler = utils::texture::ImageSampler::new(app, background_path);
  let logo_path = images.join("twitter-logo.png");
  let test_sampler = utils::texture::ImageSampler::new(app, logo_path);

  Model {
    _window_id,
    background_sampler,
    test_sampler,
  }
}

fn view(app: &App, model: &Model, frame: Frame) {
  frame.clear(BLACK);

  let draw = app.draw();
  let draw = model.background_sampler.draw(draw, None, None);

  draw.to_frame(app, &frame).unwrap();

  let draw = app.draw();
  let area = geom::Rect::from_x_y_w_h(0.5, 0.5, 10.0, 10.0);
  println!("{:?}", area);
  let draw = model.test_sampler.draw(draw, Some(area), None);
  draw.to_frame(app, &frame).unwrap();
}
