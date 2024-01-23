#![allow(dead_code)]

pub struct Solution;

impl Solution {
  /// ## `Brian Kernighan` Algorithm
  ///
  /// Here's a reference:
  ///
  /// ```
  /// let n: i32 = 0b1100;
  /// let mut cnt = 0;
  /// while n > 0 {
  ///     n &= n - 1;
  ///     cnt += 1;
  /// }
  /// assert_eq!(cnt, 2);
  /// ```
  ///
  /// `n &= n - 1;` => convert the rightest `1bit` to `0bit`
  pub fn brian_kernighan_count_bits(n: i32) -> Vec<i32> {
    let brian_kernighan = |mut n: i32| {
      let mut res = 0;
      while n > 0 {
        n &= n - 1;
        res += 1;
      }
      res
    };
    (0..=n).map(brian_kernighan).collect()
  }

  pub fn dp_highest_bit_count_bits(n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut bits = vec![0; n + 1];
    let mut high_bit = 0;
    for i in 1..=n {
      if i & (i - 1) == 0 {
        high_bit = i;
      }
      bits[i] = bits[i - high_bit] + 1;
    }
    bits
  }
}
