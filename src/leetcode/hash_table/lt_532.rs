crate::Solution!();

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashSet;
        let mut visited = HashSet::new();
        let mut diffs = HashSet::new();
        nums.into_iter().for_each(|num| {
            [num - k, num + k].into_iter().for_each(|tar| {
                if visited.contains(&tar) {
                    diffs.insert(num.min(tar));
                }
            });
            visited.insert(num);
        });
        diffs.len() as i32
    }
}
