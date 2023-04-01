#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_unstable_by(|l, r| l[1].cmp(&r[1]));
        let (mut curr, mut res) = (i32::MIN, 0);
        for pair in pairs {
            if curr < pair[0] {
                curr = pair[1];
                res += 1;
            }
        }
        res
    }
}
