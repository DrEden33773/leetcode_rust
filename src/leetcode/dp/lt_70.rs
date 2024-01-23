#![allow(dead_code)]

pub struct Solution;

impl Solution {
  pub fn climb_stairs(n: i32) -> i32 {
    let mut dp = vec![1; n as usize + 1];
    for i in 2..dp.len() {
      dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp.last().unwrap().to_owned()
  }
  pub fn climb_stairs_no_arr(n: i32) -> i32 {
    // _prev := f(0), curr := f(1), next := f(2)
    let (mut _prev, mut curr, mut next) = (1, 1, 2);
    for _ in 2..=n {
      _prev = curr;
      curr = next;
      next = _prev + curr;
    }
    curr
  }
}
