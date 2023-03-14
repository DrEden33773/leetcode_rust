#![allow(dead_code, unused_mut)]

pub struct Solution;

impl Solution {
    pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
        let mut letter_prefix = vec![0; array.len()];
        let mut digit_prefix = vec![0; array.len()];
        if array[0].chars().nth(0).unwrap().is_ascii_digit() {
            digit_prefix[0] = 1;
        } else {
            letter_prefix[0] = 1;
        }
        for i in 1..array.len() {
            let mut letter_update = 0;
            let mut digit_update = 0;
            if array[i].chars().nth(0).unwrap().is_ascii_digit() {
                letter_update = 1;
            } else {
                digit_update = 1;
            }
            letter_prefix[i] += letter_prefix[i - 1] + letter_update;
            digit_prefix[i] += digit_prefix[i - 1] + digit_update;
        }
        let mut l = 0;
        let mut r = 0;
        array[l..r].iter().map(|str| str.to_owned()).collect()
    }
}
