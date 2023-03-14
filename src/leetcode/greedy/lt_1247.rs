#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        /*
            xy => times of `('x', 'y')`
            yx => times of `('y', 'x')`
        */
        let mut xy = 0;
        let mut yx = 0;

        /*
            We will have two transition strategy
                1. swap_in(xy, yx)
                2. swap_in(xy, xy) or swap_in(yx, yx)

            swap_in(xy, yx) => transport:(_,_) = 2:1 => xy, yx -= 1
            swap_in(xy, xy) => transport:(_,_) = 1:1 => xy -= 2
            swap_in(yx, yx) => transport:(_,_) = 1:1 => yx -= 2
        */
        s1.chars()
            .zip(s2.chars())
            .for_each(|(s1, s2)| match (s1, s2) {
                ('x', 'y') => xy += 1,
                ('y', 'x') => yx += 1,
                _ => (),
            });

        /*
            Notice that, `(xy + yx) = 2n + 1` will lead to no solution.
            That's for, each transition takes two pairs
        */
        if (xy + yx) % 2 == 1 {
            return -1;
        }

        /*
            Order of strategy => try `swap_in(xy, xy)/(yx, yx)` at first

            if `remaining` xy == yx == 1 :
               additionally transit 2 times

            xy / 2 => swap_in(xy, xy)
            yx / 2 => swap_in(yx, yx)
            xy % 2 + yx % 2 => swap_in(xy, yx) if in need
        */
        xy / 2 + yx / 2 + xy % 2 + yx % 2
    }
}
