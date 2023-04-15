#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

struct DisjointSet<T: Hash + Eq + Clone> {
    dict: HashMap<T, T>,
}

impl<T: Hash + Eq + Clone> DisjointSet<T> {
    fn new(set: HashSet<T>) -> Self {
        let dict = set.iter().fold(HashMap::new(), |mut dict, e| {
            dict.insert(e.clone(), e.clone());
            dict
        });
        Self { dict }
    }
    fn find(&self, target: &T) -> T {
        let parent = self.dict.get(target).unwrap();
        if parent == target {
            target.to_owned()
        } else {
            self.find(parent)
        }
    }
    fn union(&mut self, from: &T, to: &T) {
        let root_of_from = self.find(from);
        let root_of_to = self.find(to);
        *self.dict.get_mut(&root_of_from).unwrap() = root_of_to;
    }
}

#[cfg(test)]
mod disjoint_set {}
