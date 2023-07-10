struct ZigzagIterator<T>
where
    T: Clone,
{
    pc: usize,
    data: Vec<T>,
}

impl<T> ZigzagIterator<T>
where
    T: Clone,
{
    fn new(v1: Vec<T>, v2: Vec<T>) -> Self {
        let mut p1 = 0;
        let mut p2 = 0;
        let mut data = vec![];
        while p1 < v1.len() && p2 < v2.len() {
            data.push(v1[p1].to_owned());
            data.push(v2[p2].to_owned());
            p1 += 1;
            p2 += 1;
        }
        while p1 < v1.len() {
            data.push(v1[p1].to_owned());
            p1 += 1;
        }
        while p2 < v2.len() {
            data.push(v2[p2].to_owned());
            p2 += 1;
        }
        Self { pc: 0, data }
    }

    fn next(&mut self) -> T {
        if self.pc < self.data.len() {
            let ret = self.data[self.pc].to_owned();
            self.pc += 1;
            ret
        } else {
            self.data.last().unwrap().to_owned()
        }
    }

    fn has_next(&self) -> bool {
        self.pc < self.data.len()
    }
}
