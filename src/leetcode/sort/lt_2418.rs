#![allow(dead_code)]

pub struct Solution;

impl Solution {
  pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut zipped = names.into_iter().zip(heights).collect::<Vec<_>>();
    zipped.sort_by(|(_, h1), (_, h2)| h2.cmp(h1));
    zipped.into_iter().map(|(name, _)| name).collect()
  }
}
