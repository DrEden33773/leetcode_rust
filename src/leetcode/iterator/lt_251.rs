struct Vector2D {
    index: usize,
    data: Vec<i32>,
}

impl Vector2D {
    fn new(vec: Vec<Vec<i32>>) -> Self {
        Self {
            index: 0,
            data: vec.into_iter().flatten().collect(),
        }
    }
    fn next(&mut self) -> i32 {
        if self.index < self.data.len() {
            let v = self.data[self.index];
            self.index += 1;
            v
        } else {
            self.data[self.data.len() - 1]
        }
    }
    fn has_next(&self) -> bool {
        self.index < self.data.len()
    }
}
