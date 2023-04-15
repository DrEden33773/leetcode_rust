#![allow(dead_code)]

use std::collections::HashMap;

struct Dsu<T> {
    table: HashMap<T, T>,
}

impl<T> Dsu<T> {}
