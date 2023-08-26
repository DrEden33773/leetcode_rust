crate::sln!();

impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        grid.iter_mut().for_each(|vec| {
            vec.sort();
            vec.reverse();
        });
        let mut sum = 0;
        let n = grid.first().unwrap().len();
        for i in 0..n {
            sum += grid.iter().map(|v| v[i]).max().unwrap();
        }
        sum
    }
}
