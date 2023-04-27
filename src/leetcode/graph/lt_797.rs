#![allow(dead_code)]

pub struct Solution;

impl Solution {
    fn dfs(
        start: i32,
        graph: &Vec<Vec<i32>>,
        visited: &mut [bool],
        temp: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        visited[start as usize] = true;
        temp.push(start);
        if start as usize == graph.len() - 1 {
            res.push(temp.to_owned());
            return;
        }
        for &adj in &graph[start as usize] {
            if visited[adj as usize] == true {
                continue;
            }
            Self::dfs(adj, graph, visited, temp, res);
            temp.pop();
            visited[adj as usize] = false;
        }
    }
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut temp = vec![];
        let mut visited = vec![false; graph.len()];
        Solution::dfs(0, &graph, &mut visited, &mut temp, &mut res);
        res
    }
}

#[cfg(test)]
mod all_paths_source_target {
    use super::*;

    #[test]
    fn it_works() {
        let graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
        let res = Solution::all_paths_source_target(graph);
        eprintln!("{:?}", res);
    }
}
