crate::Solution!();

use std::collections::HashSet;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut appeared = HashSet::new();
        let mut diffs = HashSet::new();
        nums.iter().for_each(|&num| {
            [num - k, num + k].iter().for_each(|tar| {
                if appeared.contains(tar) {
                    diffs.insert((num.min(*tar), num.max(*tar)));
                }
            });
            appeared.insert(num);
        });
        diffs.len() as i32
    }
}
