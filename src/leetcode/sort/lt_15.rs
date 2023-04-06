#![allow(dead_code)]

pub struct Solution;

use std::collections::HashSet;

impl Solution {
    #[inline]
    fn get_pairs_in(s: &[i32], sum: i32) -> Vec<(i32, i32)> {
        use std::cmp::Ordering as ord;
        let mut res = vec![];
        let (mut l, mut r) = (0, s.len() - 1);
        while l < r {
            match (s[l] + s[r]).cmp(&sum) {
                ord::Less => l += 1,
                ord::Equal => {
                    res.push((s[l], s[r]));
                    l += 1;
                    r -= 1;
                }
                ord::Greater => r -= 1,
            }
        }
        res
    }
    /// ### Core
    ///
    /// Sort & TwoPointers & HashSet
    #[inline]
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut prev = i32::MIN;
        for i in 0..nums.len() {
            let curr = nums[i];
            if curr == prev {
                prev = curr;
                continue;
            }
            let pairs = Solution::get_pairs_in(&[&nums[..i], &nums[i + 1..]].concat(), -curr);
            for pair in pairs {
                ans.push(vec![curr, pair.0, pair.1])
            }
            prev = curr;
        }
        Solution::fuzzy_unique(ans)
    }
    #[inline]
    fn fuzzy_unique(vec: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut sorted: Vec<[i32; 3]> = Vec::new();
        let mut appeared: HashSet<[i32; 3]> = HashSet::new();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for arr in &vec {
            let mut arr = arr.to_owned();
            arr.sort_unstable();
            sorted.push([arr[0], arr[1], arr[2]]);
        }
        for i in 0..vec.len() {
            if !appeared.contains(&sorted[i]) {
                ans.push(vec[i].to_owned());
            }
            appeared.insert(sorted[i]);
        }
        ans
    }
    /// ### Best Implementation (Official)
    #[inline]
    pub fn official_three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut res: Vec<Vec<i32>> = vec![];
        for i in 0..n - 2 {
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }
            let mut left = i + 1;
            let mut right = n - 1;
            while left < right {
                if left > i + 1 && nums[left - 1] == nums[left] {
                    left += 1;
                    continue;
                }
                let sum = nums[i] + nums[left] + nums[right];
                if sum == 0 {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod three_sum {
    use super::*;

    #[test]
    fn it_works() {
        let res = Solution::three_sum([-1, 0, 1, 2, -1, -4].to_vec());
        drop(res)
    }
}
