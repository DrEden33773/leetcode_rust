#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

struct Trie {
    children: HashMap<u8, Trie>,
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end: false,
        }
    }
    fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.bytes() {
            node = node.children.entry(c).or_insert(Trie::new());
        }
        node.is_end = true;
    }
}

pub struct Solution;

impl Solution {
    /// ## Dynamic Programming
    ///
    /// **Define:** `dp[i] := check(s[0..i])`
    ///
    /// **Therefore:** `dp[i] = dp[j] && check(s[j..i])`
    pub fn word_break(s: String, mut word_dict: Vec<String>) -> bool {
        word_dict.sort_unstable_by_key(|b| std::cmp::Reverse(b.len()));
        let dict = word_dict.into_iter().fold(HashSet::new(), |acm, word| {
            let mut acm = acm;
            acm.insert(word);
            acm
        });
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;
        for i in 1..dp.len() {
            for j in 0..i {
                let sub = String::from_utf8_lossy(&s.as_bytes()[j..i]).to_string();
                if dp[j] && dict.contains(&sub) {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp.last().unwrap().to_owned()
    }
    /// ## Dynamic Programming
    ///
    /// **Define:** `dp[i] := check(s[0..i])`
    ///
    /// **Therefore:** `dp[i] = dp[j] && check(s[j..i])`
    ///
    /// **Optimization:** `j in (0..i).rev()`, `branch-cutting` is possible
    pub fn optimized_word_break(s: String, mut word_dict: Vec<String>) -> bool {
        word_dict.sort_unstable_by_key(|b| std::cmp::Reverse(b.len()));
        let max_word_len = word_dict.get(0).unwrap().len();
        let dict = word_dict.into_iter().fold(HashSet::new(), |acm, word| {
            let mut acm = acm;
            acm.insert(word);
            acm
        });
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;
        for i in 1..dp.len() {
            for j in (0..i).rev() {
                if j - i > max_word_len {
                    break;
                }
                let sub = String::from_utf8_lossy(&s.as_bytes()[j..i]).to_string();
                if dp[j] && dict.contains(&sub) {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp.last().unwrap().to_owned()
    }
}
