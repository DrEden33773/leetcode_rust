#![allow(dead_code)]

pub struct Solution;

use std::ops::AddAssign;

impl Solution {
  pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let mut cnt = grid
      .iter()
      .fold(std::collections::HashMap::new(), |mut cnt, row| {
        cnt.entry(row.to_owned()).or_insert(0).add_assign(1);
        cnt
      });
    (0..grid.len()).fold(0, |ans, col| {
      let curr_col: Vec<i32> = (0..grid.len())
        .map(|row| grid[row][col])
        .collect::<Vec<_>>();
      ans + *cnt.entry(curr_col).or_insert(0)
    })
  }
}
