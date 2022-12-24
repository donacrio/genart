use std::collections::VecDeque;

type NodeIndex = usize;
type EdgeIndex = usize;

pub struct Graph<T> {
  nodes: Vec<Node<T>>,
  edges: Vec<Edge>,
  bfs_traversal: Option<Vec<NodeIndex>>,
}

pub struct Node<T> {
  content: T,
}

impl<T> Node<T> {
  pub fn new(content: T) -> Self {
    Self { content }
  }

  pub fn content(&self) -> &T {
    &self.content
  }
}

#[derive(Default)]
pub struct Edge {
  next_edges: Vec<EdgeIndex>,
}

impl<T> Default for Graph<T> {
  fn default() -> Self {
    Self {
      nodes: Vec::new(),
      edges: Vec::new(),
      bfs_traversal: None,
    }
  }
}
impl<T> Graph<T> {
  pub fn len(&self) -> usize {
    self.nodes.len()
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  pub fn get_node(&self, index: NodeIndex) -> Option<&Node<T>> {
    self.nodes.get(index)
  }

  pub fn add_node(&mut self, node: Node<T>) -> NodeIndex {
    let index = self.nodes.len();
    self.nodes.push(node);
    self.edges.push(Edge::default());
    index
  }

  pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
    if source > self.nodes.len() || target > self.nodes.len() {
      return;
    }
    let edge = self.edges.get_mut(source).unwrap();
    edge.next_edges.push(target);
  }

  pub fn bfs(&mut self) -> &Vec<NodeIndex> {
    if self.bfs_traversal.is_none() {
      let mut traversal = Vec::new();
      if self.nodes.is_empty() {
        self.bfs_traversal = Some(traversal);
      } else {
        let mut queue = VecDeque::new();
        queue.push_back(0);
        while let Some(index) = queue.pop_front() {
          traversal.push(index);
          let edge = self.edges.get(index).unwrap();
          for next_index in edge.next_edges.iter() {
            queue.push_back(*next_index)
          }
        }
        self.bfs_traversal = Some(traversal);
      }
    }
    self.bfs_traversal.as_ref().unwrap()
  }

  pub fn leafs(&mut self) -> Vec<NodeIndex> {
    self
      .bfs()
      .to_vec()
      .iter()
      .filter(|index| self.is_leaf(**index))
      .copied()
      .collect()
  }

  fn is_leaf(&self, index: NodeIndex) -> bool {
    let edges = self.edges.get(index).unwrap();
    edges.next_edges.is_empty()
  }
}
