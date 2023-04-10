#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![1; n];
        for _i in 1..m {
            for j in 1..n {
                dp[j] += dp[j - 1];
            }
        }
        dp.last().unwrap().to_owned()
    }
}
