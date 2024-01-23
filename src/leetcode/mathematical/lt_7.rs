crate::sln!();

impl Solution {
  pub fn reverse(x: i32) -> i32 {
    let mut rev = 0;
    let mut x = x;
    while x != 0 {
      if !(i32::MIN / 10..=i32::MAX / 10).contains(&rev) {
        return 0;
      }
      let digit = x % 10;
      x /= 10;
      rev = rev * 10 + digit;
    }
    rev
  }
}
