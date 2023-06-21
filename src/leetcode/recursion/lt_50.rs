crate::Solution!();

impl Solution {
    fn pow(x: f64, n: i64) -> f64 {
        match n {
            n if n < 0 => 1.0 / Solution::pow(x, -n),
            n if n == 0 => 1.0,
            n if n == 1 => x,
            n if n % 2 == 0 => Solution::pow(x, n / 2) * Solution::pow(x, n / 2),
            n if n % 2 != 0 => Solution::pow(x, n / 2) * Solution::pow(x, n / 2) * x,
            _ => 1.0,
        }
    }
    pub fn my_pow(x: f64, n: i32) -> f64 {
        Solution::pow(x, n.into())
    }
}
