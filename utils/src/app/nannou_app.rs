use nannou::{
  prelude::{Key, Update},
  wgpu, window, App, Draw, Frame,
};
use rand::random;
use std::path::PathBuf;

const TEXTURE_SIZE: [u32; 2] = [2160, 2160];
const RENDER_SIZE: [u32; 2] = [540, 540];

pub trait NannouApp {
  fn new(model: BaseModel) -> Self;
  fn get_options() -> NannouAppOptions;
  fn get_base_model(&self) -> &BaseModel;
  fn get_base_model_mut(&mut self) -> &mut BaseModel;
  fn current_frame_name(&self) -> String;
  fn key_pressed(&mut self, app: &App, key: Key);
  fn update(&mut self, app: &App);
}

// pub trait AppUpdate<T> {
//   fn update(model: &mut T, app: &App);
// }

pub struct NannouAppOptions {
  pub texture_size: [u32; 2],
  pub render_size: [u32; 2],
  pub background_path: Option<PathBuf>,
}

impl Default for NannouAppOptions {
  fn default() -> Self {
    Self {
      texture_size: TEXTURE_SIZE,
      render_size: RENDER_SIZE,
      background_path: None,
    }
  }
}

pub struct BaseModel {
  window_id: window::Id,
  pub draw: Draw,
  pub texture: wgpu::Texture,
  renderer: nannou::draw::Renderer,
  texture_capturer: wgpu::TextureCapturer,
  texture_reshaper: wgpu::TextureReshaper,
  background_texture: Option<wgpu::Texture>,
  pub seed: u64,
}

fn make_base_model<T: 'static + NannouApp>(app: &App, options: NannouAppOptions) -> BaseModel {
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
  let seed = rand::random();
  // Make sure the directory where we will save images to exists.
  std::fs::create_dir_all(&capture_directory(app)).unwrap();
  BaseModel {
    window_id,
    draw,
    texture,
    renderer,
    texture_capturer,
    texture_reshaper,
    background_texture,
    seed,
  }
}

pub fn make_base_nannou_app<T: 'static + NannouApp>() -> nannou::app::Builder<T> {
  nannou::app(model).update(update).exit(exit)
}

fn model<T: 'static + NannouApp>(app: &App) -> T {
  T::new(make_base_model::<T>(app, T::get_options()))
}

fn update<T: NannouApp>(app: &App, model: &mut T, _update: Update) {
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

  println!("Computing artwork...");
  model.update(app);

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

fn view<T: NannouApp>(_app: &App, model: &T, frame: Frame) {
  model
    .get_base_model()
    .texture_reshaper
    .encode_render_pass(frame.texture_view(), &mut frame.command_encoder());
}

// Wait for capture to finish.
fn exit<T: NannouApp>(app: &App, model: T) {
  let window = app.window(model.get_base_model().window_id).unwrap();
  let device = window.device();
  model
    .get_base_model()
    .texture_capturer
    .await_active_snapshots(device)
    .unwrap();
}

fn key_pressed<T: NannouApp>(app: &App, model: &mut T, key: Key) {
  let base_model = model.get_base_model_mut();
  if key == Key::S {
    let seed = random();
    base_model.seed = seed;
  }
  model.key_pressed(app, key);
}

fn capture_directory(app: &App) -> std::path::PathBuf {
  app
    .project_path()
    .expect("could not locate project_path")
    .join("out")
    .join(app.exe_name().unwrap())
}

fn captured_frame_path(app: &App, name: &str) -> std::path::PathBuf {
  capture_directory(app).join(name).with_extension("png")
}

fn images_path(app: &App, path: PathBuf) -> PathBuf {
  app.assets_path().unwrap().join("images").join(path)
}
