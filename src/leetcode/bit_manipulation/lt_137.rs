crate::sln!();

impl Solution {
  #[inline]
  fn bitwise_simulation_generic(nums: Vec<i32>, times: i32) -> i32 {
    let mut ans = 0;
    for i in 0..i32::BITS {
      let mut curr_bit_sum = 0;
      for &num in &nums {
        curr_bit_sum += (num >> i) & 1;
      }
      ans |= (curr_bit_sum % times) << i;
    }
    ans
  }

  #[inline]
  fn bitwise_simulation(nums: Vec<i32>) -> i32 {
    Solution::bitwise_simulation_generic(nums, 3)
  }

  #[inline]
  fn digital_circuit(nums: Vec<i32>) -> i32 {
    let (mut a, mut b) = (0, 0);
    for num in nums {
      b = !a & (b ^ num);
      a = !b & (a ^ num);
    }
    b
  }

  #[inline]
  pub fn single_number(nums: Vec<i32>) -> i32 {
    Solution::bitwise_simulation(nums)
    // Solution::digital_circuit(nums)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn case_1() {
    let nums = vec![30000, 500, 100, 30000, 100, 30000, 100];
    assert_eq!(Solution::single_number(nums), 500);
  }
}
