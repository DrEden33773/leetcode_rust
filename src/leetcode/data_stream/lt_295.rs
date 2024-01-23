#![allow(dead_code)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// ## Median Finder
///
/// ### Define:
///
/// - `le = @MaxHeap[e for e in stream if e <= median]`
/// - `gt = @MaxHeap[e for e in stream if e > median]`
///
/// ### Consequence:
///
/// `gt.len() == le.len()` or `gt.len() + 1 == le.len()`
#[derive(Default)]
struct MedianFinder {
  gt: BinaryHeap<Reverse<i32>>,
  le: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
  fn new() -> Self {
    Self {
      gt: BinaryHeap::new(),
      le: BinaryHeap::new(),
    }
  }

  fn add_num(&mut self, num: i32) {
    if self.le.is_empty() || num <= *self.le.peek().unwrap() {
      self.le.push(num);
      // adjust
      if self.le.len() > self.gt.len() + 1 {
        let moved = self.le.pop().unwrap();
        self.gt.push(Reverse(moved));
      }
    } else {
      self.gt.push(Reverse(num));
      // adjust
      if self.gt.len() > self.le.len() {
        let moved = self.gt.pop().unwrap().0;
        self.le.push(moved);
      }
    }
  }

  fn find_median(&self) -> f64 {
    if self.le.len() > self.gt.len() {
      *self.le.peek().unwrap() as f64
    } else {
      (*self.le.peek().unwrap() + self.gt.peek().unwrap().0) as f64 / 2.0
    }
  }
}

/*
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
