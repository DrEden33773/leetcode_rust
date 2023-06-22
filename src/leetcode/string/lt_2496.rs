crate::Solution!();

impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        let arr = strs
            .into_iter()
            .map(|str| str.parse::<i32>().unwrap_or(str.len() as i32))
            .collect::<Vec<_>>();
        arr.into_iter().max().unwrap()
    }
}
