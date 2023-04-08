#![allow(dead_code)]

use std::io::{self, Read};

#[inline]
fn backpack_01_solution(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    let num = weights.len();
    let mut dp = vec![0; num + 1];
    // i => item_upper_lim
    for i in 1..=num {
        // j => sub_capacity
        for j in (1..=capacity).rev() {
            dp[j] = dp[j];
            let (weight, value) = (weights[i - 1], values[i - 1]);
            if weight <= j {
                dp[j] = dp[j].max(dp[j - weight] + value)
            }
        }
    }
    dp[capacity]
}

#[inline]
pub fn exec_interface() {
    let mut str = String::new();
    let mut buf = vec![];
    let (mut num, mut capacity) = (0usize, 0usize);
    let (mut weights, mut values) = (Vec::<usize>::new(), Vec::<usize>::new());
    io::stdin().read_line(&mut str).unwrap();
    buf = str.split(' ').collect();
    (num, capacity) = (buf[0].parse().unwrap(), buf[1].parse().unwrap());
    for _ in 0..num {
        io::stdin().read_line(&mut str).unwrap();
        buf = str.split(' ').collect();
        weights.push(buf[0].parse().unwrap());
        values.push(buf[1].parse().unwrap());
    }
    let res = backpack_01_solution(&weights, &values, capacity);
    println!("{res}");
}
