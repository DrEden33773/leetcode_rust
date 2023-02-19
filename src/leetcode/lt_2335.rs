use std::ops::{Add, Div};

use rand::Rng;

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn fill_cups(mut amount: Vec<i32>) -> i32 {
        amount.sort();
        match amount.last().unwrap() >= (&amount[0..amount.len() - 1].iter().sum()) {
            true => *amount.last().unwrap(),
            false => amount.iter().sum::<i32>().add(1).div(2),
        }
    }

    #[allow(dead_code)]
    pub fn test() {
        let to_fill: Vec<i32> = (0..3)
            .map(|_| rand::thread_rng().gen_range(0..=100))
            .collect();
        println!(
            "{:?} => {} seconds",
            to_fill.clone(),
            Self::fill_cups(to_fill)
        );
        println!();
    }
}
