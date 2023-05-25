#![allow(dead_code)]

pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut appeared = HashSet::new();
        let unique = nums
            .iter()
            .filter(|&num| {
                let if_unique = !appeared.contains(num);
                appeared.insert(*num);
                if_unique
            })
            .map(|&num| num)
            .collect::<Vec<_>>();
        let len = unique.len();
        nums.clear();
        nums.reserve(len);
        unique.into_iter().for_each(|num| nums.push(num));
        len as i32
    }
}
