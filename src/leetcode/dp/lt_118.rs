#![allow(dead_code)]

pub struct Solution;

impl Solution {
  pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    if num_rows == 1 {
      return vec![vec![1]];
    }
    let mut res = vec![vec![1]];
    let mut prev = vec![1];
    for _ in 2..=num_rows {
      let mut curr = vec![1];
      for s in prev.windows(2) {
        curr.push(s[0] + s[1]);
      }
      curr.push(1);
      res.push(curr.clone());
      prev = curr;
    }
    res
  }
}

#[cfg(test)]
mod yang_hui_triangle_gen {
  #[test]
  fn test_window() {
    let v = vec![1, 3, 1];
    for i in v.windows(2) {
      eprintln!("{:?}", i);
    }
  }
}
