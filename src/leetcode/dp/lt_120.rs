#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; triangle.last().unwrap().len()];
        let get_min = |s: &[i32]| {
            let mut min = i32::MAX;
            for n in s {
                if *n < min {
                    min = *n;
                }
            }
            min
        };
        dp[0] = triangle[0][0];
        for row in 1..triangle.len() {
            // reversed iteration, otherwise, affected by previous sum
            dp[row] = dp[row - 1] + triangle[row][row];
            for col in (1..row).rev() {
                dp[col] = dp[col].min(dp[col - 1]) + triangle[row][col];
            }
            dp[0] = dp[0] + triangle[row][0]
        }
        get_min(&dp)
    }
}
