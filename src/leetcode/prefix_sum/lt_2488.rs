#![allow(dead_code, unused_mut)]

pub struct Solution;

use std::collections::HashMap;

impl Solution {
  #[inline]
  fn find<T: Ord>(nums: &Vec<T>, target: &T) -> Option<usize> {
    (0..nums.len()).find(|&i| &nums[i] == target)
  }
  #[inline]
  fn sign<T: Ord>(num: &T, cmp: &T) -> i32 {
    match num.cmp(cmp) {
      std::cmp::Ordering::Less => -1,
      std::cmp::Ordering::Equal => 0,
      std::cmp::Ordering::Greater => 1,
    }
  }
  pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    let k_index = Solution::find(&nums, &k).unwrap();
    let mut counts: HashMap<i32, i32> = HashMap::new();
    counts.insert(0, 1);
    let mut sum = 0;
    (0..nums.len()).for_each(|i| {
      sum += Solution::sign(&nums[i], &k);
      if i < k_index {
        *counts.entry(sum).or_insert(0) += 1;
      } else {
        let prev0 = counts.get(&sum).unwrap_or(&0).to_owned();
        let prev1 = counts.get(&(sum - 1)).unwrap_or(&0).to_owned();
        ans += prev0 + prev1;
      }
    });
    ans
  }
}

#[cfg(test)]
mod count_subarrays {
  use super::*;
  fn case_1() {
    let res = Solution::count_subarrays(vec![3, 2, 1, 4, 5], 4);
    assert_eq!(res, 3);
  }
  fn case_2() {
    let res = Solution::count_subarrays(vec![2, 3, 1], 3);
    assert_eq!(res, 1);
  }
  fn case_3() {
    let res = Solution::count_subarrays(vec![6, 5, 3, 4, 1, 2], 1);
    assert_eq!(res, /* 0 */ 3);
  }
  #[test]
  fn it_works() {
    case_1();
    case_2();
    case_3();
  }
}
