#[allow(dead_code)]
pub struct Solution;

impl Solution {
  /// _3_POWER_19 = 1162261467
  ///
  /// this is the biggest three-powered-number in i32 range
  ///
  /// any other three-powered-number `n` should satisfy constraint:
  ///
  /// ```
  /// 1162261467 % n == 0
  /// ```
  #[allow(dead_code)]
  pub fn is_power_of_three(n: i32) -> bool {
    const _3_POWER_19: i32 = 1162261467;
    n > 0 && _3_POWER_19 % n == 0
  }

  #[allow(dead_code)]
  pub fn another_is_power_of_three(mut n: i32) -> bool {
    while n > 0 && n % 3 == 0 {
      n /= 3;
    }
    n == 1
  }
}
