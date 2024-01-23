crate::Solution!();

use std::collections::HashMap;

impl Solution {
  pub fn can_permute_palindrome(s: String) -> bool {
    let map = s.bytes().fold(HashMap::new(), |mut map, c| {
      *map.entry(c).or_insert(0) += 1;
      map
    });
    let odd = map.values().fold(0, |mut odd, t| {
      odd += if t % 2 != 0 { 1 } else { 0 };
      odd
    });
    odd <= 1
  }
}
