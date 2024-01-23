#![allow(dead_code)]

pub struct Solution;

use std::collections::HashMap;

impl Solution {
  pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    struct DisjointSet {
      pa: HashMap<i32, i32>,
    }
    impl DisjointSet {
      fn new(n: i32) -> Self {
        let pa: HashMap<i32, i32> = (0..n).fold(HashMap::new(), |mut pa, e| {
          pa.insert(e, e);
          pa
        });
        Self { pa }
      }
      fn find(&self, to_find: &i32) -> i32 {
        let parent = self.pa.get(to_find).unwrap();
        if parent == to_find {
          parent.to_owned()
        } else {
          self.find(parent)
        }
      }
      fn union(&mut self, from: &i32, to: &i32) {
        let root_of_from = self.find(from);
        let root_of_to = self.find(to);
        *self.pa.get_mut(&root_of_from).unwrap() = root_of_to;
      }
    }
    let mut disjoint_set = DisjointSet::new(n);
    for edge in edges {
      disjoint_set.union(&edge[0], &edge[1]);
    }
    disjoint_set.find(&source) == disjoint_set.find(&destination)
  }
}
