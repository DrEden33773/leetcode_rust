use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

struct LRUCache {
  access_record: VecDeque<i32>,
  cache: HashMap<i32, (i32, usize)>,
  capacity: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
  fn new(capacity: i32) -> Self {
    Self {
      cache: Default::default(),
      capacity,
      access_record: Default::default(),
    }
  }

  fn get(&mut self, key: i32) -> i32 {
    if let Some(v) = self.cache.get_mut(&key) {
      v.1 += 1;
      self.access_record.push_back(key);
      v.0
    } else {
      -1
    }
  }

  fn put(&mut self, key: i32, value: i32) {
    if !self.cache.contains_key(&key) && self.is_full() {
      self.evict();
      self.cache.insert(key, (value, 1));
    } else {
      match self.cache.entry(key) {
        Entry::Occupied(mut entry) => {
          let old_count = entry.get().1;
          entry.insert((value, old_count + 1));
        }
        Entry::Vacant(entry) => {
          entry.insert((value, 1));
        }
      }
    }
    self.access_record.push_back(key);
  }

  fn evict(&mut self) {
    while let Some(k) = self.access_record.pop_front() {
      match self.cache.entry(k) {
        Entry::Occupied(mut entry) => {
          let count = entry.get().1;
          if count == 1 {
            entry.remove();
            break;
          } else {
            entry.get_mut().1 = count - 1;
          }
        }
        _ => {
          unreachable!()
        }
      }
    }
  }

  fn is_full(&self) -> bool {
    self.cache.len() == (self.capacity as usize)
  }
}
