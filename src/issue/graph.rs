#![allow(dead_code, missing_docs)]

use std::{
  collections::{HashMap, HashSet, VecDeque},
  hash::Hash,
};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Edge<T: Hash + Eq + Clone> {
  node: T,
  cost: usize,
}

impl<T: Hash + Eq + Clone> Edge<T> {
  fn new(to: T, cost: usize) -> Self {
    Self { node: to, cost }
  }
}

type Edges<T> = Vec<Edge<T>>;

#[derive(Clone)]
pub struct Graph<T: Hash + Eq + Clone> {
  /* edges_from[vex] := edges_start_with_vex */
  edges_from: HashMap<T, Edges<T>>,

  /* edges_to[vex] := edges_end_with_vex */
  edges_to: HashMap<T, Edges<T>>,

  /* num of vertex */
  vertex_num: usize,

  /* num of edge */
  edge_num: usize,
}

impl<T: Hash + Eq + Clone> Default for Graph<T> {
  fn default() -> Self {
    Self {
      edges_from: HashMap::new(),
      edges_to: HashMap::new(),
      vertex_num: 0,
      edge_num: 0,
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrType<T: Eq> {
  HasVex(T),
  NoVex(T),
  HasEdge(T, T),
  NoEdge(T, T),
  HasEdgeWithCost(T, T, usize),
  NoEdgeWithCost(T, T, usize),
}

/// Getter Methods (Read-Only)
impl<T: Hash + Eq + Clone> Graph<T> {
  pub fn vertex_num(&self) -> usize {
    self.vertex_num
  }
  pub fn edge_num(&self) -> usize {
    self.edge_num
  }
  pub fn edges(&self) -> &HashMap<T, Edges<T>> {
    &self.edges_from
  }
}

/// Vex Operations
impl<T: Hash + Eq + Clone> Graph<T> {
  pub fn add_vex(&mut self, vex: &T) -> Result<(), ErrType<T>> {
    if self.edges_from.get(vex).is_some() {
      return Err(ErrType::HasVex(vex.to_owned()));
    }

    self.edges_from.insert(vex.clone(), Edges::new());
    self.edges_to.insert(vex.clone(), Edges::new());

    self.vertex_num += 1;
    Ok(())
  }
  pub fn del_vex(&mut self, vex: &T) -> Result<(), ErrType<T>> {
    if self.edges_from.get(vex).is_none() {
      return Err(ErrType::NoVex(vex.to_owned()));
    }
    let mut deleted_edge_num = 0;

    // get all ({any, _} -> vex),
    // remove (any -> {vex, _})'s {vex, _}
    deleted_edge_num += &self.edges_to[vex].len();
    for Edge { node: any, cost: _ } in &self.edges_to[vex] {
      while let Some(pos) = self.edges_from[any]
        .iter()
        .position(|edge| &edge.node == vex)
      {
        self.edges_from.get_mut(any).unwrap().remove(pos);
      }
    }
    // remove (vex -> {_, _})'s {_, _}
    deleted_edge_num += &self.edges_from[vex].len();
    self.edges_from.remove(vex);

    self.vertex_num -= 1;
    self.edge_num -= deleted_edge_num;
    Ok(())
  }
}

/// Edge Operations
impl<T: Hash + Eq + Clone> Graph<T> {
  pub fn add_edge(&mut self, start: &T, goal: &T, cost: usize) -> Result<(), ErrType<T>> {
    if self.edges_from.get(start).is_none() {
      return Err(ErrType::NoVex(start.to_owned()));
    }
    if self.edges_from.get(goal).is_none() {
      return Err(ErrType::NoVex(goal.to_owned()));
    }
    if self.edges_from[start]
      .iter()
      .any(|edge| &edge.node == goal && edge.cost == cost)
    {
      return Err(ErrType::HasEdgeWithCost(
        start.to_owned(),
        goal.to_owned(),
        cost,
      ));
    }

    // try to add (start -> {goal, cost})
    self
      .edges_from
      .get_mut(start)
      .unwrap()
      .push(Edge::new(goal.to_owned(), cost));
    // try to add ({start, cost} -> goal)
    self
      .edges_to
      .get_mut(goal)
      .unwrap()
      .push(Edge::new(start.to_owned(), cost));

    self.edge_num += 1;
    Ok(())
  }
  pub fn del_edge_with_cost(&mut self, start: &T, goal: &T, cost: usize) -> Result<(), ErrType<T>> {
    if self.edges_from.get(start).is_none() {
      return Err(ErrType::NoVex(start.to_owned()));
    }
    if self.edges_from.get(goal).is_none() {
      return Err(ErrType::NoVex(goal.to_owned()));
    }

    // try to delete (start -> {goal, cost})'s {goal, cost}
    if let Some(pos) = self.edges_from[start]
      .iter()
      .position(|edge| &edge.node == goal && edge.cost == cost)
    {
      self.edges_from.get_mut(start).unwrap().remove(pos);
    } else {
      return Err(ErrType::NoEdgeWithCost(
        start.to_owned(),
        goal.to_owned(),
        cost,
      ));
    }
    // try to delete ({start, cost} -> goal)'s {start, cost}
    if let Some(pos) = self.edges_to[goal]
      .iter()
      .position(|edge| &edge.node == start && edge.cost == cost)
    {
      self.edges_to.get_mut(goal).unwrap().remove(pos);
    } else {
      return Err(ErrType::NoEdgeWithCost(
        start.to_owned(),
        goal.to_owned(),
        cost,
      ));
    }

    self.edge_num -= 1;
    Ok(())
  }
  pub fn del_edge_all(&mut self, start: &T, goal: &T) -> Result<(), ErrType<T>> {
    if self.edges_from.get(start).is_none() {
      return Err(ErrType::NoVex(start.to_owned()));
    }
    if self.edges_from.get(goal).is_none() {
      return Err(ErrType::NoVex(goal.to_owned()));
    }
    let mut deleted_edge_num = 0;

    // try to delete (start -> {goal, _})'s {goal, _}
    while let Some(pos) = self.edges_from[start]
      .iter()
      .position(|edge| &edge.node == goal)
    {
      self.edges_from.get_mut(start).unwrap().remove(pos);
      deleted_edge_num += 1;
    }
    // try to delete ({start, _} -> goal)'s {start, _}
    while let Some(pos) = self.edges_to[goal]
      .iter()
      .position(|edge| &edge.node == start)
    {
      self.edges_to.get_mut(goal).unwrap().remove(pos);
    }

    if deleted_edge_num == 0 {
      return Err(ErrType::NoEdge(start.to_owned(), goal.to_owned()));
    }
    self.edge_num -= deleted_edge_num;
    Ok(())
  }
}

/// Builders
impl<T: Hash + Eq + Clone> Graph<T> {
  pub fn from(vertices: Vec<T>, edges: Vec<(T, T, usize)>) -> Self {
    let mut graph = Self::default();
    for vex in vertices.iter() {
      graph.add_vex(vex).unwrap_or_default();
    }
    for (start, goal, cost) in edges.iter() {
      graph.add_edge(start, goal, *cost).unwrap_or_default();
    }
    graph
  }
  pub fn clone_vertices_from(src: &Graph<T>) -> Self {
    let mut graph = Self::default();
    let vertices = src
      .edges_from
      .keys()
      .map(|v| v.to_owned())
      .collect::<Vec<T>>();
    vertices.iter().for_each(|v| {
      graph.add_vex(v).unwrap_or_default();
    });
    graph
  }
}

/// Traverse Operations
impl<T: Hash + Eq + Clone> Graph<T> {
  /// helper(recursive function) for dfs
  fn dfs_helper<'a, 'l: 'a, P>(
    &'l self,
    vex: &'a T,
    visited: &mut HashSet<&'a T>,
    predicate: &mut P,
  ) where
    P: FnMut(&T),
  {
    predicate(vex);
    visited.insert(vex);
    for Edge { node, cost: _ } in &self.edges_from[vex] {
      if !visited.contains(node) {
        self.dfs_helper(node, visited, predicate);
      }
    }
  }
  pub fn iterate_vertices_dfs<P>(&self, start: &T, predicate: &mut P)
  where
    P: FnMut(&T),
  {
    let mut visited: HashSet<&T> = HashSet::new();
    self.dfs_helper(start, &mut visited, predicate);
  }
  pub fn iterate_vertices_bfs<P>(&self, start: &T, predicate: &mut P)
  where
    P: FnMut(&T),
  {
    let mut visited: HashSet<&T> = HashSet::new();
    let mut queue: VecDeque<&T> = VecDeque::new();
    queue.push_back(start);
    visited.insert(start);
    while let Some(vex) = queue.pop_front() {
      predicate(vex);
      for Edge { node, cost: _ } in &self.edges_from[vex] {
        if !visited.contains(node) {
          queue.push_back(node);
          visited.insert(node);
        }
      }
    }
  }
}

/// Minium Spanning Tree Operations
impl<T: Hash + Eq + Clone> Graph<T> {
  fn get_mst(&self) -> Self {
    let _mst = Self::clone_vertices_from(self);
    unimplemented!();
  }
}

#[cfg(test)]
mod self_impl_graph {
  use super::*;

  #[test]
  fn default_constructor() {
    #[derive(Hash, Default, PartialEq, Debug, Eq, Clone)]
    struct SelfDefinedType {
      x: i32,
      y: i32,
    }
    // assert_eq!(SelfDefinedType::default(), SelfDefinedType { x: 0, y: 0 });
    let _graph = Graph::<String>::default();
    let _graph = Graph::<usize>::default();
    let _graph = Graph::<SelfDefinedType>::default();
  }

  #[test]
  fn test_add_vex() {
    let mut graph = Graph::<i32>::default();
    assert!(graph.add_vex(&3).is_ok());
    assert!(graph.add_vex(&3).is_err());
    assert!(graph.add_vex(&4).is_ok());
    assert!(graph.add_vex(&4).is_err());
  }

  #[test]
  fn test_add_edge() {
    let mut graph = Graph::<char>::default();
    let vertices = ['a', 'b', 'c', 'd'];
    let edges: Vec<(char, char, usize)> =
      vec![('a', 'b', 1), ('b', 'd', 2), ('c', 'a', 3), ('a', 'b', 2)];
    // 1. normally add vex and edge
    for vex in vertices.iter() {
      assert!(graph.add_vex(vex).is_ok());
    }
    for (start, goal, cost) in edges.iter() {
      assert!(graph.add_edge(start, goal, *cost).is_ok());
    }
    // 2. check size
    assert_eq!(graph.vertex_num(), 4);
    assert_eq!(graph.edge_num(), 4);
    // 3. if could handle `HasEdgeWithCost` error
    assert_eq!(
      graph.add_edge(&'a', &'b', 1).unwrap_err(),
      ErrType::HasEdgeWithCost('a', 'b', 1)
    );
    // 4. if could handle `NoVex` error
    assert_eq!(
      graph.add_edge(&'e', &'f', 1).unwrap_err(),
      ErrType::NoVex('e')
    );
    // 5. make sure error input hadn't affect the graph
    assert_eq!(graph.vertex_num(), 4);
    assert_eq!(graph.edge_num(), 4);
  }

  #[test]
  fn test_del_edge() {
    let mut graph = Graph::<char>::default();
    let vertices = ['a', 'b', 'c', 'd'];
    let edges: Vec<(char, char, usize)> = vec![
      ('a', 'b', 1),
      ('b', 'd', 2),
      ('c', 'a', 3),
      ('a', 'b', 2),
      ('a', 'b', 3),
      ('a', 'b', 4),
    ];
    for vex in vertices.iter() {
      graph.add_vex(vex).unwrap();
    }
    for (start, goal, cost) in edges.iter() {
      graph.add_edge(start, goal, *cost).unwrap();
    }
    assert_eq!(graph.vertex_num(), 4);
    assert_eq!(graph.edge_num(), 6);
    // 1. normally remove an edge
    assert!(graph.del_edge_with_cost(&'a', &'b', 3).is_ok());
    assert_eq!(graph.edge_num(), 5);
    // 2. remove an edge with non_existed vex
    assert!(graph.del_edge_with_cost(&'a', &'f', 3).is_err());
    assert_eq!(
      graph.del_edge_with_cost(&'a', &'f', 3).unwrap_err(),
      ErrType::NoVex('f')
    );
    assert_eq!(
      graph.del_edge_all(&'a', &'f').unwrap_err(),
      ErrType::NoVex('f')
    );
    // 3. remove an edge with non_exited cost only (all vertices are existed)
    assert_eq!(
      graph.del_edge_with_cost(&'a', &'b', 3).unwrap_err(),
      ErrType::NoEdgeWithCost('a', 'b', 3)
    );
    // 4. remove all edge between
    assert!(graph.del_edge_all(&'a', &'b').is_ok());
    assert_eq!(graph.edge_num(), 2);
    // 5. remove an edge with no way between two existed vertices
    assert_eq!(
      graph.del_edge_with_cost(&'a', &'b', 3).unwrap_err(),
      ErrType::NoEdgeWithCost('a', 'b', 3)
    );
    assert_eq!(
      graph.del_edge_all(&'a', &'b').unwrap_err(),
      ErrType::NoEdge('a', 'b')
    );
  }

  #[test]
  fn test_del_vex() {
    let mut graph = Graph::<char>::default();
    let vertices = ['a', 'b', 'c', 'd'];
    let edges: Vec<(char, char, usize)> = vec![
      ('a', 'b', 1),
      ('b', 'd', 2),
      ('c', 'b', 3),
      ('c', 'a', 3),
      ('a', 'b', 2),
      ('a', 'b', 3),
    ];
    for vex in vertices.iter() {
      graph.add_vex(vex).unwrap();
    }
    for (start, goal, cost) in edges.iter() {
      graph.add_edge(start, goal, *cost).unwrap();
    }
    assert_eq!(graph.vertex_num(), 4);
    assert_eq!(graph.edge_num(), 6);
    // 1. delete vex `a`
    graph.del_vex(&'a').unwrap();
    assert_eq!(graph.vertex_num(), 3);
    assert_eq!(graph.edge_num(), 2);
    // 2. try to delete non_existed vex `e`
    assert_eq!(graph.del_vex(&'e').unwrap_err(), ErrType::NoVex('e'));
    // 3. try to delete edge with non_existed vex `a`
    assert_eq!(
      graph.del_edge_all(&'c', &'a').unwrap_err(),
      ErrType::NoVex('a')
    );
    assert_eq!(
      graph.del_edge_all(&'a', &'c').unwrap_err(),
      ErrType::NoVex('a')
    );
  }

  #[test]
  fn test_dfs_iterate_vex() {
    let vertices = vec!['a', 'b', 'c', 'd'];
    let edges: Vec<(char, char, usize)> = vec![
      ('a', 'b', 1),
      ('b', 'd', 2),
      ('c', 'b', 3),
      ('c', 'a', 3),
      ('a', 'b', 2),
      ('a', 'c', 3),
    ];
    let graph = Graph::from(vertices, edges);
    let mut container = vec![];
    graph.iterate_vertices_dfs(&'a', &mut (|x| container.push(x.to_owned())));
    dbg!(container);
  }

  #[test]
  fn test_bfs_iterate_vex() {
    let vertices = vec!['a', 'b', 'c', 'd'];
    let edges: Vec<(char, char, usize)> = vec![
      ('a', 'b', 1),
      ('b', 'd', 2),
      ('c', 'b', 3),
      ('c', 'a', 3),
      ('a', 'b', 2),
      ('a', 'c', 3),
    ];
    let graph = Graph::from(vertices, edges);
    let mut container = vec![];
    graph.iterate_vertices_bfs(&'a', &mut (|x| container.push(x.to_owned())));
    dbg!(container);
  }
}
