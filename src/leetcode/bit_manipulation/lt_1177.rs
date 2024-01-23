crate::Solution!();

impl Solution {
  pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let cnt = (0..1)
      .chain(s.bytes().scan(0, |p, v| {
        *p ^= 1 << (v - 97);
        Some(*p)
      }))
      .collect::<Vec<i32>>();
    queries
      .into_iter()
      .map(|v| (v[0] as usize, v[1] as usize, v[2] as u32))
      .map(|(i, j, k)| (cnt[i] ^ cnt[j + 1]).count_ones() / 2 <= k)
      .collect()
  }
}
