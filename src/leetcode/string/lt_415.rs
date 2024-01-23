crate::sln!();

impl Solution {
  pub fn add_strings(mut num1: String, mut num2: String) -> String {
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
}
