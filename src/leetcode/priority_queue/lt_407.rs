crate::sln!();

use std::collections::BinaryHeap;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        /* Pre-definitions */
        #[derive(PartialEq, Eq)]
        struct Board(i32, (usize, usize));
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
        let m = height_map.len();
        let n = height_map[0].len();
        if m <= 2 || n <= 2 {
            return 0;
        }

        /* Init edge */
        let mut pq = BinaryHeap::<Board>::new();
        let mut visit = vec![vec![false; n]; m];
        for x in 0..m {
            for y in 0..n {
                if x == 0 || x == m - 1 || y == 0 || y == n - 1 {
                    pq.push(Board(height_map[x][y], (x, y)));
                    visit[x][y] = true;
                }
            }
        }

        /* Pick `shortest_edge`, let it decide surrounding blocks' updating, finally add updated block as `edge` */
        let mut ans = 0;
        let dirs = [-1, 0, 1, 0, -1];
        while let Some(shortest_edge) = pq.pop() {
            let edge = shortest_edge;
            for k in 0..4 {
                let nx = (edge.1 .0 as i32 + dirs[k]) as usize;
                let ny = (edge.1 .1 as i32 + dirs[k + 1]) as usize;
                if (0..m).contains(&nx) && (0..n).contains(&ny) && !visit[nx][ny] {
                    if height_map[nx][ny] < edge.0 {
                        ans += edge.0 - height_map[nx][ny];
                    }
                    visit[nx][ny] = true;
                    pq.push(Board(height_map[nx][ny].max(edge.0), (nx, ny)));
                }
            }
        }

        ans
    }
}

impl Solution {
    pub fn trap_4d(height_map: Vec<Vec<Vec<i32>>>) -> i32 {
        /* Pre-definitions */
        #[derive(PartialEq, Eq)]
        struct Board(i32, (usize, usize, usize));
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
        let x_size = height_map.len();
        let y_size = height_map[0].len();
        let z_size = height_map[0][0].len();
        if x_size <= 2 || y_size <= 2 || z_size <= 2 {
            return 0;
        }

        /* Init edge */
        let mut pq = BinaryHeap::<Board>::new();
        let mut visit = vec![vec![vec![false; z_size]; y_size]; x_size];
        for x in 0..x_size {
            for y in 0..y_size {
                for z in 0..z_size {
                    if x == 0
                        || x == x_size - 1
                        || y == 0
                        || y == y_size - 1
                        || z == 0
                        || z == z_size - 1
                    {
                        pq.push(Board(height_map[x][y][z], (x, y, y)));
                        visit[x][y][z] = true;
                    }
                }
            }
        }

        /* Pick `shortest_edge`, let it decide surrounding blocks' updating, finally add updated block as `edge` */
        let mut ans = 0;
        let all_shf = [
            (0, 0, 1),
            (0, 0, -1),
            (-1, 0, 0),
            (0, 1, 0),
            (1, 0, 0),
            (0, -1, 0),
        ];
        while let Some(lowest_edge) = pq.pop() {
            let edge = lowest_edge;
            for (x_shf, y_shf, z_shf) in all_shf {
                let nx = ((edge.1 .0 as i32) + x_shf) as usize;
                let ny = ((edge.1 .1 as i32) + y_shf) as usize;
                let nz = ((edge.1 .2 as i32) + z_shf) as usize;
                if (0..x_size).contains(&nx)
                    && (0..y_size).contains(&ny)
                    && (0..z_size).contains(&nz)
                    && !visit[nx][ny][nz]
                {
                    if height_map[nx][ny][nz] < edge.0 {
                        ans += edge.0 - height_map[nx][ny][nz];
                    }
                    visit[nx][ny][nz] = true;
                    pq.push(Board(height_map[nx][ny][nz].max(edge.0), (nx, ny, nz)));
                }
            }
        }

        ans
    }
}
