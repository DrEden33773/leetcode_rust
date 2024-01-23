#![allow(dead_code)]

pub struct Solution;

use std::collections::HashMap;

impl Solution {
  #[inline]
  fn resort_equations(equations: Vec<String>) -> Vec<String> {
    let mut assignments = Vec::<String>::with_capacity(equations.len());
    let mut evaluations = Vec::<String>::with_capacity(equations.len());
    for equation in equations {
      if equation.chars().nth(1).unwrap() == '=' {
        assignments.push(equation)
      } else {
        evaluations.push(equation)
      }
    }
    [assignments, evaluations].concat()
  }
  pub fn equations_possible(equations: Vec<String>) -> bool {
    struct Dsu {
      parent: HashMap<char, char>,
    }
    impl Dsu {
      #[inline]
      fn init() -> Self {
        Self {
          parent: ('a'..='z').fold(HashMap::new(), |mut parent, c| {
            parent.insert(c, c);
            parent
          }),
        }
      }
      #[inline]
      fn get_root(&self, mut n: char) -> char {
        let mut pa = *self.parent.get(&n).unwrap();
        while n != pa {
          n = pa;
          pa = *self.parent.get(&n).unwrap();
        }
        n
      }
      #[inline]
      fn union(&mut self, m: char, n: char) {
        let m_root = self.get_root(m);
        let n_root = self.get_root(n);
        *self.parent.entry(m_root).or_default() = n_root
      }
      #[inline]
      fn find(&self, m: char, n: char) -> bool {
        self.get_root(m) == self.get_root(n)
      }
    }
    let equations = Solution::resort_equations(equations);
    let mut dsu = Dsu::init();
    for equation in equations {
      /* chars.next() == chars.nth(0) */
      let left = equation.chars().next().unwrap();
      let right = equation.chars().nth(3).unwrap();
      let tag = equation.chars().nth(1).unwrap();
      if tag == '=' {
        dsu.union(left, right);
      } else if dsu.find(left, right) {
        return false;
      }
    }
    true
  }
}

#[cfg(test)]
mod equations_possible {
  use super::*;

  #[test]
  fn it_works() {
    let equations = vec!["a==b", "b!=c", "c==a"]
      .into_iter()
      .map(|str| str.to_owned())
      .collect();
    let res = Solution::equations_possible(equations);
    eprintln!("{}", res)
  }
}
