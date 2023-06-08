#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn hardest_worker(_n: i32, mut logs: Vec<Vec<i32>>) -> i32 {
        for i in (1..logs.len()).rev() {
            logs[i][1] -= logs[i - 1][1];
        }
        logs.sort_unstable_by(|a, b| match a[1].cmp(&b[1]) {
            std::cmp::Ordering::Equal => b[0].cmp(&a[0]),
            _ => a[1].cmp(&b[1]),
        });
        logs.last().unwrap()[0]
    }
    pub fn hardest_worker_hash_table(n: i32, mut logs: Vec<Vec<i32>>) -> i32 {
        let mut id_duration_table = vec![0; n as usize];
        for i in (1..logs.len()).rev() {
            logs[i][1] -= logs[i - 1][1];
        }
        for log in logs {
            let (id, duration) = (log[0] as usize, log[1]);
            id_duration_table[id] = duration.max(id_duration_table[id]);
        }
        let mut curr_id = 0;
        let mut max_duration = 0;
        for (id, duration) in id_duration_table.into_iter().enumerate() {
            match duration.cmp(&max_duration) {
                std::cmp::Ordering::Greater => {
                    curr_id = id;
                    max_duration = duration;
                }
                std::cmp::Ordering::Equal if id < curr_id => curr_id = id,
                _ => {}
            }
        }
        curr_id as i32
    }
}

#[cfg(test)]
mod hardest_worker {
    use super::*;

    #[test]
    fn it_works() {
        let n = 26;
        let logs = vec![vec![1, 1], vec![3, 7], vec![2, 12], vec![7, 17]];
        let _res = Solution::hardest_worker(n, logs);
    }
}
