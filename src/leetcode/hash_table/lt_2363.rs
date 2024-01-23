#![allow(dead_code)]

pub struct Solution;

impl Solution {
  pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::HashMap;
    let mut map = items1
      .iter()
      .map(|info| (info[0], info[1]))
      .collect::<HashMap<_, _>>();
    items2
      .iter()
      .map(|info| (info[0], info[1]))
      .for_each(|(v, w)| {
        if let Some(weight) = map.get_mut(&v) {
          *weight += w;
        } else {
          map.insert(v, w);
        }
      });
    let mut res = map
      .iter()
      .map(|(v, w)| vec![*v, *w])
      .collect::<Vec<Vec<i32>>>();
    res.sort_by_key(|info| info[0]);
    res
  }
}
