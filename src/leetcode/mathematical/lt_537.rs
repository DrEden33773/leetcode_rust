#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let mut lhs = num1.split(['+', 'i']).collect::<Vec<_>>();
        lhs.pop();
        let mut rhs = num2.split(['+', 'i']).collect::<Vec<_>>();
        rhs.pop();
        let (a, b, c, d) = (
            lhs[0].parse::<i16>().unwrap(),
            lhs[1].parse::<i16>().unwrap(),
            rhs[0].parse::<i16>().unwrap(),
            rhs[1].parse::<i16>().unwrap(),
        );
        let (ac, bd) = (a * c, b * d);
        let total = (a + b) * (c + d);
        let real = ac - bd;
        let imaginary = total - ac - bd;
        format!("{}+{}i", real, imaginary)
    }
}

#[cfg(test)]
mod complex_number_multiply {
    use super::*;

    #[test]
    fn it_works() {
        let (num1, num2) = ("1+1i".to_string(), "1+1i".to_string());
        let res = Solution::complex_number_multiply(num1, num2);
        assert_eq!(res, "0+2i".to_string());
    }
}
