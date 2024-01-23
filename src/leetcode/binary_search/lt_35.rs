#![allow(dead_code)]

pub struct Solution;

impl Solution {
  pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len();
    while l < r {
      let m = (l + r) / 2;
      match target.cmp(&nums[m]) {
        std::cmp::Ordering::Greater => l = m + 1,
        std::cmp::Ordering::Less => r = m,
        std::cmp::Ordering::Equal => return m as i32,
      }
    }
    l as i32
  }
}
