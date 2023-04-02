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
    pub fn max_value_new(mut grid: Vec<Vec<i32>>) -> i32 {
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
    pub fn max_value_1d_dp(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; grid[0].len() + 1];
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                dp[col + 1] = grid[row][col] + max(dp[col], dp[col + 1]);
            }
        }
        *dp.last().unwrap()
    }
}
