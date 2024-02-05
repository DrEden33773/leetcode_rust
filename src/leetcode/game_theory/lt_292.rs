crate::sln!();

impl Solution {
  #[inline]
  pub const fn can_win_nim(n: i32) -> bool {
    Solution::generic_can_win_nim::<3>(n)
  }

  #[inline]
  pub const fn generic_can_win_nim<const UPPER_LIMIT: i32>(n: i32) -> bool {
    n % (UPPER_LIMIT + 1) != 0
  }
}
