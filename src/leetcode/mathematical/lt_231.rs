#[allow(dead_code)]
pub struct Solution;

impl Solution {
    /// a = 2^x <=> a = 1000...000_b
    ///
    /// a = 1000...000_b <=> a - 1 = 111...111_b
    ///
    /// a & a - 1 = 0 <=> a = 2^x
    ///
    /// Now, you can solve this issue
    #[allow(dead_code)]
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0
    }
}
