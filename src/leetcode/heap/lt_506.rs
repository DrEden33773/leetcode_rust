#![allow(dead_code)]

pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
  pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    const FIRST: &str = "Gold Medal";
    const SECOND: &str = "Silver Medal";
    const THIRD: &str = "Bronze Medal";

    #[derive(Default, Clone, Copy)]
    struct Info {
      score: i32,
      index: i32,
    }
    impl PartialEq for Info {
      fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.index == other.index
      }
    }
    impl Eq for Info {}
    impl PartialOrd for Info {
      fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.score.partial_cmp(&other.score) {
          Some(core::cmp::Ordering::Equal) => {}
          ord => return ord,
        }
        self.index.partial_cmp(&other.index)
      }
    }
    impl Ord for Info {
      fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
      }
    }

    let mut res: Vec<String> = vec![String::new(); score.len()];
    let mut index = -1;
    let mut heap: BinaryHeap<Info> = score
      .iter()
      .map(|s| {
        index += 1;
        Info { score: *s, index }
      })
      .collect();
    let mut rank = 1;
    while let Some(Info { score: _, index }) = heap.pop() {
      let reward = match rank {
        1 => FIRST.to_owned(),
        2 => SECOND.to_owned(),
        3 => THIRD.to_owned(),
        _ => rank.to_string(),
      };
      res[index as usize] = reward;
      rank += 1;
    }
    res
  }
}
