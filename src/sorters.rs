pub struct BubbleSort {
    index: usize,
}

impl BubbleSort {
    pub fn new() -> BubbleSort {
        BubbleSort { index: 0 }
    }
    pub fn tick(&mut self, data: &mut Vec<u32>) {
        let temp = data[self.index + 1];
        data[self.index + 1] = data[self.index];
        data[self.index] = temp;

        self.index += 1;
        self.index = self.index % 13;

        println!("{:?}", data);
        // data[5] += 1;
        // data[2] += 8;
        // data[1] += 22;
    }
}
