#![allow(dead_code)]

pub struct Solution;

impl Solution {
  pub fn add_strings(num1: String, num2: String) -> String {
    let (num1, num2) = (num1.as_bytes(), num2.as_bytes());
    let mut ans = Vec::<u8>::new();
    let mut l = num1.len() as i32 - 1;
    let mut r = num2.len() as i32 - 1;
    let mut carry = 0;
    while l >= 0 || r >= 0 || carry != 0 {
      let x = if l >= 0 { num1[l as usize] - b'0' } else { 0 };
      let y = if r >= 0 { num2[r as usize] - b'0' } else { 0 };
      let res = x + y + carry;
      ans.push(b'0' + res % 10);
      carry = res / 10;
      l -= 1;
      r -= 1;
    }
    ans.reverse();
    String::from_utf8(ans).unwrap()
  }
}
