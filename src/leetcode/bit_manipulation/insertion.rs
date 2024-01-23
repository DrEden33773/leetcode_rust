crate::sln!();

impl Solution {
  pub fn insert(n: u32, m: u32, i: u32, j: u32) -> u32 {
    let mut ans = n;
    (i..=j).for_each(|p| {
      ans &= !(1 << p);
    });
    ans | (m << i)
  }
  pub fn another_insert(n: u32, m: u32, i: u32, j: u32) -> u32 {
    let mut masks = 0u32;
    (0..=(j - i)).for_each(|p| masks |= 1 << p);
    masks <<= i;
    masks = !masks;
    let ans = n & masks;
    ans | m << i
  }
  pub fn insert_bits(n: i32, m: i32, i: i32, j: i32) -> i32 {
    Self::insert(n as u32, m as u32, i as u32, j as u32) as i32
  }
}
