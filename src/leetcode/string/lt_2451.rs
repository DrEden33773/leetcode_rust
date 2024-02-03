#![allow(dead_code)]

pub struct Solution;

use std::collections::HashMap;

impl Solution {
  #[inline]
  pub fn to_sub_vec_table(words: Vec<String>) -> Vec<(String, Vec<isize>)> {
    let mut table = vec![];
    for word in words {
      let mut sub_vec = vec![];
      for i in 1..word.len() {
        sub_vec.push(word.as_bytes()[i] as isize - word.as_bytes()[i - 1] as isize)
      }
      table.push((word, sub_vec));
    }
    table
  }
  #[inline]
  pub fn count_sub_vec(sub_vec_table: &Vec<(String, Vec<isize>)>) -> HashMap<Vec<isize>, isize> {
    let mut count = HashMap::new();
    for (_, vec) in sub_vec_table {
      count
        .entry(vec.to_owned())
        .and_modify(|cnt| *cnt += 1)
        .or_insert(1);
    }
    count
  }
  pub fn odd_string(words: Vec<String>) -> String {
    let sub_vec_table = Solution::to_sub_vec_table(words);
    let count = Solution::count_sub_vec(&sub_vec_table);
    let mut ret = String::new();
    for (word, vec) in sub_vec_table {
      if *count.get(&vec).unwrap() == 1 {
        ret = word;
        break;
      }
    }
    ret
  }
}

#[cfg(test)]
mod odd_string {
  use super::*;

  #[test]
  fn it_works() {
    let words = ["dtzca", "dtzca", "dtzca", "yqyyo", "dtzca", "dtzca"]
      .iter()
      .map(|&str| str.to_owned())
      .collect::<Vec<_>>();
    let ans = Solution::odd_string(words);
    eprintln!("{}", ans)
  }
}
