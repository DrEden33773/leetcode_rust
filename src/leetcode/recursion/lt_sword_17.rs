#![allow(dead_code)]

pub struct Solution;

impl Solution {
    fn recursive_core(x: f64, n: i128) -> f64 {
        if n == 0 {
            return 1.0;
        }
        let y = Solution::recursive_core(x, n / 2);
        match n % 2 == 0 {
            true => y * y,
            false => y * y * x,
        }
    }
    fn core(x: f64, mut n: i128) -> f64 {
        let mut res = 1.0;
        let mut x_contribute = x;
        while n > 0 {
            if n % 2 == 1 {
                res *= x_contribute;
            }
            x_contribute *= x_contribute;
            n /= 2;
        }
        res
    }
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let n = n as i128;
        match n >= 0 {
            true => Solution::core(x, n),
            false => 1.0 / Solution::core(x, -n),
        }
    }
    pub fn my_recursive_pow(x: f64, n: i32) -> f64 {
        let n = n as i128;
        match n >= 0 {
            true => Solution::recursive_core(x, n),
            false => 1.0 / Solution::recursive_core(x, -n),
        }
    }
}
