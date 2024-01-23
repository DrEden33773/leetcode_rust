#![allow(dead_code)]

pub struct Solution;

impl Solution {
  pub fn common_factors(a: i32, b: i32) -> i32 {
    (1..=a.min(b)).fold(0, |acm, num| {
      if a % num == 0 && b % num == 0 {
        acm + 1
      } else {
        acm
      }
    })
  }

  pub fn better_common_factors(a: i32, b: i32) -> i32 {
    let c = |mut a: i32, mut b: i32| -> i32 {
      while b != 0 {
        let t = b;
        b = a % b;
        a = t;
      }
      a
    }(a, b);
    (1..=(c as f64).sqrt().ceil() as i32).fold(0, |acm, num| {
      if c % num == 0 {
        acm + if num * num != c { 2 } else { 1 }
      } else {
        acm
      }
    })
  }
}

#[cfg(test)]
mod common_factors {
  use super::*;

  #[test]
  fn it_works() {
    let c = Solution::better_common_factors(12, 6);
    eprintln!("{c}")
  }
}
