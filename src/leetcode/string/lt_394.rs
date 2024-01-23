#![allow(dead_code)]

pub struct Solution;

impl Solution {
  fn get_digits(s: &str, ptr: &mut usize) -> String {
    let mut ret = String::new();
    while s.chars().nth(*ptr).unwrap().is_ascii_digit() {
      ret.push(s.chars().nth(*ptr).unwrap());
      *ptr += 1;
    }
    ret
  }
  fn get_str(strs: &[String]) -> String {
    let mut ret = String::new();
    for str in strs {
      ret += str;
    }
    ret
  }
  pub fn decode_string(s: String) -> String {
    let mut stack = vec![];
    let mut ptr: usize = 0;
    while ptr < s.len() {
      let curr = s.chars().nth(ptr).unwrap();
      if curr.is_ascii_digit() {
        let digits = Solution::get_digits(&s, &mut ptr);
        stack.push(digits);
      } else if curr.is_ascii_alphabetic() || curr == '[' {
        stack.push(curr.to_string());
        ptr += 1;
      } else {
        let mut sub = vec![];
        while stack.last().unwrap().as_str() != "[" {
          sub.push(stack.pop().unwrap());
        }
        sub.reverse();
        stack.pop();
        let rep_time = stack.pop().unwrap().parse().unwrap();
        let curr_str = Solution::get_str(&sub).repeat(rep_time);
        stack.push(curr_str);
        ptr += 1;
      }
    }
    Solution::get_str(&stack)
  }
}

#[cfg(test)]
mod decode_string {
  use super::*;

  #[test]
  fn it_works() {
    let s = "3[2[a]b]2[a2[bc]]".to_string();
    let res = Solution::decode_string(s);
    eprintln!("{res}")
  }
}
