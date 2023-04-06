#![allow(dead_code)]

pub struct Solution;

use std::cmp::max;

impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        let row = grid.len();
        let col = grid.first().unwrap().len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; col]; row];
        dp[0][0] = grid[0][0];
        for r in 1..row {
            dp[r][0] = dp[r - 1][0] + grid[r][0];
        }
        for c in 1..col {
            dp[0][c] = dp[0][c - 1] + grid[0][c];
        }
        for r in 1..row {
            for c in 1..col {
                dp[r][c] = max(dp[r - 1][c], dp[r][c - 1]) + grid[r][c];
            }
        }
        dp[row - 1][col - 1]
    }
    pub fn max_value_cheat(mut grid: Vec<Vec<i32>>) -> i32 {
        let row = grid.len();
        let col = grid.first().unwrap().len();
        for r in 1..row {
            grid[r][0] += grid[r - 1][0];
        }
        for c in 1..col {
            grid[0][c] += grid[0][c - 1];
        }
        for r in 1..row {
            for c in 1..col {
                grid[r][c] += max(grid[r - 1][c], grid[r][c - 1]);
            }
        }
        grid[row - 1][col - 1]
    }
    pub fn max_value_one_1d_dp(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; grid[0].len() + 1];
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                dp[col + 1] = grid[row][col] + max(dp[col], dp[col + 1]);
            }
        }
        *dp.last().unwrap()
    }
    /// **Recommended Method:**
    ///
    ///  `1d_dp` * `2`
    pub fn max_value_two_1d_dp(grid: Vec<Vec<i32>>) -> i32 {
        let mut prev = grid[0].to_owned();
        let mut curr = vec![0; grid[0].len()];
        // deal with the initial status of `prev`
        for col in 1..prev.len() {
            prev[col] += prev[col - 1];
        }
        for row in 1..grid.len() {
            // deal with the case where `col := 0`
            curr[0] = prev[0] + grid[row][0];
            // deal with `col in 1..max_col`
            for col in 1..grid[0].len() {
                curr[col] = grid[row][col] + curr[col - 1].max(prev[col]);
            }
            prev = curr;
            curr = vec![0; grid[0].len()];
        }
        prev.last().unwrap().to_owned()
    }
}
