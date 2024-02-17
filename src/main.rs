#![allow(dead_code)]

mod issue;
mod leetcode;
mod luogu;

fn main() {
  playground();
}

fn playground() {
  let a = [1, 2, 3, 4];
  assert_eq!(a[1..], a[1..a.len()])
}
