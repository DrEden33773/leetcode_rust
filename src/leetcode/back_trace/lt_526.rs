use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
pub struct Solution;

impl Solution {
  #[allow(dead_code)]
  fn back_track_solution(n: i32) -> i32 {
    /* {index: i32} => {possible: Vec<i32>} */
    let mut all_possible: HashMap<i32, Vec<i32>> = HashMap::with_capacity(n as usize);
    /* used item */
    let mut used: HashSet<i32> = HashSet::with_capacity(n as usize);
    /* result */
    let mut res = 0;

    fn init_all_possible(n: i32, all_possible: &mut HashMap<i32, Vec<i32>>) {
      for index in 1..=n {
        all_possible.insert(index, vec![]);
      }
      for index in 1..=n {
        for item in 1..=n {
          if index % item == 0 || item % index == 0 {
            all_possible.get_mut(&index).unwrap().push(item);
          }
        }
      }
    }
    fn back_track(
      all_possible: &HashMap<i32, Vec<i32>>,
      index: i32,
      n: i32,
      res: &mut i32,
      used: &mut HashSet<i32>,
    ) {
      if index > n {
        *res += 1;
        return;
      }
      for possible in all_possible.get(&index).unwrap() {
        if !used.contains(possible) {
          used.insert(*possible);
          back_track(all_possible, index + 1, n, res, used);
          used.remove(possible);
        }
      }
    }

    init_all_possible(n, &mut all_possible);
    back_track(&all_possible, 1, n, &mut res, &mut used);

    res
  }

  #[allow(dead_code)]
  fn bit_calculation_solution(n: i32) -> i32 {
    let n1 = 1usize << n;
    let mut f = vec![0; n1];
    f[0] = 1;
    for mask in 0..n1 {
      let num = mask.count_ones() as i32;
      for i in 0..n {
        if mask & (1 << i) > 0 && (num % (i + 1) == 0 || (i + 1) % num == 0) {
          f[mask] += f[mask ^ (1 << i)];
        }
      }
    }
    f[n1 - 1]
  }

  #[allow(dead_code)]
  pub fn count_arrangement(n: i32) -> i32 {
    Self::bit_calculation_solution(n)
  }
}
