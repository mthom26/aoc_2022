use std::fs;

fn main() {
    let input: Vec<char> = fs::read_to_string("./input.txt").unwrap().chars().collect();

    let mut unique_idx = 0;
    let mut unique_idx_14 = 0;

    for (i, w) in input.windows(4).enumerate() {
        if w[0] == w[1] || w[0] == w[2] || w[0] == w[3] {
            continue;
        }
        if w[1] == w[2] || w[1] == w[3] {
            continue;
        }
        if w[2] == w[3] {
            continue;
        }
        unique_idx = i + 4;
        println!("{:?}", w);
        break;
    }

    // This one is probably quite inefficient
    'outer: for (i, w) in input.windows(14).enumerate() {
        let mut w: Vec<char> = w.iter().map(|c| *c).collect();
        w.sort();

        let mut prev: char = '1';

        for c in w.iter() {
            if *c == prev {
                continue 'outer;
            }
            prev = *c;
        }

        unique_idx_14 = i + 14;
        println!("{:?}", w);
        break;
    }

    println!("Part 1: {}", unique_idx);
    println!("Part 2: {}", unique_idx_14);
}
