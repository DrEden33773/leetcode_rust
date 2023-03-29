#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (m, n) = (text1.len(), text2.len());
        let (mut prev, mut curr) = (vec![0; n + 1], vec![0; n + 1]);
        for i in 1..=m {
            for j in 1..=n {
                curr[j] = if text1.chars().nth(i - 1) == text2.chars().nth(j - 1) {
                    prev[j - 1] + 1
                } else {
                    std::cmp::max(prev[j], curr[j - 1])
                }
            }
            (prev, curr) = (curr, prev)
        }
        prev[n]
    }
}
