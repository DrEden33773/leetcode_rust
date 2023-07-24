crate::sln!();

impl Solution {
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        #[derive(PartialEq, PartialOrd, Clone, Copy)]
        struct NonNanDouble(f64);
        impl Eq for NonNanDouble {}
        impl Ord for NonNanDouble {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.partial_cmp(other).unwrap()
            }
        }
        impl From<f64> for NonNanDouble {
            fn from(value: f64) -> Self {
                NonNanDouble(value)
            }
        }
        impl NonNanDouble {
            fn new(value: f64) -> Self {
                if value.is_nan() {
                    panic!("Must build from an `NonNaN` value, but received `NaN`!")
                }
                Self(value)
            }
        }
        use std::collections::BinaryHeap;
        let sum = NonNanDouble::new(nums.iter().map(|num| *num as f64).sum::<f64>());
        let mut used = NonNanDouble::new(0.0);
        let mut steps = 0;
        let mut pq = nums
            .into_iter()
            .map(|num| NonNanDouble::new(num as f64))
            .collect::<BinaryHeap<_>>();
        while used.0 < sum.0 / 2.0 {
            let max = pq.pop().unwrap();
            let half = max.0 as f64 / 2.0;
            used.0 += half;
            pq.push(NonNanDouble::new(half));
            steps += 1;
        }
        steps
    }
}
