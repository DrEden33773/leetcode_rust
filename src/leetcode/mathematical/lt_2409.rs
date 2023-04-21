#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let (arrive_alice, leave_alice) = (
            Solution::convert_to_date(arrive_alice),
            Solution::convert_to_date(leave_alice),
        );
        let (arrive_bob, leave_bob) = (
            Solution::convert_to_date(arrive_bob),
            Solution::convert_to_date(leave_bob),
        );
        if leave_alice < arrive_bob || leave_bob < arrive_alice {
            0
        } else {
            1 + leave_alice.min(leave_bob) - arrive_alice.max(arrive_bob)
        }
    }
    #[inline]
    fn convert_to_date(info: String) -> i32 {
        let info = info.split('-').collect::<Vec<&str>>();
        static DAYS_BEFORE: [i32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let (month, day) = (
            info[0].parse::<usize>().unwrap(),
            info[1].parse::<i32>().unwrap(),
        );
        DAYS_BEFORE[month - 1] + day
    }
}
