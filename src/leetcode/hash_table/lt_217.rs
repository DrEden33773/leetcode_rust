#![allow(dead_code)]

pub struct Solution;

use std::collections::HashSet;

impl Solution {
  pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::with_capacity(nums.len());
    for num in nums {
      if set.get(&num).is_some() {
        return true;
      }
      set.insert(num);
    }
    false
  }
}
