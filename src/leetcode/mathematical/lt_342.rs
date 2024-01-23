#[allow(dead_code)]
pub struct Solution;

impl Solution {
  /// 4^x should always be 2^(2x) => judge `is_power_of_two` first
  ///
  /// 4^x == (3 + 1)^x, (3 + 1)^x == 3k + 1 => 4^x mod 3 == 1
  ///
  /// 2^(2x) mod 3 == 1
  ///
  /// 2^(2x + 1) mod 3 == (4^x * 2) mod 3 == 4^x mod 3 * 2 == 2
  ///
  /// Now, we could solve this issue.
  #[allow(dead_code)]
  pub fn is_power_of_four(n: i32) -> bool {
    let if_gt_zero = n > 0;
    let is_power_of_two = n & (n - 1) == 0 && if_gt_zero;
    is_power_of_two && n % 3 == 1
  }
}
