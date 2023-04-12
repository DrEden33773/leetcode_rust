#![allow(dead_code)]

pub struct Solution;

impl Solution {
    /// ## Back Track Helper
    ///
    /// ### Core:
    ///
    /// **Enumerate** all combinations, by correctly `swapping`
    #[inline]
    fn back_track(ans: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, beg: usize, len: usize) {
        if beg == len {
            ans.push(nums.to_owned());
            return;
        }
        for i in beg..len {
            nums.swap(beg, i);
            Solution::back_track(ans, nums, beg + 1, len);
            nums.swap(beg, i);
        }
    }
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let len = nums.len();
        Solution::back_track(&mut ans, &mut nums, 0, len);
        ans
    }
}
