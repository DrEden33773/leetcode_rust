crate::sln!();

use std::{collections::HashMap, ops::AddAssign};

impl Solution {
    pub fn four_sum_count(a_: Vec<i32>, b_: Vec<i32>, c_: Vec<i32>, d_: Vec<i32>) -> i32 {
        let mut count_ab = HashMap::new();
        for &a in a_.iter() {
            for &b in b_.iter() {
                count_ab.entry(a + b).or_insert(0).add_assign(1)
            }
        }
        let mut ans = 0;
        for &c in c_.iter() {
            for &d in d_.iter() {
                ans += count_ab.get(&(-c - d)).unwrap_or(&0)
            }
        }
        ans
    }
}
