pub struct BubbleSort {}

impl BubbleSort {
    pub fn tick(data: &mut Vec<u32>) {
        let temp = data[1];
        data[2] = data[1];
        data[1] = temp;

        println!("{:?}", data);
        // data[5] += 1;
        // data[2] += 8;
        // data[1] += 22;
    }
}
