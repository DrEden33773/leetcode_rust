#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>),
}

struct NestedIterator {
  iters: Vec<std::iter::Peekable<Box<dyn Iterator<Item = NestedInteger>>>>,
}

use NestedInteger::{Int, List};

impl NestedIterator {
  fn new(nested_list: Vec<NestedInteger>) -> Self {
    Self {
      iters: vec![
        (Box::new(nested_list.into_iter()) as Box<dyn Iterator<Item = NestedInteger>>).peekable(),
      ],
    }
  }
  fn next(&mut self) -> i32 {
    while let Some(it) = self.iters.last_mut() {
      if let Some(v) = it.next() {
        match v {
          Int(x) => return x,
          List(l) => {
            self.iters.push(
              (Box::new(l.into_iter()) as Box<dyn Iterator<Item = NestedInteger>>).peekable(),
            );
          }
        }
      } else {
        self.iters.pop();
      }
    }
    panic!("no data")
  }
  fn has_next(&mut self) -> bool {
    while let Some(it) = self.iters.last_mut() {
      if let Some(v) = it.peek() {
        if let Int(_) = v {
          return true;
        }
        if let List(l) = it.next().unwrap() {
          self
            .iters
            .push((Box::new(l.into_iter()) as Box<dyn Iterator<Item = NestedInteger>>).peekable());
        }
      } else {
        self.iters.pop();
      }
    }
    false
  }
}
