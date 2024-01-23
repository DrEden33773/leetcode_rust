#![allow(dead_code)]

pub struct Solution;

use std::collections::HashSet;

impl Solution {
  pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
    let s = s.as_bytes();
    let dist = distance
      .into_iter()
      .map(|n| n as usize)
      .collect::<Vec<usize>>();
    let hash = |c: u8| (c - b'a') as usize;
    let mut appeared = HashSet::<usize>::new();
    for curr in 0..s.len() {
      if appeared.contains(&curr) {
        continue;
      }
      let next = curr + dist[hash(s[curr])] + 1;
      if next >= s.len() || s[curr] != s[next] {
        return false;
      }
      appeared.insert(next);
    }
    true
  }
}

#[cfg(test)]
mod check_distances {
  use super::*;

  #[test]
  fn it_works() {
    let s = "abaccb".to_string();
    let distance = vec![
      1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let res = Solution::check_distances(s, distance);
    eprintln!("{res}")
  }
}
