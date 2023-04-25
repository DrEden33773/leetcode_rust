#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut zipped = names
            .into_iter()
            .zip(heights.into_iter())
            .collect::<Vec<_>>();
        zipped.sort_unstable_by(|(_, height_1), (_, height_2)| height_2.cmp(height_1));
        zipped.into_iter().map(|(name, _)| name).collect()
    }
}
