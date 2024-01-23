crate::sln!();

impl Solution {
  pub fn old_alternate_digit_sum(mut n: i32) -> i32 {
    let mut digits = vec![];
    let mut ans = 0;
    while n > 0 {
      digits.push(n % 10);
      n /= 10;
    }
    for (i, e) in digits.into_iter().rev().enumerate() {
      ans += if i % 2 == 0 { e } else { -e }
    }
    ans
  }
  pub fn alternate_digit_sum(mut n: i32) -> i32 {
    let mut sign = 1;
    let mut ans = 0;
    while n > 0 {
      ans += n % 10 * sign;
      sign = -sign;
      n /= 10;
    }
    ans * -sign
  }
}
