#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let is_vowel = |c: char| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u');
        let mut prefix_sum = vec![0; words.len()];
        for (i, e) in words.iter().enumerate() {
            let if_suit =
                is_vowel(e.chars().nth(0).unwrap()) && is_vowel(e.chars().nth_back(0).unwrap());
            if i == 0 {
                prefix_sum[0] = if_suit as i32
            } else {
                prefix_sum[i] = prefix_sum[i - 1] + if_suit as i32
            }
        }
        queries
            .into_iter()
            .map(|v| (v[0] as usize, v[1] as usize))
            .map(|(from, to)| {
                if from == 0 {
                    prefix_sum[to]
                } else {
                    prefix_sum[to] - prefix_sum[from - 1]
                }
            })
            .collect::<Vec<_>>()
    }
}
