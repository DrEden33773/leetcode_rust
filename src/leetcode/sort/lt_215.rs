#![allow(dead_code)]

pub struct Solution;

use rand::{self, Rng};

impl Solution {
    #[inline]
    fn partition(v: &mut [i32], l: usize, r: usize) -> usize {
        let last = v[r - 1];
        let mut pivot = l as i32 - 1;
        for i in l..r {
            if v[i] <= last {
                pivot += 1;
                v.swap(pivot as usize, i);
            }
        }
        pivot as usize
    }
    #[inline]
    fn random_partition(v: &mut [i32], l: usize, r: usize) -> usize {
        let pivot = rand::thread_rng().gen_range(l..r);
        v.swap(pivot, r - 1);
        Solution::partition(v, l, r)
    }
    #[inline]
    fn quick_select(v: &mut Vec<i32>, l: usize, r: usize, index: usize) -> i32 {
        let p = Solution::random_partition(v, l, r);
        match p.cmp(&index) {
            std::cmp::Ordering::Less => Solution::quick_select(v, p + 1, r, index),
            std::cmp::Ordering::Equal => v[p],
            std::cmp::Ordering::Greater => Solution::quick_select(v, l, p, index),
        }
    }
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let expected_index = len - k as usize;
        Solution::quick_select(&mut nums, 0, len, expected_index)
    }
}

#[cfg(test)]
mod find_kth_largest {
    use super::*;

    #[test]
    fn it_works() {
        let first = Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2);
        let second = Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4);
        eprintln!("{first}");
        eprintln!("{second}");
    }
}
