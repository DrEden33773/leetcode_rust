#![allow(dead_code)]

pub struct Solution;

impl Solution {
    fn dfs(start: i32, graph: &Vec<Vec<i32>>, visited: &mut [bool], res: &mut Vec<Vec<i32>>) {
        visited[start as usize] = true;
        for &adj in &graph[start as usize] {
            if visited[adj as usize] == true {
                continue;
            }
            Self::dfs(adj, graph, visited, res);
        }
    }
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut visited = vec![false; graph.len()];
        Solution::dfs(0, &graph, &mut visited, &mut res);
        res
    }
}
