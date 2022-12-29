use std::path::PathBuf;

use super::artwork::{DynamicArtwork, DynamicArtworkOptions};
use nannou::{
  prelude::{map_range, Key, Update},
  wgpu, window, App, Draw, Frame,
};
use rand::random;

pub struct DynamicBaseModel {
  window_id: window::Id,
  pub draw: Draw,
  pub texture: wgpu::Texture,
  renderer: nannou::draw::Renderer,
  texture_capturer: wgpu::TextureCapturer,
  texture_reshaper: wgpu::TextureReshaper,
  background_texture: Option<wgpu::Texture>,
  // Recording utility fields
  current_frame: u32,
  recording: bool,
  pub seed: i32,
  fps: u32,
  n_sec: u32,
}

fn make_base_model<T: 'static + DynamicArtwork>(
  app: &App,
  options: DynamicArtworkOptions,
) -> DynamicBaseModel {
  let [win_w, win_h] = options.render_size;
  let window_id = app
    .new_window()
    .size(win_w, win_h)
    .view::<T>(view)
    .key_pressed::<T>(key_pressed)
    .build()
    .unwrap();
  let window = app.window(window_id).unwrap();
  let draw = Draw::new();

  // Retrieve the wgpu device.
  let device = window.device();
  // Create our custom texture.
  let sample_count = window.msaa_samples();

  let texture = wgpu::TextureBuilder::new()
    .size(options.texture_size)
    // Our texture will be used as the RENDER_ATTACHMENT for our `Draw` render pass.
    // It will also be SAMPLED by the `TextureCapturer` and `TextureResizer`.
    .usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
    // Use nannou's default multisampling sample count.
    .sample_count(sample_count)
    // Use a spacious 16-bit linear sRGBA format suitable for high quality drawing.
    .format(wgpu::TextureFormat::Rgba16Float)
    // Build it!
    .build(device);
  let texture_view = texture.view().build();

  let descriptor = texture.descriptor();
  let renderer =
    nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);

  // Create the texture capturer.
  let texture_capturer = wgpu::TextureCapturer::default();

  // Create the texture reshaper for GUI display
  let texture_reshaper = wgpu::TextureReshaper::new(
    device,
    &texture_view,
    sample_count,
    texture.sample_type(),
    sample_count,
    texture.format(),
  );

  let background_texture = options.background_path.map(|background_path| {
    wgpu::Texture::from_path(&window, images_path(app, background_path)).unwrap()
  });
  // Make sure the directory where we will save images to exists.
  std::fs::create_dir_all(&capture_directory(app)).unwrap();
  DynamicBaseModel {
    window_id,
    draw,
    texture,
    renderer,
    texture_capturer,
    texture_reshaper,
    background_texture,
    current_frame: 0,
    recording: false,
    seed: random(),
    fps: options.fps,
    n_sec: options.n_sec,
  }
}

pub fn make_dynamic_nannou_app<T: 'static + DynamicArtwork>() -> nannou::app::Builder<T> {
  nannou::app(model).update(update).exit(exit)
}

fn model<T: 'static + DynamicArtwork>(app: &App) -> T {
  T::new(make_base_model::<T>(app, T::get_options()))
}

fn update<T: DynamicArtwork>(app: &App, model: &mut T, _update: Update) {
  // Rendering texture as background
  if let Some(background_texture) = &model.get_model().background_texture {
    let sampler = wgpu::SamplerBuilder::new()
      .address_mode(wgpu::AddressMode::ClampToBorder)
      .into_descriptor();
    let draw = &model.get_model().draw;
    draw.sampler(sampler);
    draw.texture(&background_texture);
  }

  // Drawing artwork
  let [w, _] = model.get_model().texture.size();
  let n_frames = model.get_model().fps * model.get_model().n_sec;
  let elapsed_frames = if model.get_model().recording {
    model.get_model().current_frame
  } else {
    let pos = 2. * (4. * app.mouse.x + (w as f32)) / w as f32;
    (pos * n_frames as f32) as u32 % n_frames
  };
  let t = map_range(elapsed_frames as f64, 0., n_frames as f64, 0., 1.);
  model.draw_at_time(t);

  let window = app.window(model.get_model().window_id).unwrap();
  let device = window.device();
  let base_model = model.get_model_mut();

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
  let snapshot =
    model
      .get_model()
      .texture_capturer
      .capture(device, &mut encoder, &model.get_model().texture);
  window.queue().submit(Some(encoder.finish()));

  if model.get_model().recording {
    record(app, model, elapsed_frames, snapshot);
  } else {
    println!("Rendering frame {}/{}", elapsed_frames, n_frames);
  }
}

fn view<T: DynamicArtwork>(_app: &App, model: &T, frame: Frame) {
  model
    .get_model()
    .texture_reshaper
    .encode_render_pass(frame.texture_view(), &mut frame.command_encoder());
}

// Wait for capture to finish.
fn exit<T: DynamicArtwork>(app: &App, model: T) {
  let window = app.window(model.get_model().window_id).unwrap();
  let device = window.device();
  model
    .get_model()
    .texture_capturer
    .await_active_snapshots(device)
    .unwrap();
}

fn key_pressed<T: DynamicArtwork>(_app: &App, model: &mut T, key: Key) {
  let base_model = model.get_model_mut();
  match key {
    Key::S => {
      base_model.seed = random();
    }
    Key::R => {
      if base_model.recording {
        base_model.recording = false;
      } else {
        base_model.recording = true;
        base_model.current_frame = 0;
      }
    }
    _ => {}
  }
}

fn record<T: DynamicArtwork>(
  app: &App,
  model: &mut T,
  elapsed_frames: u32,
  snapshot: wgpu::TextueSnapshot,
) {
  let mut base_model = model.get_model_mut();
  let n_frames = base_model.fps * base_model.n_sec;

  let path = captured_frame_path(app, elapsed_frames.to_string());
  println!(
    "Saving frame {}/{} into {} ...",
    elapsed_frames,
    n_frames,
    path.to_str().unwrap()
  );
  snapshot
    .read(move |result| {
      let image = result.expect("Failed to map texture memory").to_owned();
      image
        .save(&path)
        .expect("Failed to save texture to png image");
    })
    .unwrap();

  base_model.current_frame += 1;
  if base_model.current_frame > n_frames {
    base_model.recording = false;
  }
}

fn capture_directory(app: &App) -> std::path::PathBuf {
  app
    .project_path()
    .expect("could not locate project_path")
    .join("out")
    .join(app.exe_name().unwrap())
    .join("frames")
}

fn captured_frame_path(app: &App, name: String) -> std::path::PathBuf {
  capture_directory(app).join(name).with_extension("png")
}

fn images_path(app: &App, path: PathBuf) -> PathBuf {
  app.assets_path().unwrap().join("images").join(path)
}
