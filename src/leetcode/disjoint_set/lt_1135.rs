#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn minimum_cost(n: i32, mut connections: Vec<Vec<i32>>) -> i32 {
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
        let mut ans = 0;
        let mut picked = 0;
        connections.sort_unstable_by(|a, b| a[2].cmp(&b[2]));
        for tuple in connections.iter() {
            if picked == n - 1 {
                break;
            }
            if dsu.find(tuple[0] as usize, tuple[1] as usize) {
                continue;
            }
            dsu.union(tuple[0] as usize, tuple[1] as usize);
            ans += tuple[2];
            picked += 1;
        }
        if dsu.num_of_root() == 1 {
            ans
        } else {
            -1
        }
    }
}
