use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut total = 0;
    let mut total_group = 0;

    // Part 1
    for line in input.lines() {
        let i = line.len() / 2; // Length should always be even
        'outer: for c_one in line[0..i].chars() {
            for c_two in line[i..line.len()].chars() {
                if c_one == c_two {
                    total += get_priority(c_one);
                    break 'outer;
                }
            }
        }
    }

    // Part 2
    let lines: Vec<&str> = input.lines().collect();

    for groups in lines.windows(3).step_by(3) {
        'outer: for a in groups[0].chars() {
            for b in groups[1].chars() {
                for c in groups[2].chars() {
                    if a == b && a == c {
                        total_group += get_priority(a);
                        break 'outer;
                    }
                }
            }
        }
    }

    println!("Total priority: {}", total);
    println!("Group priority: {}", total_group);
}

fn get_priority(c: char) -> usize {
    if c.is_lowercase() {
        c as usize - 96
    } else {
        c as usize - 38
    }
}
