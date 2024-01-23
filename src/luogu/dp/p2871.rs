#![allow(dead_code)]

use std::io;

#[inline]
fn backpack_01_solution(weights: &[usize], values: &[usize], capacity: usize) -> usize {
  let num = weights.len();
  let mut dp = vec![0; capacity + 1];
  // i => item_upper_lim
  for i in 1..=num {
    let (weight, value) = (weights[i - 1], values[i - 1]);
    // j => sub_capacity
    for j in (weight..=capacity).rev() {
      dp[j] = dp[j].max(dp[j - weight] + value)
    }
  }
  dp[capacity]
}

#[inline]
pub fn exec_interface() {
  /* buffer */
  let mut buf: Vec<&str>;
  /* var */
  let mut weights = Vec::<usize>::new();
  let mut values = Vec::<usize>::new();
  /* read proc */
  let mut str = String::new();
  io::stdin().read_line(&mut str).unwrap();
  buf = str.trim().split(' ').collect();
  let num = buf[0].parse().unwrap();
  let capacity = buf[1].parse().unwrap();
  for _ in 0..num {
    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();
    buf = str.trim().split(' ').collect();
    weights.push(buf[0].parse().unwrap());
    values.push(buf[1].parse().unwrap());
  }
  /* res */
  let res = backpack_01_solution(&weights, &values, capacity);
  println!("{res}");
}

#[cfg(test)]
mod the_p2871 {
  use super::*;

  #[test]
  fn it_works() {
    let weights = vec![1, 2, 3, 2];
    let values = vec![4, 6, 12, 7];
    let capacity = 6;
    let res = backpack_01_solution(&weights, &values, capacity);
    eprintln!("{res}")
  }
}
