pub struct BubbleSort {}

impl BubbleSort {
    pub fn new() -> BubbleSort {
        BubbleSort {}
    }
    pub fn solve(&mut self, data: &Vec<u32>) -> Vec<Vec<u32>> {
        let mut result: Vec<Vec<u32>> = vec![data.clone()];
        let mut copy = data.clone();
        for _ in 0..data.len() - 1 {
            for i in 0..data.len() - 1 {
                if copy[i] > copy[i + 1] {
                    let temp = copy[i];
                    copy[i] = copy[i + 1];
                    copy[i + 1] = temp;
                    result.push(copy.clone());
                }
            }
        }
        result
    }
}
