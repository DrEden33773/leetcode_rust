#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        use std::collections::BinaryHeap;

        if s.len() <= num_rows as usize {
            return s;
        }

        #[derive(Clone, Copy)]
        struct Info {
            c: char,
            row: i32,
            col: i32,
        }
        impl PartialEq for Info {
            fn eq(&self, other: &Self) -> bool {
                self.c == other.c && self.row == other.row && self.col == other.col
            }
        }
        impl Eq for Info {}
        impl PartialOrd for Info {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                match other.row.partial_cmp(&self.row) {
                    Some(core::cmp::Ordering::Equal) => {}
                    ord => return ord,
                }
                other.col.partial_cmp(&self.col)
            }
        }
        impl Ord for Info {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.partial_cmp(&other).unwrap()
            }
        }
        impl Default for Info {
            fn default() -> Self {
                Self::new('\0', 0, 1)
            }
        }
        impl Info {
            pub fn new(c: char, row: i32, col: i32) -> Self {
                Self { c, row, col }
            }
            pub fn move_down_with(&self, c: char) -> Self {
                Self::new(c, self.row + 1, self.col)
            }
            pub fn move_left_up_with(&self, c: char) -> Self {
                Self::new(c, self.row + 1, self.col + 1)
            }
        }

        let mut if_down = true;
        let mut prev_info = Info::default();
        let mut heap = BinaryHeap::<Info>::new();
        for c in s.chars() {
            let curr_info = match if_down {
                true => prev_info.move_down_with(c),
                false => prev_info.move_left_up_with(c),
            };
            heap.push(curr_info);
            prev_info = curr_info;
            if if_down && curr_info.row == num_rows {
                if_down = false;
            }
            if !if_down && curr_info.row == 1 {
                if_down = true;
            }
        }
        let mut res = String::new();
        while let Some(info) = heap.pop() {
            res.push(info.c);
        }
        res
    }
}

#[cfg(test)]
mod z_string_convert {
    use super::*;

    #[test]
    fn it_works() {
        let str = Solution::convert("PAYPALISHIRING".to_string(), 3);
        assert_eq!(&str, "PAHNAPLSIIGYIR");
        let str = Solution::convert("PAYPALISHIRING".to_string(), 4);
        assert_eq!(&str, "PINALSIGYAHRPI");
        let str = Solution::convert("A".to_string(), 1);
        assert_eq!(&str, "A");
    }
}
