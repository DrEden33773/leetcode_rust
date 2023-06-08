#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        struct Dsu {
            pa: Vec<usize>,
            num_of_root: usize,
        }
        impl Dsu {
            #[inline(always)]
            fn new(n: usize) -> Self {
                Self {
                    pa: (0..n).collect(),
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
                if self.find(m, n) {
                    return;
                }
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
        let mut dsu = Dsu::new(is_connected.len());
        for i in 0..is_connected.len() {
            for j in (i + 1)..is_connected.len() {
                if is_connected[i][j] != 0 {
                    dsu.union(i, j);
                }
            }
        }
        dsu.num_of_root() as i32
    }
}

#[cfg(test)]
mod find_circle_num {
    use super::*;

    #[test]
    fn it_works() {
        let is_connected = vec![vec![1; 3]; 3];
        let res = Solution::find_circle_num(is_connected);
        eprintln!("{}", res);
    }
}
