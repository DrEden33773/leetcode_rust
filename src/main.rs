#![allow(dead_code)]

mod issue;
mod leetcode;
mod luogu;

fn i64_to_f64_test() {
    println!("{}", i64::MAX);
    println!("2^{}", f64::MAX_EXP);
    assert!(f64::MAX >= i64::MAX as f64);
    println!()
}

fn show_info() {
    println!();
    println!("Please use `test` instead of running in `main func`...");
    println!()
}

fn main() {
    show_info();
}
