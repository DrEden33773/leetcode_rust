#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        cost.push(0);
        let mut i = cost.len() - 1;
        while i > 1 {
            let move_forward: usize = match cost[i - 2].cmp(&cost[i - 1]) {
                std::cmp::Ordering::Less => 2,
                _ => 1,
            };
            *cost.last_mut().unwrap() += cost[i - move_forward];
            i -= move_forward;
        }
        cost.last().unwrap().to_owned()
    }
}

#[cfg(test)]
mod min_cost_climbing_stairs {
    use super::*;

    #[test]
    fn two_cases() {
        let case1 = vec![10, 15, 20];
        let case2 = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        let res1 = Solution::min_cost_climbing_stairs(case1.clone());
        let res2 = Solution::min_cost_climbing_stairs(case2.clone());
        eprintln!("{:?} => {res1}", case1);
        eprintln!("{:?} => {res2}", case2);
    }
}
