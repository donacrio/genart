// TODO(refactor): create paths as linestring then use LineInterpolatePoint trait for uniform speed
use geo::{coord, ChaikinSmoothing, Coord, LineString, Rect};
use nannou::{
  prelude::{Key, Vec2, BLACK, WHITE},
  App,
};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use utils::app::{
  make_static_artwork, update_static, BaseModel, NannouApp, NannouAppOptions, StaticArtwork,
};

const DEPTH: usize = 0;
const N: usize = 2;
const POINT_SIZE: f32 = 10.0;

fn main() {
  make_static_artwork::<Model>().run();
}

struct Model {
  base_model: BaseModel,
  n: usize,
  depth: usize,
  current_frame: u32,
}

impl NannouApp for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      n: N,
      depth: DEPTH,
      current_frame: 0,
    }
  }
  fn get_options() -> NannouAppOptions {
    NannouAppOptions {
      ..NannouAppOptions::default()
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
      }
      Key::Down => {
        if self.depth > 0 {
          self.depth -= 1
        }
      }
      Key::Left => {
        if self.n > 0 {
          self.n -= 1
        }
      }
      Key::Right => {
        self.n += 1;
      }
      _ => {}
    }
  }
  fn update(&mut self, _app: &App) {
    update_static(self)
  }
}

impl StaticArtwork for Model {
  fn draw(&self) {
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

    let draw = &self.base_model.draw;
    draw.background().color(WHITE);

    paths
      .iter()
      .map(|line_string| line_string.coords())
      .map(|coords| coords.map(|coord| Vec2::from(coord.x_y())))
      .for_each(|points| {
        draw
          .polyline()
          .weight(POINT_SIZE)
          .points(points)
          .color(BLACK);
      });
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
