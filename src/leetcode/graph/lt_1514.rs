#![allow(dead_code)]

pub struct Solution;

use std::collections::BinaryHeap;

#[derive(Clone, Copy, Default)]
struct Edge {
  dst: usize,
  prob: f64,
}

#[derive(Clone, Copy, Default, PartialEq)]
struct Accumulation {
  dst: usize,
  prob: f64,
}

impl Ord for Accumulation {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.prob.partial_cmp(&other.prob).unwrap()
  }
}

impl Eq for Accumulation {}

impl PartialOrd for Accumulation {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Solution {
  fn heap_dij(adj_list: Vec<Vec<Edge>>, start: usize, goal: usize) -> f64 {
    let mut prob = vec![0.0; adj_list.len()];
    let mut heap = BinaryHeap::new();

    prob[start] = 1.0;
    heap.push(Accumulation {
      dst: start,
      prob: 1.0,
    });

    while let Some(Accumulation {
      dst: picked,
      prob: start_to_picked,
    }) = heap.pop()
    {
      if picked == goal {
        return start_to_picked;
      }

      if start_to_picked < prob[picked] {
        continue;
      }

      for &Edge {
        dst: next,
        prob: picked_to_next,
      } in adj_list[picked].iter()
      {
        let start_to_next = start_to_picked * picked_to_next;
        if start_to_next > prob[next] {
          prob[next] = start_to_next;
          heap.push(Accumulation {
            dst: next,
            prob: start_to_next,
          });
        }
      }
    }

    0.0
  }

  pub fn max_probability(
    n: i32,
    edges: Vec<Vec<i32>>,
    succ_prob: Vec<f64>,
    start: i32,
    end: i32,
  ) -> f64 {
    let n = n as usize;
    let start = start as usize;
    let goal = end as usize;

    let mut adj_list = vec![vec![]; n];
    for (pair, prob) in edges.into_iter().zip(succ_prob) {
      let (start, end) = (pair[0] as usize, pair[1] as usize);

      // according to `test`, edges are `double-arrowed`
      adj_list[start].push(Edge { dst: end, prob });
      adj_list[end].push(Edge { dst: start, prob });
    }

    Solution::heap_dij(adj_list, start, goal)
  }
}

#[cfg(test)]
mod test_solution {
  use super::*;

  #[test]
  fn test() {
    let edges = vec![
      vec![1, 4],
      vec![2, 4],
      vec![0, 4],
      vec![0, 3],
      vec![0, 2],
      vec![2, 3],
    ];
    let succ_prob = vec![0.37, 0.17, 0.93, 0.23, 0.39, 0.04];
    let n = 5;
    let start = 3;
    let end = 4;

    assert_eq!(
      Solution::max_probability(n, edges, succ_prob, start, end),
      0.2139
    );
  }
}
