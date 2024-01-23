struct ZigzagIterator<T>
where
  T: Clone,
{
  pc: usize,
  data: Vec<T>,
}

impl<T> ZigzagIterator<T>
where
  T: Clone,
{
  fn new(v1: Vec<T>, v2: Vec<T>) -> Self {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut data = vec![];
    while p1 < v1.len() && p2 < v2.len() {
      data.push(v1[p1].to_owned());
      data.push(v2[p2].to_owned());
      p1 += 1;
      p2 += 1;
    }
    while p1 < v1.len() {
      data.push(v1[p1].to_owned());
      p1 += 1;
    }
    while p2 < v2.len() {
      data.push(v2[p2].to_owned());
      p2 += 1;
    }
    Self { pc: 0, data }
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

struct AdvancedZigzagIterator<T: Clone> {
  pc: usize,
  data: Vec<T>,
}

impl<T: Clone> AdvancedZigzagIterator<T> {
  fn new(vs: Vec<Vec<T>>) -> Self {
    let mut data = vec![];
    let max_len = vs.iter().max_by_key(|&v| v.len()).unwrap().len();
    (0..max_len).for_each(|i| {
      vs.iter().for_each(|v| {
        if let Some(e) = v.get(i) {
          data.push(e.to_owned())
        }
      });
    });
    Self { pc: 0, data }
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
  fn data(&self) -> Vec<T> {
    self.data.to_owned()
  }
}

#[cfg(test)]
mod test_advanced_zigzag_iterator {
  use super::*;

  #[test]
  fn it_works() {
    let iter = AdvancedZigzagIterator::new(vec![vec![1, 2, 3], vec![4, 5, 6, 7], vec![8, 9]]);
    assert_eq!(iter.data(), vec![1, 4, 8, 2, 5, 9, 3, 6, 7])
  }
}
