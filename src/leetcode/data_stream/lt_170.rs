#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Default)]
struct TwoSum {
  map: HashMap<i32, usize>,
  vec: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TwoSum {
  fn new() -> Self {
    Self::default()
  }

  fn add(&mut self, number: i32) {
    match self.map.get_mut(&number) {
      Some(times) => {
        *times += 1;
      }
      None => {
        self.map.insert(number, 1);
      }
    }
    self.vec.push(number);
  }

  fn find(&self, value: i32) -> bool {
    for num in self.vec.iter() {
      let target = value - *num;
      if target != *num {
        if self.map.contains_key(&target) {
          return true;
        }
      } else if self.map.get(num).unwrap().gt(&1) {
        return true;
      }
    }
    false
  }
}

/*
 * Your TwoSum object will be instantiated and called as such:
 * let obj = TwoSum::new();
 * obj.add(number);
 * let ret_2: bool = obj.find(value);
 */
