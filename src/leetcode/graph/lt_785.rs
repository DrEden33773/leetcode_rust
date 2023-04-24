#![allow(dead_code)]

pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    fn bfs_from_vertex(
        vertex: i32,
        adj_list: &Vec<Vec<i32>>,
        board: &mut Vec<bool>,
        visited: &mut HashSet<i32>,
    ) {
        if visited.contains(&vertex) {
            return;
        }
        let mut q = VecDeque::new();
        q.push_back(vertex);
        while !q.is_empty() {
            let curr = q.pop_front().unwrap();
            visited.insert(curr);
            for adj in &adj_list[curr as usize] {
                if !visited.contains(adj) {
                    board[*adj as usize] = !board[curr as usize];
                    q.push_back(*adj);
                }
            }
        }
    }
    fn bfs(adj_list: &Vec<Vec<i32>>, board: &mut Vec<bool>) {
        let mut visited = HashSet::new();
        for v in 0..adj_list.len() as i32 {
            Solution::bfs_from_vertex(v, adj_list, board, &mut visited);
        }
    }
    pub fn is_bipartite(adj_list: Vec<Vec<i32>>) -> bool {
        let mut board = vec![true; adj_list.len()];
        Solution::bfs(&adj_list, &mut board);
        for i in 0..adj_list.len() {
            for j in &adj_list[i] {
                if board[i] == board[*j as usize] {
                    return false;
                }
            }
        }
        return true;
    }
}
