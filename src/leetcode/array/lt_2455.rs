#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let (sum, num) = nums
            .into_iter()
            .filter(|&num| num % 2 == 0 && num % 3 == 0)
            .fold((0, 0), |(sum, num), curr| (sum + curr, num + 1));
        if num != 0 {
            sum / num
        } else {
            0
        }
    }
}
