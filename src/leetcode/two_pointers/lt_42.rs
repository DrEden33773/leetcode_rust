crate::sln!();

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let n = height.len();
        let mut l_max = vec![height[0]; n];
        let mut r_max = vec![height[n - 1]; n];
        (1..n).for_each(|i| l_max[i] = l_max[i - 1].max(height[i]));
        (0..n - 1)
            .rev()
            .for_each(|i| r_max[i] = r_max[i + 1].max(height[i]));
        l_max
            .into_iter()
            .zip(r_max)
            .enumerate()
            .map(|(i, (l, r))| l.min(r) - height[i])
            .sum()
    }
    pub fn two_pointers_trap(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut l_max = height[l];
        let mut r_max = height[r];
        while l < r {
            l_max = l_max.max(height[l]);
            r_max = r_max.max(height[r]);
            if height[l] < height[r] {
                ans += l_max - height[l];
                l += 1;
            } else {
                ans += r_max - height[r];
                r -= 1;
            }
        }
        ans
    }
}
