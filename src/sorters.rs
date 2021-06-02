pub struct Sorters {}

impl Sorters {
    pub fn bubble_sort(data: &Vec<u32>) -> Vec<Vec<u32>> {
        let mut result: Vec<Vec<u32>> = vec![data.clone()];
        let mut copy = data.clone();
        for _ in 0..copy.len() - 1 {
            for i in 0..copy.len() - 1 {
                if copy[i] > copy[i + 1] {
                    let temp = copy[i];
                    copy[i] = copy[i + 1];
                    copy[i + 1] = temp;
                    record_slide(&mut result, &copy);
                }
            }
        }
        result
    }
}

fn record_slide(data: &mut Vec<Vec<u32>>, new_line: &Vec<u32>) {
    data.push(new_line.clone());
}
