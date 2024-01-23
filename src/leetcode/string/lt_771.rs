crate::sln!();

impl Solution {
  pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    use std::collections::HashSet;
    let set = jewels.bytes().collect::<HashSet<_>>();
    stones
      .bytes()
      .fold(0, |ans, curr| ans + set.contains(&curr) as i32)
  }
}
