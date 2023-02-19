#[allow(dead_code)]
pub fn solution(num: usize) -> usize {
    fn update(curr: Box<Vec<usize>>) -> Box<Vec<usize>> {
        let new_len = curr.len() + 1;
        let mut new: Box<Vec<usize>> = Box::new(Vec::with_capacity(new_len));
        new.push(1);
        for index in 1..new_len - 1 {
            new.push(curr[index - 1] + curr[index]);
        }
        new.push(1);
        new
    }
    let mut curr: Box<Vec<usize>> = Box::new(vec![1]);
    let mut pos: usize = 1;
    let mut prev_len: usize = 0;
    loop {
        if pos > prev_len + curr.len() {
            prev_len += curr.len();
            curr = update(curr);
        }
        let pos_in_curr = pos - prev_len;
        if curr[pos_in_curr - 1] == num {
            break;
        }
        pos += 1;
    }
    pos
}

#[allow(dead_code)]
pub fn test() {
    println!("yang_hui_triangle: {}\n", solution(9));
}
