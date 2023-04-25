#![allow(dead_code)]

pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    #[inline(always)]
    fn bfs_from_vertex(v: i32, adj_table: &Vec<Vec<i32>>, visited: &mut HashSet<i32>) {
        let mut q = VecDeque::new();
        q.push_back(v);
        while !q.is_empty() {
            let curr = q.pop_front().unwrap();
            visited.insert(curr);
            for &adj in adj_table[curr as usize].iter() {
                if !visited.contains(&adj) {
                    q.push_back(adj);
                }
            }
        }
    }
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = HashSet::new();
        Solution::bfs_from_vertex(0, &rooms, &mut visited);
        for v in 0..rooms.len() as i32 {
            if !visited.contains(&v) {
                return false;
            }
        }
        true
    }
}
