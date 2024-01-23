#![allow(dead_code)]

pub struct Solution;

impl Solution {
  pub fn tribonacci(n: i32) -> i32 {
    match n {
      0 => return 0,
      1 | 2 => return 1,
      _ => {}
    }
    let mut before_prev = 0;
    let mut prev = 1;
    let mut curr = 1;
    for _ in 3..=n {
      let next = before_prev + prev + curr;
      before_prev = prev;
      prev = curr;
      curr = next;
    }
    curr
  }
}
