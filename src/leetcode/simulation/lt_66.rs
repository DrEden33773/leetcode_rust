crate::sln!();

impl Solution {
  pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    for digit in digits.iter_mut().rev() {
      if *digit == 9 {
        *digit = 0;
      } else {
        *digit += 1;
        return digits;
      }
    }
    let mut ans = vec![0; digits.len() + 1];
    ans[0] = 1;
    ans
  }
}
