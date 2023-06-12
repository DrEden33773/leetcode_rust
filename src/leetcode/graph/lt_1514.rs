#![allow(dead_code)]

pub struct Solution;

use std::collections::HashSet;

impl Solution {
    fn dijkstra(mat: Vec<Vec<f64>>, src: usize) -> Vec<f64> {
        let num = mat.len();
        let mut unjoined: HashSet<usize> = (0..num).filter(|&v| v != src).collect();
        let mut dist: Vec<f64> = (0..num).map(|v| mat[src][v]).collect();
        while !unjoined.is_empty() {
            let (closest, _) = dist
                .iter()
                .enumerate()
                .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
                .unwrap();
            unjoined.remove(&closest);
            for &v in &unjoined {
                let new_dist = dist[closest] + mat[closest][v];
                if new_dist < dist[v] {
                    dist[v] = new_dist;
                }
            }
        }
        dist
    }
    fn build_mat(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>) -> Vec<Vec<f64>> {
        let mut mat = vec![vec![f64::INFINITY; n as usize]; n as usize];
        (0..edges.len()).for_each(|i| {
            let (from, to) = (edges[i][0] as usize, edges[i][1] as usize);
            mat[from][to] = succ_prob[i];
        });
        mat
    }
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let mat = Solution::build_mat(n, edges, succ_prob);
        let dist = Solution::dijkstra(mat, start as usize);
        dist[end as usize]
    }
}
