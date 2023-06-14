crate::Solution!();

impl Solution {
    /// # num_times_all_blue
    ///
    /// ## Core
    ///
    /// record rightest flip index
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let n = flips.len();
        let (mut ans, mut right) = (0, 0);
        (0..n).for_each(|i| {
            right = right.max(flips[i] as usize);
            ans += if right == i + 1 { 1 } else { 0 };
        });
        ans
    }
}
