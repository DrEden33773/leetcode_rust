crate::sln!();

impl Solution {
  pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let mut ans = i32::MAX;

    let mut update = |curr: i32| {
      if (curr - target).abs() < (ans - target).abs() {
        ans = curr;
      }
    };

    for i in 0..n {
      if i > 0 && nums[i - 1] == nums[i] {
        continue;
      }
      let mut l = i + 1;
      let mut r = n - 1;
      while l < r {
        let sum = nums[i] + nums[l] + nums[r];
        if sum == target {
          return target;
        }
        update(sum);
        if sum > target {
          let mut r_next = r - 1;
          while l < r_next && nums[r_next] == nums[r] {
            r_next -= 1;
          }
          r = r_next;
        } else {
          let mut l_next = l + 1;
          while r > l_next && nums[l_next] == nums[l] {
            l_next += 1;
          }
          l = l_next;
        }
      }
    }

    ans
  }
}
