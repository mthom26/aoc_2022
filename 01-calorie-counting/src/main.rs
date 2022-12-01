use std::fs;

struct TopThree {
    nums: [usize; 3],
}

impl TopThree {
    fn new() -> Self {
        TopThree { nums: [0, 0, 0] }
    }

    // Return the lowest number in the array (and it's index)
    fn get_lowest(&self) -> (usize, usize) {
        let (index, num) = self
            .nums
            .iter()
            .enumerate()
            .min_by(|(_, x), (_, y)| x.cmp(y))
            .unwrap();
        (index, *num)
    }

    fn replace_at(&mut self, index: usize, new_num: usize) {
        self.nums[index] = new_num
    }
}

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();

    let mut acc = 0;

    let mut top_three = TopThree::new();

    for _ in contents.lines().map(|x| {
        if x.is_empty() {
            let (index, lowest) = top_three.get_lowest();
            if acc > lowest {
                top_three.replace_at(index, acc);
            }
            acc = 0;
        } else {
            acc += usize::from_str_radix(x, 10).unwrap();
        }
    }) {}

    let total: usize = top_three.nums.iter().sum();
    println!("{}", total);
}
