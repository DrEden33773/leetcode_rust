crate::sln!();

use std::collections::BinaryHeap;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        /* Pre-definitions */
        #[derive(PartialEq, Eq)]
        struct Board(i32, usize);
        impl PartialOrd for Board {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                other.0.partial_cmp(&self.0)
            }
        }
        impl Ord for Board {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                other.0.cmp(&self.0)
            }
        }

        /* Handle special case */
        let n = height.len();
        if n <= 2 {
            return 0;
        }

        /* Init edge */
        let mut pq = BinaryHeap::<Board>::new();
        let mut visit = vec![false; n];
        for x in [0, n - 1] {
            pq.push(Board(height[x], x));
            visit[x] = true;
        }

        /* Pick `shortest_edge`, let it decide surrounding blocks' updating, finally add updated block as `edge` */
        let mut ans = 0;
        let all_shf = [-1, 1];
        while let Some(shortest_edge) = pq.pop() {
            let edge = shortest_edge;
            for shf in all_shf {
                let nx = (edge.1 as i32 + shf) as usize;
                if (0..n).contains(&nx) && !visit[nx] {
                    if height[nx] < edge.0 {
                        ans += edge.0 - height[nx];
                    }
                    visit[nx] = true;
                    pq.push(Board(height[nx].max(edge.0), nx));
                }
            }
        }

        ans
    }
}
