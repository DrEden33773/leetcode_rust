#![allow(dead_code)]

pub struct Solution;

impl Solution {
    #[inline]
    fn min_max_element(nums: &[i32]) -> (i32, i32) {
        nums.iter().fold((nums[0], nums[0]), |pair, num| {
            let (mut min, mut max) = pair;
            min = min.min(*num);
            max = max.max(*num);
            (min, max)
        })
    }
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let (min, max) = Solution::min_max_element(&nums);
        let diff = max - min;
        let mut ans = 1;
        for d in -diff..=diff {
            let mut f = vec![-1; max as usize + 1];
            for num in &nums {
                let num = *num;
                let prev = num - d;
                if prev >= min && prev <= max && f[prev as usize] != -1 {
                    f[num as usize] = f[num as usize].max(f[prev as usize] + 1);
                    ans = ans.max(f[num as usize]);
                }
                f[num as usize] = f[num as usize].max(1);
            }
        }
        ans
    }
}
