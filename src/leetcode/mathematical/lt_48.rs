crate::sln!();

impl Solution {
  pub fn rotate(matrix: &mut [Vec<i32>]) {
    let n = matrix.len();

    type IndexPair = (usize, usize);
    let mut swap = |l: IndexPair, r: IndexPair| {
      let temp = matrix[r.0][r.1];
      matrix[r.0][r.1] = matrix[l.0][l.1];
      matrix[l.0][l.1] = temp;
    };

    for i in 0..n / 2 {
      for j in 0..n {
        swap((i, j), (n - 1 - i, j));
      }
    }

    for i in 0..n {
      for j in 0..i {
        swap((i, j), (j, i));
      }
    }
  }
}
