crate::sln!();

impl Solution {
  pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    for row in 1..n {
      for col in 0..n {
        let mut prev_min = matrix[row - 1][col];
        prev_min = prev_min.min(
          matrix
            .get(row - 1)
            .unwrap()
            .get(col - 1)
            .unwrap_or(&prev_min)
            .to_owned(),
        );
        prev_min = prev_min.min(
          matrix
            .get(row - 1)
            .unwrap()
            .get(col + 1)
            .unwrap_or(&prev_min)
            .to_owned(),
        );
        matrix[row][col] += prev_min;
      }
    }
    matrix[n - 1].iter().min().unwrap().to_owned()
  }
}
