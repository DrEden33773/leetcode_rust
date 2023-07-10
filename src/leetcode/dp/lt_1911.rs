crate::sln!();

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let nums = nums.into_iter().map(|num| num as i64).collect::<Vec<_>>();
        let mut even = nums[0];
        let mut odd = 0;
        (1..nums.len()).for_each(|i| {
            even = even.max(odd + nums[i]);
            odd = odd.max(even - nums[i]);
        });
        even
    }
}
