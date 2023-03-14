use rand::Rng;

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    fn if_ascending(input: &[i32]) -> bool {
        for (l, r) in input.windows(2).map(|s| (&s[0], &s[1])) {
            if l > r {
                return false;
            }
        }
        true
    }

    #[allow(dead_code)]
    pub fn sort_colors(nums: &mut Vec<i32>) {
        /* [0, eof_0) => 0 */
        let mut eof_0: isize = -1;

        /* [eof_0, eof_1) => 1 */
        let mut eof_1: isize = -1;

        /* [eof_1, eof_2] => 2 */
        for eof_2 in 0..nums.len() {
            if nums[eof_2] != 2 {
                eof_1 += 1;
                nums.swap(eof_2, eof_1 as usize);
                if nums[eof_1 as usize] != 1 {
                    eof_0 += 1;
                    nums.swap(eof_1 as usize, eof_0 as usize);
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn sort_colors_official(nums: &mut Vec<i32>) {
        let mut v = [0; 3];
        for i in 0..nums.len() {
            v[nums[i] as usize] += 1;
        }
        let mut i: usize = 0;
        for _ in 0..v[0] {
            nums[i] = 0;
            i += 1;
        }
        for _ in 0..v[1] {
            nums[i] = 1;
            i += 1;
        }
        for _ in 0..v[2] {
            nums[i] = 2;
            i += 1;
        }
    }

    #[allow(dead_code)]
    pub fn test() {
        let mut gen: Vec<i32> = (0..15)
            .map(|_| rand::thread_rng().gen_range(0..=2))
            .collect();
        println!("before => {:?}", &gen);
        Self::sort_colors(&mut gen);
        println!("after => {:?}", &gen);
        match Self::if_ascending(&gen) {
            true => println!("Correct!"),
            false => println!("Error!"),
        }
        println!();
    }
}
