crate::Solution!();

use std::collections::HashMap;

impl Solution {
  pub fn longest_palindrome(s: String) -> i32 {
    let map = s.bytes().fold(HashMap::new(), |mut map, curr| {
      *map.entry(curr).or_insert(0) += 1;
      map
    });
    let mut ans = 0;
    map.values().for_each(|&t| {
      ans += t / 2 * 2;
      ans += if t % 2 == 1 && ans % 2 == 0 { 1 } else { 0 };
    });
    ans
  }
}
