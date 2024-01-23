crate::sln!();

impl Solution {
  pub fn reconstruct_matrix(upper: i32, lower: i32, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
    let mut upper = upper;
    let mut lower = lower;
    let mut ans = vec![vec![0; col_sum.len()]; 2];
    let twice = col_sum.iter().enumerate().filter(|(_, times)| **times == 2);
    let once = col_sum.iter().enumerate().filter(|(_, times)| **times == 1);
    for (i, _) in twice {
      if upper == 0 || lower == 0 {
        return vec![];
      }
      ans[0][i] = 1;
      ans[1][i] = 1;
      upper -= 1;
      lower -= 1;
    }
    for (i, _) in once {
      if upper > 0 {
        ans[0][i] = 1;
        upper -= 1;
      } else if lower > 0 {
        ans[1][i] = 1;
        lower -= 1;
      } else {
        return vec![];
      }
    }
    if upper > 0 || lower > 0 {
      return vec![];
    }
    ans
  }
}
