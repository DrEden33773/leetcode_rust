crate::sln!();

impl Solution {
  pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut five = 0;
    let mut ten = 0;
    for b in bills {
      if b == 5 {
        five += 1
      } else if b == 10 {
        five -= 1;
        ten += 1
      } else if ten != 0 {
        five -= 1;
        ten -= 1
      } else {
        five -= 3
      }
      if five < 0 {
        return false;
      }
    }
    true
  }
}
