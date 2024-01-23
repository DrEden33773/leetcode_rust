#![allow(dead_code)]

fn get_tables(mats: &[(usize, usize)]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
  for window in mats.windows(2) {
    assert_eq!(window[0].1, window[1].0);
  }
  let num = mats.len();
  let mut dp = vec![vec![0; num]; num];
  let mut s = dp.clone();
  for gap in 1..num {
    for end in gap..num {
      let beg = end - gap;
      dp[beg][end] = usize::MAX;
      for sep in beg..end {
        let curr_cost = mats[beg].0 * mats[sep].1 * mats[end].1;
        let sum_cost = dp[beg][sep] + dp[sep + 1][end] + curr_cost;
        if sum_cost < dp[beg][end] {
          dp[beg][end] = sum_cost;
          s[beg][end] = sep;
        }
      }
    }
  }
  (dp, s)
}

fn get_optimal_parens(s: &Vec<Vec<usize>>) -> String {
  fn render(s: &Vec<Vec<usize>>, str: &mut String, beg: usize, end: usize) {
    if beg == end {
      *str += &format!("A{}", beg + 1);
    } else {
      *str += "(";
      render(s, str, beg, s[beg][end]);
      render(s, str, s[beg][end] + 1, end);
      *str += ")";
    }
  }
  let mut ans = String::new();
  render(s, &mut ans, 0, s.len() - 1);
  ans
}

pub fn get_mat_chain_solution(mats: &[(usize, usize)]) -> (usize, String) {
  assert!(!mats.is_empty());
  let (dp, s) = get_tables(mats);
  (dp[0][mats.len() - 1], get_optimal_parens(&s))
}

#[cfg(test)]
mod the_mat_chain_multiplication {
  use super::*;

  #[test]
  fn it_works() {
    let (times, str) =
      get_mat_chain_solution(&[(30, 35), (35, 15), (15, 5), (5, 10), (10, 20), (20, 25)]);
    eprintln!("Minium multiply times: {}\n", times);
    eprintln!("Optimal parens: {}\n", str);
  }
}
