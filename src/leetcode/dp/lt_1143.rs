#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn better_lcs(text1: String, text2: String) -> i32 {
        let (m, n) = (text1.len(), text2.len());
        let (mut prev, mut curr) = (vec![0; n + 1], vec![0; n + 1]);
        for i in 1..=m {
            for j in 1..=n {
                curr[j] = if text1.as_bytes()[i - 1] == text2.as_bytes()[j - 1] {
                    prev[j - 1] + 1
                } else {
                    std::cmp::max(prev[j], curr[j - 1])
                }
            }
            (prev, curr) = (curr, prev)
        }
        prev[n]
    }

    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (m, n) = (text1.len(), text2.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = if text1.as_bytes()[i - 1] == text2.as_bytes()[j - 1] {
                    dp[i - 1][j - 1] + 1
                } else {
                    std::cmp::max(dp[i - 1][j], dp[i][j - 1])
                }
            }
        }
        dp[m][n]
    }
}
