crate::sln!();

impl Solution {
  /// 1. `a ^ b ^ c = a ^ c ^ b`
  /// 2. `a ^ a = 0`
  /// 3. `a ^ 0 = a`
  pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    for &num in &nums[1..] {
      res ^= num;
    }
    res
  }
}
