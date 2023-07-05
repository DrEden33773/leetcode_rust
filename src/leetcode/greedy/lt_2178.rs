crate::sln!();

impl Solution {
    pub fn maximum_even_split(mut final_sum: i64) -> Vec<i64> {
        if final_sum % 2 != 0 {
            return vec![];
        }
        let mut ans = vec![];
        for i in (2..).step_by(2) {
            if i > final_sum {
                break;
            }
            ans.push(i);
            final_sum -= i;
        }
        *ans.last_mut().unwrap() += final_sum;
        ans
    }
}
