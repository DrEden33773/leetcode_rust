#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = 0;
        for price in prices {
            max_profit = std::cmp::max(max_profit, price - min_price);
            min_price = std::cmp::min(min_price, price);
        }
        max_profit
    }
}
