crate::sln!();

impl Solution {
  pub fn calculate(s: String) -> i32 {
    let str = s.as_bytes();
    let mut stack = vec![];
    let mut pre_opt = b'+';
    let mut num = 0;
    for (i, &c) in str.iter().enumerate() {
      if c.is_ascii_digit() {
        num *= 10;
        num += (c - b'0') as i32;
      }
      if (!c.is_ascii_digit() && c != b' ') || i == s.len() - 1 {
        match pre_opt {
          b'+' => stack.push(num),
          b'-' => stack.push(-num),
          b'*' => *stack.last_mut().unwrap() *= num,
          _ => *stack.last_mut().unwrap() /= num,
        }
        pre_opt = c;
        num = 0;
      }
    }
    stack.iter().sum()
  }
}
