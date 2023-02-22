use std::{collections::VecDeque, f32::consts::FRAC_PI_2};

use geo::{coord, Coord, LineInterpolatePoint, LineString, Rect};
use nannou::{
  prelude::{Hsl, Hsla, Key, Vec2, Vec2Rotate, PI},
  App,
};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use utils::app::{make_dynamic_artwork, Artwork, ArtworkOptions, BaseModel, DynamicArtwork};

const FPS: u32 = 60;
const N_SEC: u32 = 30;

const N_MAIN_OBITS: usize = 50;
const N_CHILDREN_OBITS: usize = 7;
const CHILDREN_RADIUS_FACTOR: f32 = 1.0;
const MAX_POINTS: usize = 10;
const POINT_SIZE: f32 = 5.0;
const ALPHA_FACTOR: f32 = 0.75;
const SEED: Option<u64> = None;

fn main() {
  make_dynamic_artwork::<Model>().run();
}

type Transposition = (usize, usize);

struct Model {
  base_model: BaseModel,
  current_frame: u32,
  main_permutation_path: Vec<LineString<f32>>,
  permutations_paths: Vec<Vec<LineString<f32>>>,
  permutations_points: Vec<Vec<VecDeque<Vec2>>>,
}

impl Artwork for Model {
  fn new(base_model: BaseModel) -> Self {
    let mut model = Self {
      base_model,
      current_frame: 0,
      main_permutation_path: vec![],
      permutations_paths: vec![],
      permutations_points: vec![],
    };
    model.compute_paths();
    model
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

    let background_color = Hsl::new(40.0, 0.35, 0.93); // PAPER: hsl(40,35%,93%)
    draw.background().color(background_color);

    // TODO: Compute new center for each permutation and rotate all points
    self
      .permutations_paths
      .iter()
      .enumerate()
      .for_each(|(i, permutation_paths)| {
        // Find the main path new center
        let center = self
          .main_permutation_path
          .get(i)
          .unwrap()
          .line_interpolate_point(t as f32)
          .unwrap();
        let center = Vec2::from(center.x_y());
        // Compute polar coordinates angle (between -pi/2 and pi/2)
        let theta = (center.x / center.y).atan();
        let theta = 2.0 * (theta + FRAC_PI_2);
        permutation_paths.iter().enumerate().for_each(|(j, path)| {
          if let Some(point) = path.line_interpolate_point(t as f32) {
            // Update queue with the next point on the path
            let point = Vec2::from(point.x_y());
            let point = point.rotate(-theta) + center;
            let queue = self
              .permutations_points
              .get_mut(i)
              .unwrap()
              .get_mut(j)
              .unwrap();
            if queue.len() == MAX_POINTS {
              queue.pop_front();
            }
            queue.push_back(point);
          }
        })
      });

    self
      .permutations_points
      .iter()
      .for_each(|permutation_points| {
        permutation_points.iter().for_each(|points| {
          points.iter().enumerate().for_each(|(i, point)| {
            // Rotate the translate the new point
            let alpha = i as f32 / (MAX_POINTS - 1) as f32;
            let alpha = ALPHA_FACTOR * alpha.powi(2);
            let color = Hsla::new(204.0, 0.188, 0.261, alpha); // charcoal hsl(204°, 18.8%, 26.1%)
            draw
              .ellipse()
              .xy(*point)
              .w_h(POINT_SIZE, POINT_SIZE)
              .color(color);
          })
        })
      })
  }
}

impl Model {
  fn compute_paths(&mut self) {
    let mut rng = StdRng::seed_from_u64(SEED.unwrap_or(self.base_model.seed));

    let [w_w, w_h] = self.base_model.texture.size();
    let w = w_w as f32 * 0.9;
    let h = w_h as f32 * 0.9;

    let rect = Rect::new(
      coord! {x:-(w / 2.0), y:-(h as f32 / 2.0) },
      coord! {x:w as f32 / 2.0, y:h as f32 / 2.0 },
    );

    let mut vec: Vec<usize> = (0..N_MAIN_OBITS).collect();
    vec.shuffle(&mut rng);
    let main_permutation = compute_transpositions(vec);
    self.main_permutation_path = compute_paths(N_MAIN_OBITS, main_permutation, w / 2.0)
      .into_iter()
      .map(compute_linestring_from_path)
      .collect();

    self.permutations_paths = (0..N_MAIN_OBITS)
      .map(|_| {
        // Initialize the poits queues
        self
          .permutations_points
          .push(vec![VecDeque::new(); N_CHILDREN_OBITS]);
        let mut vec: Vec<usize> = (0..N_CHILDREN_OBITS).collect();
        vec.shuffle(&mut rng);
        let transpositions = compute_transpositions(vec);
        println!("{:#?}", transpositions);
        compute_paths(
          N_CHILDREN_OBITS,
          transpositions,
          CHILDREN_RADIUS_FACTOR * rect.width() / N_MAIN_OBITS as f32,
        )
        .into_iter()
        .map(compute_linestring_from_path)
        .collect() // TODO: add smoothing?
      })
      .collect();
  }
}

fn compute_transpositions(vec: Vec<usize>) -> Vec<Transposition> {
  let mut transpositions: Vec<Transposition> = Vec::new();
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

#[derive(Debug)]
enum PathPart {
  Straight(f32, f32, f32),
  Arc(f32, f32, f32),
}

fn compute_paths(
  n_elements: usize,
  transpositions: Vec<Transposition>,
  max_radius: f32,
) -> Vec<Vec<PathPart>> {
  let n_transpositions = transpositions.len();

  let mut paths = Vec::new();
  for i in 0..n_elements {
    let mut path = Vec::new();
    let mut orbit = i;
    for j in 0..=n_transpositions {
      let radius = (orbit + 1) as f32 / (n_elements + 1) as f32 * max_radius;
      let theta_start = 2.0 * PI * j as f32 / (n_transpositions + 1) as f32 + FRAC_PI_2;
      let theta_end = 2.0 * PI * (j + 1) as f32 / (n_transpositions + 1) as f32 + FRAC_PI_2;
      path.push(PathPart::Arc(radius, theta_start, theta_end));

      if let Some(next_transposition) = transpositions.get(j) {
        let mut next_orbit = None;
        if next_transposition.0 == orbit {
          next_orbit = Some(next_transposition.1);
        }
        if next_transposition.1 == orbit {
          next_orbit = Some(next_transposition.0);
        }
        if let Some(next_orbit) = next_orbit {
          let next_radius = (next_orbit + 1) as f32 / (n_elements + 1) as f32 * max_radius;
          path.push(PathPart::Straight(radius, next_radius, theta_end));
          orbit = next_orbit;
        }
      }
    }
    paths.push(path)
  }
  paths
}

fn compute_linestring_from_path(path: Vec<PathPart>) -> LineString<f32> {
  path
    .iter()
    .flat_map(|part| match part {
      PathPart::Straight(radius_start, radius_end, theta) => {
        let support: Coord<f32> = (theta.cos(), theta.sin()).into();
        vec![(support * *radius_start), (support * *radius_end)]
      }
      PathPart::Arc(radius, thetha_start, thetha_end) => {
        let length = radius * (thetha_start - thetha_end).abs();
        let n_samples = length as usize;
        let theta_incr = (thetha_start - thetha_end).abs() / n_samples as f32;
        (0..n_samples)
          .map(|i| {
            let theta = thetha_start + i as f32 * theta_incr;
            let support: Coord<f32> = (theta.cos(), theta.sin()).into();
            support * *radius
          })
          .collect()
      }
    })
    .collect()
}
