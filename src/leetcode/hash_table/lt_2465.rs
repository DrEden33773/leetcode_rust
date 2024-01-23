#![allow(dead_code)]

pub struct Solution;

use std::collections::HashSet;

impl Solution {
  pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    nums.sort_unstable();
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
      set.insert(nums[left] + nums[right]);
      left += 1;
      right -= 1;
    }
    set.len() as i32
  }
}
