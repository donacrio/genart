use geo::Coord;
use rand::Rng;
use rand_distr::{Bernoulli, Distribution};
use std::collections::VecDeque;

use crate::data_structures::graph::{Graph, Node};

pub trait SpaceTile {
  fn new(min: Coord, max: Coord) -> Self;
  fn width(&self) -> f64;
  fn height(&self) -> f64;
  fn min(&self) -> Coord;
  fn max(&self) -> Coord;
}

pub type Space<T> = Graph<T>;

pub fn compute_space<T: SpaceTile>(root: T, max_children: u32, min_size: f64) -> Space<T> {
  let mut space = Space::default();
  let root = Node::new(root);
  split_bfs(&mut space, root, max_children, min_size);
  space
}

fn split_bfs<T: SpaceTile>(space: &mut Space<T>, root: Node<T>, max_children: u32, min_size: f64) {
  let parent_index = space.add_node(root);

  let mut queue = VecDeque::new();
  queue.push_back(parent_index);

  while let Some(index) = queue.pop_front() {
    // Stop condition on spaces lenght
    if space.len() >= max_children as usize {
      return;
    }
    let tile = space
      .get_node(index)
      .expect("Node with index  does not exists")
      .content();

    // Only divide rectangle if it's not too small
    if tile.width() > min_size && tile.height() > min_size {
      let (child_1, child_2) = divide(tile);
      println!(
        "child_1 {{
            min: {:#?},
            max:{:#?}
        }}",
        child_1.min(),
        child_1.max()
      );
      println!(
        "child_2 {{
            min: {:#?},
            max:{:#?}
        }}",
        child_2.min(),
        child_2.max()
      );

      let child_index_1 = space.add_node(Node::new(child_1));
      let child_index_2 = space.add_node(Node::new(child_2));
      space.add_edge(index, child_index_1);
      space.add_edge(index, child_index_2);

      queue.push_back(child_index_1);
      queue.push_back(child_index_2);
    }
  }
}

fn divide<T: SpaceTile>(tile: &T) -> (T, T) {
  let axis = Bernoulli::new(0.5).unwrap().sample(&mut rand::thread_rng());

  match axis {
    true => {
      let y = rand::thread_rng().gen_range(0.0..tile.height());
      let min_1 = tile.min();
      let max_1 = (tile.max().x, tile.min().y + y).into();
      let min_2 = (tile.min().x, tile.min().y + y).into();
      let max_2 = tile.max();
      (T::new(min_1, max_1), T::new(min_2, max_2))
    }
    false => {
      let x = rand::thread_rng().gen_range(0.0..tile.width());
      let min_1 = tile.min();
      let max_1 = (tile.min().x + x, tile.max().y).into();
      let min_2 = (tile.min().x + x, tile.min().y).into();
      let max_2 = tile.max();
      (T::new(min_1, max_1), T::new(min_2, max_2))
    }
  }
}
