#![allow(dead_code)]

use std::cmp::min;

pub struct Solution;

impl Solution {
  pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut prev = 0;
    let mut curr = 0;
    for i in 2..cost.len() {
      let next = min(curr + cost[i - 1], prev + cost[i - 2]);
      prev = curr;
      curr = next;
    }
    curr
  }
  pub fn old_method(cost: Vec<i32>) -> i32 {
    let mut dp = vec![0; cost.len() + 1];
    dp[0] = 0;
    dp[1] = 0;
    for i in 2..=cost.len() {
      dp[i] = min(dp[i - 1] + cost[i - 1], dp[i - 2] + cost[i - 2])
    }
    dp.last().unwrap().to_owned()
  }
}

#[cfg(test)]
mod min_cost_climbing_stairs {
  use super::*;

  #[test]
  fn two_cases() {
    let case1 = vec![10, 15, 20];
    let case2 = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    let res1 = Solution::min_cost_climbing_stairs(case1.clone());
    let res2 = Solution::min_cost_climbing_stairs(case2.clone());
    eprintln!("{:?} => {res1}", case1);
    eprintln!("{:?} => {res2}", case2);
  }
}
