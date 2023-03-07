#![allow(dead_code)]

use passwords::PasswordGenerator;
use rand::Rng;
use std::collections::BinaryHeap;
use std::time::{self, Duration};

pub fn safe_equal(lhs: &String, rhs: &String) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }
    let mut mask: u8 = 0;
    for (l, r) in lhs.chars().zip(rhs.chars()) {
        mask |= (l != r) as u8;
    }
    mask == 0
}

pub fn equal(lhs: &String, rhs: &String) -> bool {
    for (l, r) in lhs.chars().zip(rhs.chars()) {
        if l != r {
            return false;
        }
    }
    true
}

fn random_password() -> String {
    let pg = PasswordGenerator {
        length: rand::thread_rng().gen_range(8..=16),
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: false,
        exclude_similar_characters: false,
        strict: false,
    };
    pg.generate_one().unwrap()
}

#[derive(Default)]
struct CharTimePair {
    c: char,
    t: Duration,
}

impl PartialEq for CharTimePair {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}
impl Eq for CharTimePair {}
impl PartialOrd for CharTimePair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.t.partial_cmp(&other.t) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.c.partial_cmp(&other.c)
    }
}
impl Ord for CharTimePair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn crack_password_with_equal() {
    let dict = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*',
        '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`',
        '{', '|', '}', '~',
    ];
    let password = random_password();
    let mut cracked = String::new();
    while cracked.len() < password.len() {
        let mut heap = BinaryHeap::<CharTimePair>::new();
        for c in dict {
            cracked.push(c);
            let start = time::Instant::now();
            equal(&cracked, &password);
            let end = time::Instant::now();
            let duration = end - start;
            heap.push(CharTimePair { c, t: duration });
            cracked.pop().unwrap();
        }
        let CharTimePair { c, t: _ } = heap.peek().unwrap();
        eprintln!("{} => {}", cracked.len(), c);
        cracked.push(*c);
    }
    eprintln!("original => {}", password);
    eprintln!("cracked => {}", cracked);
}

fn crack_first_pos_with_equal() {
    let dict = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*',
        '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`',
        '{', '|', '}', '~',
    ];
    let password = random_password();
    let mut cracked = String::new();
    let mut vec = Vec::<CharTimePair>::new();
    for c in dict {
        cracked.push(c);
        let start = time::Instant::now();
        equal(&cracked, &password);
        let end = time::Instant::now();
        let duration = end - start;
        vec.push(CharTimePair { c, t: duration });
        cracked.pop().unwrap();
    }
    vec.sort_by(|l, r| r.cmp(l));
    eprintln!("password[0] => {}", password.chars().nth(0).unwrap());
    eprintln!();
    for CharTimePair { c, t } in vec {
        eprintln!("{} => {}s", c, t.as_secs_f64())
    }
}

#[cfg(test)]
mod safe_str_equal {
    use super::*;

    #[test]
    fn test_random_password_gen() {
        let pg = PasswordGenerator {
            length: rand::thread_rng().gen_range(8..=16),
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: true,
            symbols: true,
            spaces: false,
            exclude_similar_characters: false,
            strict: false,
        };
        let a = pg.generate_one().unwrap();
        dbg!(a);
    }

    #[test]
    fn test_safe_equal() {
        let pg = PasswordGenerator {
            length: rand::thread_rng().gen_range(8..=16),
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: true,
            symbols: true,
            spaces: false,
            exclude_similar_characters: false,
            strict: false,
        };
        let a = pg.generate_one().unwrap();
        assert_eq!(safe_equal(&a, &a), true);
        let l = "llll".to_owned();
        let r = "rrrr".to_owned();
        assert_eq!(safe_equal(&l, &r), false);
    }

    #[test]
    fn test_crack_password_with_equal() {
        // crack_password_with_equal();
        crack_first_pos_with_equal();
    }
}
