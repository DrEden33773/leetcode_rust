crate::sln!();

impl Solution {
  pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    fn range(left: i32, right: i32) -> std::ops::Range<i32> {
      left.min(right)..right
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut res = Vec::with_capacity(rows * cols);

    let mut top = 0;
    let mut bottom = rows as i32 - 1;
    let mut left = 0;
    let mut right = cols as i32 - 1;

    while left <= right && top <= bottom {
      for col in range(left, right + 1) {
        res.push(matrix[top as usize][col as usize]);
      }
      for row in range(top + 1, bottom + 1) {
        res.push(matrix[row as usize][right as usize]);
      }
      if left < right && top < bottom {
        for col in range(left, right).rev() {
          res.push(matrix[bottom as usize][col as usize]);
        }
        for row in range(top + 1, bottom).rev() {
          res.push(matrix[row as usize][left as usize]);
        }
      }
      (left, right, top, bottom) = (left + 1, right - 1, top + 1, bottom - 1);
    }

    res
  }
}
