use geo::{coord, line_string, Coord, LineString, MultiLineString, Rect};
use nannou::{
  prelude::{Key, Vec2, BLACK, PI, WHITE},
  App,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::f32::consts::FRAC_PI_2;
use utils::app::{make_static_artwork, Artwork, ArtworkOptions, BaseModel, StaticArtwork};

fn main() {
  make_static_artwork::<Model>().run();
}

struct Model {
  base_model: BaseModel,
  depth: usize,
  line_width: f32,
}

impl Artwork for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      depth: 4,
      line_width: 10.0,
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
    format!("frame_{}", self.base_model.seed)
  }
  fn key_pressed(&mut self, _app: &App, key: Key) {
    match key {
      Key::Up => self.depth += 1,
      Key::Down => {
        if self.depth > 0 {
          self.depth -= 1
        }
      }
      Key::Right => self.line_width += 1.0,
      Key::Left => {
        if self.line_width > 1.0 {
          self.line_width -= 1.0;
        }
      }
      _ => {}
    }
  }
}

impl StaticArtwork for Model {
  fn draw(&mut self) {
    let draw = &self.base_model.draw;

    draw.background().color(WHITE);

    let mut rng = StdRng::seed_from_u64(self.base_model.seed);

    let [w_w, w_h] = self.base_model.texture.size();
    let w = w_w as f32 * 0.9;
    let h = w_h as f32 * 0.9;

    let rect = Rect::new(
      coord! {x:-(w / 2.0), y:-(h as f32 / 2.0) },
      coord! {x:w as f32 / 2.0, y:h as f32 / 2.0 },
    );

    let rects = (0..self.depth).fold(vec![rect], |r, _| tile(r));
    let tiles = rects.into_iter().map(|rect| Tile::new(rect, &mut rng));
    let lines = tiles.flat_map(|tile| tile.create_lines());
    lines
      .map(|line| line.into_iter().map(|coord| Vec2::from(coord.x_y())))
      .for_each(|coords| {
        draw
          .polyline()
          .weight(self.line_width * 2.)
          .color(WHITE)
          .points(coords.clone());
        draw
          .polyline()
          .weight(self.line_width)
          .color(BLACK)
          .points(coords);
      });
  }
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
