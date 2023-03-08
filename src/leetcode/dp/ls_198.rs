#![allow(dead_code, unused_imports)]

pub struct Solution;

use std::cmp::{max, min};

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => return 0,
            1 => return nums[0],
            _ => {}
        }
        let mut prev = nums[0];
        let mut curr = max(nums[0], nums[1]);
        for i in 2..nums.len() {
            let original = curr;
            curr = max(curr, prev + nums[i]);
            prev = original;
        }
        curr
    }
}
