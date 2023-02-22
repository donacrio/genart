// TODO(refactor): create paths as linestring then use LineInterpolatePoint trait for uniform speed
use geo::{coord, ChaikinSmoothing, Coord, LineString, Rect};
use nannou::{
  color::IntoLinSrgba,
  prelude::{map_range, Hsla, Key, Vec2, BLACK, WHITE},
  App, Draw,
};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use utils::app::{make_dynamic_artwork, Artwork, ArtworkOptions, BaseModel, DynamicArtwork};

const FPS: u32 = 60;
const N_SEC: u32 = 10;
const DEPTH: usize = 0;
const N: usize = 2;

fn main() {
  make_dynamic_artwork::<Model>().run();
}

struct Model {
  base_model: BaseModel,
  paths: Vec<LineString<f32>>,
  n: usize,
  depth: usize,
  current_frame: u32,
}

impl Artwork for Model {
  fn new(base_model: BaseModel) -> Self {
    let mut model = Self {
      base_model,
      paths: vec![],
      n: N,
      depth: DEPTH,
      current_frame: 0,
    };
    model.compute_paths();
    model
  }
  fn get_options() -> ArtworkOptions {
    ArtworkOptions {
      ..ArtworkOptions::default()
    }
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
  fn key_pressed(&mut self, _app: &App, key: Key) {
    match key {
      Key::Up => {
        self.depth += 1;
        self.compute_paths();
      }
      Key::Down => {
        if self.depth > 0 {
          self.depth -= 1
        }
        self.compute_paths();
      }
      Key::Left => {
        if self.n > 0 {
          self.n -= 1
        }
        self.compute_paths();
      }
      Key::Right => {
        self.n += 1;
        self.compute_paths();
      }
      Key::S => self.compute_paths(),
      _ => {}
    }
  }
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

    draw.background().color(BLACK);

    self
      .paths
      .iter()
      .for_each(|line_string| draw_line_string(line_string, draw, t));
  }
}

impl Model {
  fn compute_paths(&mut self) {
    let mut rng = StdRng::seed_from_u64(self.base_model.seed);

    let [w_w, w_h] = self.base_model.texture.size();
    let w = w_w as f32 * 0.9;
    let h = w_h as f32 * 0.9;
    let rect = Rect::new(
      coord! {x:-(w / 2.0), y:-(h as f32 / 2.0) },
      coord! {x:w as f32 / 2.0, y:h as f32 / 2.0 },
    );

    let mut tiles = vec![rect];
    for _ in 0..self.depth {
      tiles = tile(tiles);
    }

    // TODO: really slow and dirty code
    let paths = tiles
      .iter()
      .flat_map(|tile| {
        let mut vec: Vec<usize> = (0..self.n).collect();
        vec.shuffle(&mut rng);
        let transpositions = compute_transpositions(vec);
        // Create nxn matrix representing each permutation path
        // Columns i represents the path for element i
        let paths = (0..self.n).fold(
          vec![(0..self.n).collect(), (0..self.n).collect()],
          |mut acc: Vec<Vec<usize>>, i| {
            let mut current = acc.last().unwrap().clone();
            if let Some((a, b)) = transpositions.get(i) {
              current.swap(*a, *b);
            }
            acc.push(current);
            acc
          },
        );
        // Transpose paths so line i represents the path for value i
        let paths = transpose(paths);
        paths
          .iter()
          .map(|path| create_line_string(path, self.n, tile.center(), tile.width(), tile.height()))
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
    self.paths = paths;
  }
}

fn compute_transpositions(vec: Vec<usize>) -> Vec<(usize, usize)> {
  let mut transpositions: Vec<(usize, usize)> = Vec::new();
  let mut visited = vec![false; vec.len()];
  let mut index = 0;
  while index < vec.len() {
    if !visited[index] {
      let mut value = index;
      while vec[value] != index {
        transpositions.push((value, vec[value]));
        value = vec[value];
        visited[value] = true;
      }
    }
    index += 1;
  }
  transpositions
}

fn tile(tiles: Vec<Rect<f32>>) -> Vec<Rect<f32>> {
  tiles
    .iter()
    .flat_map(|rect| rect.split_x().map(|x_rect| x_rect.split_y()))
    .flatten()
    .collect()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
  assert!(!v.is_empty());
  let len = v[0].len();
  let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
  (0..len)
    .map(|_| {
      iters
        .iter_mut()
        .map(|n| n.next().unwrap())
        .collect::<Vec<T>>()
    })
    .collect()
}

fn create_line_string(
  path: &[usize],
  n_elements: usize,
  center: Coord<f32>,
  width: f32,
  height: f32,
) -> LineString<f32> {
  let line_string = path
    .iter()
    .enumerate()
    // Mapping path on a grid
    .map(|(depth, index)| (*index as f32, depth as f32).into())
    // Scaling for the current tile
    .map(|coord: Coord<f32>| {
      coord! {x:(coord.x / (n_elements-1) as f32 - 0.5) * width * 0.8 + center.x,
      y:(coord.y / (n_elements+1) as f32 - 0.5) * height * 0.8 + center.y,}
    })
    .collect::<LineString<f32>>();
  let first_coord = *line_string.coords().next().unwrap();
  let line_string = line_string
    .coords()
    .fold(vec![first_coord], |mut acc, coord| {
      let previous_coord = *acc.last().unwrap();
      if previous_coord != *coord {
        acc.push(coord! {x:previous_coord.x, y: coord.y});
      }
      acc.push(*coord);
      acc
    })
    .into_iter()
    .collect::<LineString<f32>>();
  line_string.chaikin_smoothing(4)
}

fn draw_line_string(line_string: &LineString<f32>, draw: &Draw, t: f64) {
  let n_coords = line_string.coords().count();
  let first_element = map_range(t, 0.0, 1.0, n_coords - 1, 0);

  let points = line_string.coords().enumerate().map(|(index, coord)| {
    let distance_to_first = if index <= first_element {
      index + n_coords - first_element
    } else {
      index - first_element
    };
    let alpha = map_range(
      (1.0 - (distance_to_first as f32 / n_coords as f32)).exp(),
      1.0,
      std::f32::consts::E,
      -0.25,
      1.0,
    );

    let color = Hsla::new(0.0, 0.0, 1.0, alpha);
    (Vec2::from(coord.x_y()), color.into_lin_srgba())
  });
  let start = line_string.0.get(first_element).unwrap();

  draw
    .ellipse()
    .xy(Vec2::from(start.x_y()))
    .w_h(5.0, 5.0)
    .color(WHITE);

  draw.polyline().weight(5.0).points_colored(points);
}
