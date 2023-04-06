#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn base_neg2(n: i32) -> String {
        if n == 0 {
            return "0".to_string();
        }
        let mut n = n;
        let mut v = Vec::<i32>::new();
        while n != 0 {
            match n % -2 {
                0 => {
                    v.push(0);
                    n /= -2;
                }
                _ => {
                    v.push(1);
                    n = (n - 1) / -2;
                }
            }
        }
        v.reverse();
        v.into_iter()
            .fold(String::new(), |acm, num| acm + &num.to_string())
    }
}

#[cfg(test)]
mod base_neg2 {
    use super::*;

    #[test]
    fn observer() {
        let a = -1 / -2;
        eprintln!("{a}");
        let a = 3 / -2;
        eprintln!("{a}");
        let a = -3 / 2;
        eprintln!("{a}");
    }

    #[test]
    fn dbg() {
        assert_eq!(&Solution::base_neg2(2), "110");
        assert_eq!(&Solution::base_neg2(3), "111");
        assert_eq!(&Solution::base_neg2(4), "100");
    }
}
