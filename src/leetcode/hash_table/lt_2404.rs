#![allow(dead_code)]

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let even_table = nums.into_iter().fold(HashMap::new(), |mut table, num| {
            if num % 2 == 0 {
                *table.entry(num).or_insert(0) += 1;
            }
            table
        });
        let (mut ans, mut ans_count) = (-1, 0);
        for (even, even_count) in even_table.iter() {
            if ans == -1 || *even_count > ans_count {
                (ans, ans_count) = (*even, *even_count)
            } else if *even_count == ans_count && *even < ans {
                (ans, ans_count) = (*even, *even_count)
            }
        }
        ans
    }
}

#[cfg(test)]
mod most_frequent_even {
    use super::*;

    #[test]
    fn it_works() {
        let nums_vec = vec![
            vec![0, 1, 2, 2, 4, 4, 1],
            vec![4, 4, 4, 9, 2, 4],
            vec![29, 47, 21, 41, 13, 37, 25, 7],
        ];
        for nums in nums_vec {
            let ans = Solution::most_frequent_even(nums.to_owned());
            eprintln!("{:?} => {}", nums, ans)
        }
    }
}
