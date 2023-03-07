#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut iter = t.chars();
        let matched = s
            .chars()
            .filter(|x| {
                while let Some(y) = iter.next() {
                    if x == &y {
                        return true;
                    }
                }
                false
            })
            .collect::<String>();
        matched.len() == s.len()
    }
}
