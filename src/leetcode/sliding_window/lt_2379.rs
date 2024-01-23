#![allow(dead_code)]

pub struct Solution;

impl Solution {
  pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
    // 0. reform inputs
    let blocks: Vec<char> = blocks.chars().collect();
    let k = k as usize;

    // 1. calculate value of first window
    let mut min = 0;
    (0..k).for_each(|i| {
      let to_add = if blocks[i] == 'W' { 1 } else { 0 };
      min += to_add;
    });

    // 2. start to update the window
    let mut new = min;
    for i in k..blocks.len() {
      let to_add = if blocks[i] == 'W' { 1 } else { 0 };
      let to_sub = if blocks[i - k] == 'W' { 1 } else { 0 };
      new += to_add - to_sub;
      min = if new < min { new } else { min };
    }

    // 3. return
    min
  }
}
