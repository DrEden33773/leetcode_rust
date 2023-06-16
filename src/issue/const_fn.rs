#![allow(dead_code)]

pub const fn fib(i: usize) -> usize {
    match i {
        i if i < 2 => 0,
        _ => fib(i - 1) + fib(i - 2),
    }
}
