#![allow(dead_code)]

pub struct Solution;

impl Solution {
  pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
    let (min, max) = (
      nums.iter().min().unwrap().to_owned(),
      nums.iter().max().unwrap().to_owned(),
    );
    let diff = max - min;
    let mut ans = 1;
    for d in -diff..=diff {
      let mut f = vec![-1; max as usize + 1];
      for num in &nums {
        let num = *num;
        let prev = num - d;
        if prev >= min && prev <= max && f[prev as usize] != -1 {
          f[num as usize] = f[num as usize].max(f[prev as usize] + 1);
          ans = ans.max(f[num as usize]);
        }
        f[num as usize] = f[num as usize].max(1);
      }
    }
    ans
  }
}
