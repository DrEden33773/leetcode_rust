crate::sln!();

impl Solution {
  pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.len() < 4 {
      return vec![];
    }
    nums.sort();
    let nums = nums.into_iter().map(|num| num as i64).collect::<Vec<_>>();
    let target = target as i64;
    let n = nums.len();
    let mut ans = vec![];
    for i in 0..(n - 3) {
      if i > 0 && nums[i - 1] == nums[i] {
        continue;
      }
      if nums[i] + nums[i + 1] + nums[i + 2] + nums[i + 3] > target {
        break;
      }
      if nums[i] + nums[n - 3] + nums[n - 2] + nums[n - 1] < target {
        continue;
      }
      for j in (i + 1)..(n - 2) {
        if j > i + 1 && nums[j - 1] == nums[j] {
          continue;
        }
        if nums[i] + nums[j] + nums[j + 1] + nums[j + 2] > target {
          break;
        }
        if nums[i] + nums[j] + nums[n - 2] + nums[n - 1] < target {
          continue;
        }
        let mut l = j + 1;
        let mut r = n - 1;
        while l < r {
          let sum = nums[i] + nums[j] + nums[l] + nums[r];
          match sum.cmp(&target) {
            std::cmp::Ordering::Equal => {
              ans.push(vec![
                nums[i] as i32,
                nums[j] as i32,
                nums[l] as i32,
                nums[r] as i32,
              ]);
              let mut l_next = l + 1;
              while l_next < r && nums[l_next] == nums[l] {
                l_next += 1;
              }
              l = l_next;
              let mut r_next = r - 1;
              while r_next > l && nums[r_next] == nums[r] {
                r_next -= 1;
              }
              r = r_next;
            }
            std::cmp::Ordering::Less => {
              let mut l_next = l + 1;
              while l_next < r && nums[l_next] == nums[l] {
                l_next += 1;
              }
              l = l_next;
            }
            std::cmp::Ordering::Greater => {
              let mut r_next = r - 1;
              while r_next > l && nums[r_next] == nums[r] {
                r_next -= 1;
              }
              r = r_next;
            }
          }
        }
      }
    }
    ans
  }
}
