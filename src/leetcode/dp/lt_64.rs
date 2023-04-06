#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = grid.clone();
        for row in 1..grid.len() {
            dp[row][0] += dp[row - 1][0]
        }
        for col in 1..grid[0].len() {
            dp[0][col] += dp[0][col - 1]
        }
        for row in 1..grid.len() {
            for col in 1..grid[0].len() {
                dp[row][col] = dp[row][col - 1].min(dp[row - 1][col]) + grid[row][col]
            }
        }
        dp.last().unwrap().last().unwrap().to_owned()
    }

    pub fn best_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; grid[0].len() + 1];
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                match (row, col) {
                    (0, 0) => dp[col + 1] = grid[row][col],
                    (0, _) => dp[col + 1] = grid[row][col] + dp[col],
                    (_, 0) => dp[col + 1] = grid[row][col] + dp[col + 1],
                    (_, _) => dp[col + 1] = grid[row][col] + dp[col].min(dp[col + 1]),
                }
            }
        }
        dp.last().unwrap().to_owned()
    }
}
