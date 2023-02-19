#[allow(dead_code)]
pub struct Solution;

impl Solution {
    /// ## LeetCode
    ///
    /// ### 1792. Maximum Average Pass Ratio
    ///
    /// **algorithm** = `greedy`
    ///
    /// **data structure** = `priority_queue` / `binary_heap`
    ///
    /// **technique** = try to cmp in `i32`, not in `f64` => `Fractional simplification`
    ///
    #[allow(dead_code)]
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        use std::collections::BinaryHeap;

        #[derive(Default)]
        struct Ratio {
            pass: i32,
            total: i32,
        }
        impl PartialEq for Ratio {
            fn eq(&self, other: &Self) -> bool {
                let self_icm = (other.total + 1) * other.total * (self.total - self.pass);
                let other_icm = (self.total + 1) * self.total * (other.total - other.pass);
                self_icm == other_icm
            }
        }
        impl Eq for Ratio {}
        impl PartialOrd for Ratio {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                let self_icm = (other.total + 1) * other.total * (self.total - self.pass);
                let other_icm = (self.total + 1) * self.total * (other.total - other.pass);
                self_icm.partial_cmp(&other_icm)
            }
        }
        impl Ord for Ratio {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                let self_icm = (other.total + 1) * other.total * (self.total - self.pass);
                let other_icm = (self.total + 1) * self.total * (other.total - other.pass);
                self_icm.partial_cmp(&other_icm).unwrap()
            }
        }
        impl Ratio {
            fn get_ratio(&self) -> f64 {
                self.pass as f64 / self.total as f64
            }
            fn new(pass: i32, total: i32) -> Self {
                Ratio { pass, total }
            }
            fn get_increased(self) -> Self {
                Ratio::new(self.pass + 1, self.total + 1)
            }
        }

        let mut priority_queue: BinaryHeap<Ratio> = classes
            .iter()
            .map(|class| Ratio::new(class[0], class[1]))
            .collect();

        for _ in 0..extra_students {
            let max_increment = priority_queue.pop().unwrap();
            priority_queue.push(max_increment.get_increased());
        }

        let sum_ratio = priority_queue
            .iter()
            .fold(0.0, |acm, ratio| acm + ratio.get_ratio());

        sum_ratio / classes.len() as f64
    }

    #[allow(dead_code)]
    pub fn test() {
        let classes = vec![vec![1, 2], vec![3, 5], vec![2, 2]];
        let extra_students = 2;
        println!(
            "max_average_ratio => {}",
            Self::max_average_ratio(classes, extra_students)
        );
        println!()
    }
}
