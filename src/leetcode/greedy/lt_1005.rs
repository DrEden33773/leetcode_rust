crate::Solution!();

impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        for num in nums.iter_mut() {
            if k > 0 && *num < 0 {
                *num *= -1;
                k -= 1;
            }
        }
        if k % 2 == 1 {
            let min = nums.iter_mut().min().unwrap();
            *min *= -1
        }
        nums.iter().sum()
    }
}
