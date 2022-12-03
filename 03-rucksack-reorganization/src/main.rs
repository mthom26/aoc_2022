use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut total = 0;

    for line in input.lines() {
        let i = line.len() / 2; // Length should always be even
        'outer: for c_one in line[0..i].chars() {
            for c_two in line[i..line.len()].chars() {
                if c_one == c_two {
                    let priority = if c_one.is_lowercase() {
                        c_one as usize - 96
                    } else {
                        c_one as usize - 38
                    };

                    total += priority;
                    break 'outer;
                }
            }
        }
    }

    println!("Total priority: {}", total);
}
