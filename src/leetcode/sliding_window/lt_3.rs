#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut counter = [0; 256];
        let bytes = s.as_str().as_bytes();
        let n = bytes.len();

        let mut ans = 0;
        let (mut l, mut r) = (0, 0);

        while r < n {
            counter[bytes[r] as usize] += 1;
            while counter[bytes[r] as usize] == 2 {
                counter[bytes[l] as usize] -= 1;
                l += 1;
            }
            ans = ans.max(r - l + 1);
            r += 1;
        }

        ans as i32
    }
}
