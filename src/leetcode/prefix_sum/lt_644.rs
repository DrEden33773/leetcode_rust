// #![allow(dead_code, unused_mut)]

pub struct Solution;

impl Solution {
    #[inline]
    fn turn_to_prefix(nums: &Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0];
        for num in nums {
            ans.push(ans.last().unwrap() + num);
        }
        ans
    }
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut ans = 0.0;
        let k = k as usize;
        let prefix = Solution::turn_to_prefix(&nums);
        for len in k..=nums.len() {
            for end in len..prefix.len() {
                let avg = (prefix[end] - prefix[end - len]) as f64 / len as f64;
                ans = if avg > ans { avg } else { ans };
            }
        }
        ans
    }
}
