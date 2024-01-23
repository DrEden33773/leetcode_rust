#![allow(dead_code)]

pub struct Solution;

use std::ops::Sub;

impl Solution {
  #[inline(always)]
  fn generate_edges(points: &[Vec<i32>]) -> Vec<(usize, usize, i32)> {
    let mut res = vec![];
    for from in 0..points.len() {
      for to in (from + 1)..points.len() {
        let dist =
          points[from][0].sub(points[to][0]).abs() + points[from][1].sub(points[to][1]).abs();
        res.push((from, to, dist));
      }
    }
    res
  }
  pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    struct Dsu {
      pa: Vec<usize>,
      num_of_root: usize,
    }
    impl Dsu {
      #[inline(always)]
      fn new(n: usize) -> Self {
        Self {
          pa: (0..=n).collect(),
          num_of_root: n,
        }
      }
      #[inline(always)]
      fn get_root(&self, mut n: usize) -> usize {
        while self.pa[n] != n {
          n = self.pa[n]
        }
        n
      }
      #[inline(always)]
      fn union(&mut self, m: usize, n: usize) {
        // if self.find(m, n) {
        //     return;
        // }
        let m_root = self.get_root(m);
        let n_root = self.get_root(n);
        self.pa[m_root] = n_root;
        self.num_of_root -= 1;
      }
      #[inline(always)]
      fn find(&self, m: usize, n: usize) -> bool {
        self.get_root(m) == self.get_root(n)
      }
      #[inline(always)]
      fn num_of_root(&self) -> usize {
        self.num_of_root
      }
    }
    let mut edges = Solution::generate_edges(&points);
    edges.sort_unstable_by_key(|(_, _, weight)| *weight);
    let mut dsu = Dsu::new(points.len());
    let mut ans = 0;
    let mut picked = 0;
    for (from, to, weight) in edges {
      if picked == points.len() - 1 {
        break;
      }
      if dsu.find(from, to) {
        continue;
      }
      dsu.union(from, to);
      ans += weight;
      picked += 1;
    }
    ans
  }
}
