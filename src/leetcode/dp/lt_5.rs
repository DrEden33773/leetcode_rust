#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let len = s.len();
        let mut dp = vec![vec![false; len]; len];
        for i in 0..len {
            dp[i][i] = true;
        }
        for l in 0..len - 1 {
            let r = l + 1;
            dp[l][r] = s[l] == s[r];
        }
        for gap in 2..len {
            for r in gap..len {
                let l = r - gap;
                dp[l][r] = dp[l + 1][r - 1] && s[l] == s[r];
            }
        }
        let (mut left, mut right) = (0, 0);
        for l in 0..len {
            for r in l..len {
                if r - l > right - left && dp[l][r] {
                    right = r;
                    left = l;
                }
            }
        }
        String::from_utf8_lossy(&s[left..=right]).into()
    }

    pub fn unofficial_center_expansion(s: String) -> String {
        let s = s.as_bytes();
        let (mut start, mut size) = (0, 0);
        for i in 0..(2 * s.len() - 1) {
            let mut l = i >> 1;
            let mut r = l + (i & 1);
            while l < s.len() && r < s.len() && s[l] == s[r] {
                l -= 1;
                r += 1;
            }
            if r - l > size {
                size = r - l;
                start = l;
            }
        }
        String::from_utf8_lossy(&s[start + 1..start + size]).into()
    }

    pub fn official_center_expansion(s: String) -> String {
        let s = s.as_bytes();
        let (mut end, mut start) = (0, 0);
        let expand = |mut l: isize, mut r: usize| {
            while l >= 0 && r < s.len() && s[l as usize] == s[r] {
                l -= 1;
                r += 1;
            }
            ((l + 1) as usize, r - 1)
        };
        for i in 0..s.len() {
            let (l1, r1) = expand(i as isize, i);
            let (l2, r2) = expand(i as isize, i + 1);
            if r1 >= l1 && r1 - l1 > end - start {
                start = l1;
                end = r1;
            }
            if r2 >= l2 && r2 - l2 > end - start {
                start = l2;
                end = r2;
            }
        }
        String::from_utf8_lossy(&s[start..=end]).into()
    }
}

#[cfg(test)]
mod longest_palindrome {
    use super::*;

    #[test]
    fn test_official_center_expansion() {
        let input = "babas".to_string();
        let _output = Solution::official_center_expansion(input);
    }
}
