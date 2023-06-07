#![allow(dead_code)]

pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    /// ## Greedy Approach
    ///
    /// 1. Assume that `mouse2` eats all the cheese
    ///
    /// 2. Then, pick `max(reward1[i] - reward[i])` each time
    ///
    /// 3. Repeat `2.` until meet the requirements of `k`
    ///
    /// This method use `Heap`
    pub fn mice_and_cheese_using_heap(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let mut unpicked = reward1
            .iter()
            .zip(reward2.iter())
            .map(|(r1, r2)| r1 - r2)
            .collect::<BinaryHeap<_>>();
        let mut ans = reward2.iter().sum::<i32>();
        (0..k).for_each(|_| ans += unpicked.pop().unwrap());
        ans
    }

    /// ## Greedy Approach
    ///
    /// This method using `SortedVec` instead of `Heap`
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let mut unpicked = reward1
            .iter()
            .zip(reward2.iter())
            .map(|(r1, r2)| r1 - r2)
            .collect::<Vec<_>>();
        unpicked.sort_unstable_by(|a, b| b.cmp(a));
        reward2.iter().sum::<i32>() + unpicked[0..k as usize].iter().sum::<i32>()
    }
}
