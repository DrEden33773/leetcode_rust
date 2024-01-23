use std::collections::HashMap;

use rand::Rng;

#[allow(dead_code)]
pub struct Solution;

impl Solution {
  #[allow(dead_code)]
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut table = HashMap::new();
    for (curr_index, &num) in nums.iter().enumerate() {
      let another = target - num;
      if let Some(another_index) = table.get(&another) {
        return vec![curr_index as i32, *another_index as i32];
      }
      table.insert(num, curr_index);
    }
    vec![]
  }

  #[allow(dead_code)]
  pub fn test() {
    const LEN: usize = 10_000;
    const LIM: i32 = 1_000_000;
    let nums: Vec<i32> = (0..LEN)
      .map(|_| rand::thread_rng().gen_range(-LIM..=LIM))
      .collect();
    let (target, _, _) = {
      let idx_1 = rand::thread_rng().gen_range(0..LEN);
      let idx_2 = rand::thread_rng().gen_range(0..LEN);
      (nums[idx_1] + nums[idx_2], idx_1, idx_2)
    };
    let vec = Self::two_sum(nums.clone(), target);
    let res = nums[vec[0] as usize] + nums[vec[1] as usize];
    match res == target {
      true => println!("Correct!"),
      false => println!("Error!"),
    }
    println!();
  }
}
