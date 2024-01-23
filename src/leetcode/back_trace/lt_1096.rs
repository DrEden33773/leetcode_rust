#![allow(dead_code, unused_variables, unused_mut)]

pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
  pub fn brace_expansion_ii(expression: String) -> Vec<String> {
    // special lex
    const CUP: char = ',';
    const BEG_SET: char = '{';
    const END_SET: char = '}';
    // lexer_stack
    let mut lexer: VecDeque<char> = VecDeque::new();
    // sets
    let mut sets: Vec<HashSet<char>> = vec![];
    // res
    let mut res = Vec::<String>::new();
    // opt
    for c in expression.chars() {
      // TODO
    }
    // fin
    res
  }
}
