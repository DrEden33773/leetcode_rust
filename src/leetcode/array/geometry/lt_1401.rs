crate::Solution!();

impl Solution {
  pub fn check_overlap(
    radius: i32,
    x_center: i32,
    y_center: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
  ) -> bool {
    let mut dist = 0;
    if x_center < x1 || x_center > x2 {
      dist += (x_center - x1).pow(2).min((x_center - x2).pow(2));
    }
    if y_center < y1 || y_center > y2 {
      dist += (y_center - y1).pow(2).min((y_center - y2).pow(2));
    }
    dist <= radius.pow(2)
  }
}
