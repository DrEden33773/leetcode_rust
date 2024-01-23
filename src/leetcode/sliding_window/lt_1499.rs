crate::sln!();

impl Solution {
  pub fn slower_find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
    use std::collections::VecDeque;
    let mut res = i32::MIN;
    let mut q = VecDeque::<(i32, i32)>::new();
    for (x, y) in points.iter().map(|v| (v[0], v[1])) {
      while matches!(q.front(), Some((_, yi)) if x - *yi > k) {
        q.pop_front();
      }
      if let Some((xi, _)) = q.front() {
        res = res.max(x + y + xi);
      }
      while matches!(q.back(), Some((xi, _)) if y - x >= *xi) {
        q.pop_back();
      }
      q.push_back((y - x, x))
    }
    res
  }
  pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
    use std::collections::VecDeque;
    let mut stk = VecDeque::new();
    stk.push_back(&points[0]);
    let mut res = i32::MIN;
    for p in points.iter().skip(1) {
      while matches!(stk.front(), Some(f) if p[0] - f[0] > k) {
        stk.pop_front();
      }
      if let Some(f) = stk.front() {
        res = res.max(f[1] - f[0] + p[1] + p[0]);
      }
      while matches!(stk.back(), Some(b) if b[1] - b[0] < p[1] - p[0]) {
        stk.pop_back();
      }
      stk.push_back(p);
    }
    res
  }
}
