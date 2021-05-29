pub struct BubbleSort {
    index: usize,
}

impl BubbleSort {
    pub fn new() -> BubbleSort {
        BubbleSort { index: 0 }
    }
    pub fn tick(&mut self, data: &mut Vec<u32>) {
        println!("{}", self.index);

        let temp = data[self.index + 1];
        data[self.index + 1] = data[self.index];
        data[self.index] = temp;

        self.index += 1;
        if self.index >= data.len() - 1 {
            self.index = 0;
        }

        // println!("{:?}", data);
    }
}
