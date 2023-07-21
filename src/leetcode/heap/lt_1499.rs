crate::sln!();

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut res = i32::MIN;
        let mut heap = BinaryHeap::<Reverse<(i32, i32)>>::new();
        for (x, y) in points.into_iter().map(|v| (v[0], v[1])) {
            while let Some(top) = heap.peek() {
                if x - top.0 .1 > k {
                    heap.pop();
                } else {
                    break;
                }
            }
            if let Some(top) = heap.peek() {
                res = res.max(x + y + top.0 .0);
            }
            heap.push(Reverse((x - y, x)));
        }
        res
    }
}
