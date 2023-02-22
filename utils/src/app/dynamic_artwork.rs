use super::artwork::{captured_frame_path, make_base_nannou_app, Artwork};
use nannou::{
  prelude::{map_range, Update},
  wgpu, App,
};

pub trait DynamicArtwork: Artwork {
  fn draw_at_time(&mut self, t: f64); // TODO: make immutable ref and create update function with mutable ref
  fn fps(&self) -> u32;
  fn n_sec(&self) -> u32;
  fn current_frame(&mut self) -> &mut u32;
}

pub fn make_dynamic_artwork<T: 'static + DynamicArtwork>() -> nannou::app::Builder<T> {
  make_base_nannou_app().update(update)
}

// TODO: implement slider to see frame per frame
fn update<T: DynamicArtwork>(app: &App, model: &mut T, _update: Update) {
  let n_frames = model.fps() * model.n_sec();
  let elapsed_frames = *model.current_frame();
  let t = map_range(elapsed_frames as f64, 0., n_frames as f64, 0., 1.);

  println!("Computing artwork...");
  model.draw_at_time(t);

  *model.current_frame() = if elapsed_frames == n_frames {
    0
  } else {
    *model.current_frame() + 1
  };

  println!("\nUsing seed {}", model.get_base_model().seed);
  if let Some(background_texture) = &model.get_base_model().background_texture {
    // Rendering texture as background
    let sampler = wgpu::SamplerBuilder::new()
      .address_mode(wgpu::AddressMode::ClampToBorder)
      .into_descriptor();
    let draw = &model.get_base_model().draw;
    draw.sampler(sampler);
    draw.texture(&background_texture);
  }

  println!("Drawing to texture...");
  let window = app.window(model.get_base_model().window_id).unwrap();
  let device = window.device();
  let base_model = model.get_base_model_mut();

  // Render to texture
  let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    label: Some("Texture Renderer"),
  });
  base_model.renderer.render_to_texture(
    device,
    &mut encoder,
    &base_model.draw,
    &base_model.texture,
  );
  let snapshot = model.get_base_model().texture_capturer.capture(
    device,
    &mut encoder,
    &model.get_base_model().texture,
  );
  window.queue().submit(Some(encoder.finish()));

  if model.get_base_model().recording {
    let path = captured_frame_path(app, model.current_frame_name().as_str());
    println!("Saving texture {} ...", path.to_str().unwrap());
    let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: Some("Save texture Renderer"),
    });
    snapshot
      .read(move |result| {
        let image = result.expect("Failed to map texture memory").to_owned();
        image
          .save(&path)
          .expect("Failed to save texture to png image");
      })
      .unwrap();
    window.queue().submit(Some(encoder.finish()));
  }
}
