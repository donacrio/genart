use geo::{coord, line_string, Coord, LineString, MultiLineString, Rect, Rotate};
use nannou::{
  prelude::{map_range, Key, Vec2, BLACK, PI, WHITE},
  App,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::f32::consts::FRAC_PI_2;
use utils::app::{make_dynamic_artwork, Artwork, ArtworkOptions, BaseModel, DynamicArtwork};

const FPS: u32 = 60;
const N_SEC: u32 = 20;

const DEPTH: usize = 4;

const N_ROTATIONS: usize = 60;
const ROTATION_MAX: f32 = 90.0;

fn main() {
  make_dynamic_artwork::<Model>().run();
}

struct Model {
  base_model: BaseModel,
  current_frame: u32,
  lines: Vec<MultiLineString<f32>>,
  rotating_elements: Vec<usize>,
  current_rotating: usize,
}

impl Artwork for Model {
  fn new(base_model: BaseModel) -> Self {
    let mut rng = StdRng::seed_from_u64(base_model.seed);

    let [w_w, w_h] = base_model.texture.size();
    let width = w_w as f32 * 0.9;
    let height = w_h as f32 * 0.9;
    let lines = create_lines(width, height, &mut rng);

    let rotating_elements = (0..N_ROTATIONS)
      .map(|_| rng.gen_range(0..lines.len()))
      .collect();

    Self {
      base_model,
      current_frame: 0,
      lines,
      rotating_elements,
      current_rotating: 0,
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
    format!("frame_{}", self.current_frame)
  }
  fn key_pressed(&mut self, _app: &App, _key: Key) {}
}

impl DynamicArtwork for Model {
  fn fps(&self) -> u32 {
    FPS
  }
  fn n_sec(&self) -> u32 {
    N_SEC
  }
  fn current_frame(&mut self) -> &mut u32 {
    &mut self.current_frame
  }
  fn draw_at_time(&mut self, t: f64) {
    let draw = &self.base_model.draw;

    draw.background().color(WHITE);

    let rotation = map_range(
      t as f32,
      self.current_rotating as f32 / N_ROTATIONS as f32,
      (self.current_rotating + 1) as f32 / N_ROTATIONS as f32,
      0.0,
      ROTATION_MAX,
    );

    self
      .lines
      .iter()
      .enumerate()
      .flat_map(|(i, lines)| {
        if self.rotating_elements[0..self.current_rotating].contains(&i) {
          return lines.rotate_around_center(ROTATION_MAX);
        }
        if i == self.rotating_elements[self.current_rotating] {
          return lines.rotate_around_center(rotation);
        }
        lines.clone()
      })
      .map(|line| {
        line
          .into_iter()
          .map(|coord| Vec2::from(coord.x_y()))
          .collect::<Vec<_>>()
      })
      .for_each(|coords| {
        draw
          .polyline()
          .weight(60.0)
          .color(WHITE)
          .points(coords.clone());
        draw.polyline().weight(20.0).color(BLACK).points(coords);
      });

    if t >= (self.current_rotating + 1) as f64 / N_ROTATIONS as f64 {
      self.current_rotating += 1;
    }
    if self.current_rotating == N_ROTATIONS {
      self.current_rotating = 0;
    }
  }
}

fn create_lines(width: f32, height: f32, rng: &mut StdRng) -> Vec<MultiLineString<f32>> {
  let rect = Rect::new(
    coord! {x:-(width / 2.0), y:-(height as f32 / 2.0) },
    coord! {x:width as f32 / 2.0, y:height as f32 / 2.0 },
  );

  let rects = (0..DEPTH).fold(vec![rect], |r, _| tile(r));
  let tiles = rects.into_iter().map(|rect| Tile::new(rect, rng));

  tiles.map(|tile| tile.create_lines()).collect::<Vec<_>>()
}

enum Position {
  Top,
  Bottom,
  Left,
  Right,
}

impl Position {
  fn get_position_coords(&self, rect: &Rect<f32>) -> Coord<f32> {
    let (x_min, y_min) = rect.min().into();
    let (x_max, y_max) = rect.max().into();
    match self {
      Position::Top => coord! {x:(x_min+x_max)/2.,y:y_max},
      Position::Bottom => coord! {x:(x_min+ x_max)/2.,y:y_min},
      Position::Left => coord! {x:x_min,y:(y_min+y_max)/2.},
      Position::Right => coord! {x:x_max,y:(y_min+y_max)/2.},
    }
  }
}

enum Pipe {
  TopBottomLeftRight,
  LeftRightTopBottom,
  TopLeftBottomRigh,
  TopRightBottomLeft,
}

impl Pipe {
  fn create_lines(&self, rect: &Rect<f32>) -> MultiLineString<f32> {
    // let (x_min, y_min) = rect.min().into();
    // let (x_max, y_max) = rect.max().into();
    match self {
      Pipe::TopLeftBottomRigh => {
        let tl: Coord<f32> = (rect.min().x, rect.max().y).into();
        let br: Coord<f32> = (rect.max().x, rect.min().y).into();
        MultiLineString::new(vec![
          create_arc_line(tl, rect.width() / 2., 1.5 * PI, 2.0 * PI),
          create_arc_line(br, rect.width() / 2., FRAC_PI_2, PI),
        ])
      }
      Pipe::TopRightBottomLeft => MultiLineString::new(vec![
        create_arc_line(rect.max(), rect.width() / 2., PI, 1.5 * PI),
        create_arc_line(rect.min(), rect.width() / 2., 0.0, FRAC_PI_2),
      ]),
      _ => MultiLineString::new(vec![
        line_string![
          Position::Top.get_position_coords(rect),
          Position::Bottom.get_position_coords(rect)
        ],
        line_string![
          Position::Left.get_position_coords(rect),
          Position::Right.get_position_coords(rect)
        ],
      ]),
    }
  }
}

struct Tile {
  rect: Rect<f32>,
  pipe: Pipe,
  _rotation: f32,
  _ratio: f32,
}

impl Tile {
  fn new(rect: Rect<f32>, rng: &mut StdRng) -> Self {
    Self {
      rect,
      pipe: gen_pipe(rng),
      _rotation: 0.0,
      _ratio: 1.0,
    }
  }

  fn create_lines(&self) -> MultiLineString<f32> {
    self.pipe.create_lines(&self.rect)
  }
}

fn gen_pipe(rng: &mut StdRng) -> Pipe {
  match rng.gen_range(0..4) {
    0 => Pipe::TopBottomLeftRight,
    1 => Pipe::TopLeftBottomRigh,
    2 => Pipe::TopRightBottomLeft,
    _ => Pipe::LeftRightTopBottom,
  }
}

fn tile(rectangles: Vec<Rect<f32>>) -> Vec<Rect<f32>> {
  rectangles
    .iter()
    .flat_map(|child| child.split_x().map(|x_child| x_child.split_y()))
    .flatten()
    .collect()
}

fn create_arc_line(
  center: Coord<f32>,
  radius: f32,
  theta_start: f32,
  theta_end: f32,
) -> LineString<f32> {
  let n_points = 10;
  let theta_incr = (theta_end - theta_start).abs() / (n_points - 1) as f32;
  let mut coords: Vec<Coord<f32>> = vec![];
  for i in 0..n_points {
    let theta = theta_start + i as f32 * theta_incr;
    coords.push(center + (radius * theta.cos(), radius * theta.sin()).into());
  }
  LineString::new(coords)
}
