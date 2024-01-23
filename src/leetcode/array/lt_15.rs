crate::sln!();

impl Solution {
  pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut ans = vec![];
    let n = nums.len();
    for first in 0..(n - 2) {
      if first > 0 && nums[first - 1] == nums[first] {
        continue;
      }
      if nums[first] + nums[first + 1] + nums[first + 2] > 0 {
        break;
      }
      if nums[first] + nums[n - 1] + nums[n - 2] < 0 {
        continue;
      }
      let mut third = n - 1;
      let target = -nums[first];
      for second in (first + 1)..(n - 1) {
        if second > first + 1 && nums[second - 1] == nums[second] {
          continue;
        }
        while second < third && nums[second] + nums[third] > target {
          third -= 1;
        }
        if second == third {
          break;
        }
        if nums[second] + nums[third] == target {
          ans.push(vec![nums[first], nums[second], nums[third]])
        }
      }
    }
    ans
  }
}
