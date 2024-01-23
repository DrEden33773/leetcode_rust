#[derive(Debug, PartialEq, Eq)]
pub enum NestedList<T: Clone> {
  Element(T),
  List(Vec<NestedList<T>>),
}

struct GenericNestedIterator<T: Clone> {
  pc: usize,
  data: Vec<T>,
}

impl<T: Clone> GenericNestedIterator<T> {
  fn collect(nested_list: Vec<NestedList<T>>) -> Vec<T> {
    nested_list
      .into_iter()
      .flat_map(|x| match x {
        NestedList::Element(x) => [x].into(),
        NestedList::List(x) => Self::collect(x),
      })
      .collect()
  }
  fn new(nested_list: Vec<NestedList<T>>) -> Self {
    Self {
      pc: 0,
      data: Self::collect(nested_list),
    }
  }
  fn next(&mut self) -> T {
    if self.pc < self.data.len() {
      let ret = self.data[self.pc].to_owned();
      self.pc += 1;
      ret
    } else {
      self.data.last().unwrap().to_owned()
    }
  }
  fn has_next(&self) -> bool {
    self.pc < self.data.len()
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>),
}

struct NestedIterator {
  pc: usize,
  data: Vec<i32>,
}

impl NestedIterator {
  fn collect(nested_list: Vec<NestedInteger>) -> Vec<i32> {
    nested_list
      .into_iter()
      .flat_map(|x| match x {
        NestedInteger::Int(x) => vec![x],
        NestedInteger::List(x) => Self::collect(x),
      })
      .collect()
  }
  fn new(nested_list: Vec<NestedInteger>) -> Self {
    Self {
      pc: 0,
      data: Self::collect(nested_list),
    }
  }
  fn next(&mut self) -> i32 {
    if self.pc < self.data.len() {
      let ret = self.data[self.pc];
      self.pc += 1;
      ret
    } else {
      self.data.last().unwrap().to_owned()
    }
  }
  fn has_next(&self) -> bool {
    self.pc < self.data.len()
  }
}
