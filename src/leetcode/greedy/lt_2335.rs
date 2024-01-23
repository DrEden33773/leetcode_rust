use rand::Rng;

#[allow(dead_code)]

pub struct Solution;

impl Solution {
  #[allow(dead_code)]
  pub fn fill_cups(mut amount: Vec<i32>) -> i32 {
    assert_eq!(amount.len(), 3);
    amount.sort();
    let a = &amount;
    match a[2] >= a[0] + a[1] {
      true => a[2],
      // false => match a[0] + a[1] - a[2] % 2 == 0 {
      //     true => a.iter().sum::<i32>() / 2,
      //     false => (a.iter().sum::<i32>() + 1) / 2,
      // },
      false => (a.iter().sum::<i32>() + 1) / 2,
    }
  }

  #[allow(dead_code, unused)]
  pub fn test() {
    let mut to_fill: Vec<i32> = (0..3)
      .map(|_| rand::thread_rng().gen_range(0..=100))
      .collect();
    to_fill = vec![5, 4, 4];
    println!(
      "{:?} => {} seconds",
      to_fill.clone(),
      Self::fill_cups(to_fill)
    );
    println!();
  }
}
