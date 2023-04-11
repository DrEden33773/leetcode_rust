#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut pos = (0, 0);
        let mut direction = (0, 1);
        fn turn_left(direction: (i32, i32)) -> (i32, i32) {
            match direction {
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                _ => (0, 0),
            }
        }
        fn turn_right(direction: (i32, i32)) -> (i32, i32) {
            match direction {
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                _ => (0, 0),
            }
        }
        for instruction in instructions.as_bytes() {
            match *instruction {
                b'G' => {
                    pos.0 += direction.0;
                    pos.1 += direction.1;
                }
                b'L' => direction = turn_left(direction),
                b'R' => direction = turn_right(direction),
                _ => {}
            }
        }
        if pos == (0, 0) || direction != (0, 1) {
            true
        } else {
            false
        }
    }
}
