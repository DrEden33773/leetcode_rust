crate::sln!();

impl Solution {
  fn add_strings(mut num1: String, mut num2: String) -> String {
    let mut ans = String::new();
    let mut carry = 0;
    if num1.len() < num2.len() {
      num1 = "0".repeat(num2.len() - num1.len()).to_string() + &num1;
    } else {
      num2 = "0".repeat(num1.len() - num2.len()).to_string() + &num2;
    }
    for (n1, n2) in num1.bytes().rev().zip(num2.bytes().rev()) {
      let sum = n1 - b'0' + n2 - b'0' + carry;
      carry = sum / 10;
      ans.push((sum % 10 + b'0') as char);
    }
    if carry != 0 {
      ans.push((carry + b'0') as char);
    }
    ans.chars().rev().collect()
  }

  fn mul_then_add(num1: String, num2: String) -> String {
    if num1 == "0" || num2 == "0" {
      return "0".to_string();
    }
    let mut ans = String::new();
    // `x` is a one-digit number in `num1`
    for (i, x) in num1.bytes().rev().enumerate() {
      let mut carry = 0;
      // (need to be reversed before executing `add_string()`)
      let mut curr = String::new();
      // suffix-0
      for _ in 0..i {
        curr.push('0');
      }
      // iterate over `num2` in reverse order
      for y in num2.bytes().rev() {
        let product = (x - b'0') * (y - b'0') + carry;
        carry = product / 10;
        curr.push((product % 10 + b'0') as char);
      }
      // deal with the last carry
      if carry != 0 {
        curr.push((carry + b'0') as char);
      }
      curr = curr.chars().rev().collect();
      ans = Self::add_strings(ans, curr);
    }
    ans
  }

  pub fn multiply(num1: String, num2: String) -> String {
    Self::mul_then_add(num1, num2)
  }
}
