#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        struct Dsu {
            pa: Vec<usize>,
            num_of_root: usize,
        }
        impl Dsu {
            #[inline(always)]
            fn new(n: usize) -> Self {
                Self {
                    pa: (0..=n).collect(),
                    num_of_root: n,
                }
            }
            #[inline(always)]
            fn get_root(&self, mut n: usize) -> usize {
                while self.pa[n] != n {
                    n = self.pa[n]
                }
                n
            }
            #[inline(always)]
            fn union(&mut self, m: usize, n: usize) {
                // if self.find(m, n) {
                //     return;
                // }
                let m_root = self.get_root(m);
                let n_root = self.get_root(n);
                self.pa[m_root] = n_root;
                self.num_of_root -= 1;
            }
            #[inline(always)]
            fn find(&self, m: usize, n: usize) -> bool {
                self.get_root(m) == self.get_root(n)
            }
            #[inline(always)]
            fn num_of_root(&self) -> usize {
                self.num_of_root
            }
        }
        let mut dsu = Dsu::new(n as usize);
        for edge in edges {
            let from = edge[0] as usize;
            let to = edge[1] as usize;
            if dsu.find(from, to) {
                return false;
            }
            dsu.union(from, to);
        }
        dsu.num_of_root() == 1
    }
}
