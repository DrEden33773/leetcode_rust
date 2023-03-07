#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        if row_index == 0 {
            return vec![1];
        }
        let mut prev = vec![1];
        for _ in 1..=row_index {
            let mut curr = vec![1];
            for s in prev.windows(2) {
                curr.push(s[0] + s[1]);
            }
            curr.push(1);
            prev = curr;
        }
        prev
    }
}

#[cfg(test)]
mod yang_hui_triangle_gen {}
