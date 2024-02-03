crate::sln!();

use std::collections::HashMap;

impl Solution {
  pub fn roman_to_int(s: String) -> i32 {
    let s = s.as_bytes();
    let symbol_values = [
      (b'I', 1),
      (b'V', 5),
      (b'X', 10),
      (b'L', 50),
      (b'C', 100),
      (b'D', 500),
      (b'M', 1000),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();
    let mut ans = 0;
    for i in 0..s.len() {
      let value = symbol_values[&s[i]];
      if i + 1 < s.len() && value < symbol_values[&s[i + 1]] {
        ans -= value;
      } else {
        ans += value;
      }
    }
    ans
  }
}
