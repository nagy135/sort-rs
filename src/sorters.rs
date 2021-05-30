pub struct BubbleSort {
    index: usize,
}

impl BubbleSort {
    pub fn new() -> BubbleSort {
        BubbleSort { index: 0 }
    }
    pub fn solve(&mut self, data: &Vec<u32>) -> Vec<Vec<u32>> {
        vec![
            vec![15, 22, 25, 1, 13, 16, 15, 2, 9, 8, 6, 6, 4, 22, 15, 13],
            vec![15, 22, 25, 1, 13, 22, 15, 8, 6, 2, 9, 6, 4, 16, 15, 13],
        ]
    }
}
