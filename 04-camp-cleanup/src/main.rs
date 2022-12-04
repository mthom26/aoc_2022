use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut overlapping_pairs = 0;
    let mut partial_overlapping = 0;

    for line in input.lines() {
        let (a, b) = {
            let split: Vec<&str> = line.split(",").collect();
            (split[0], split[1])
        };
        let (a, b) = (get_min_max(a), get_min_max(b));

        if is_overlapping(a, b) {
            overlapping_pairs += 1;
        }
        if is_partial_overlapping(a, b) {
            partial_overlapping += 1;
        }
    }

    println!("Overlapping pairs: {}", overlapping_pairs);
    println!("Partial Overlapping pairs: {}", partial_overlapping);
}

fn get_min_max(input: &str) -> (usize, usize) {
    let split: Vec<&str> = input.split("-").collect();
    (
        split[0].parse::<usize>().unwrap(),
        split[1].parse::<usize>().unwrap(),
    )
}

fn is_overlapping(a: (usize, usize), b: (usize, usize)) -> bool {
    if (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1) {
        return true;
    }
    false
}

fn is_partial_overlapping(a: (usize, usize), b: (usize, usize)) -> bool {
    if (a.1 < b.0) || (b.1 < a.0) {
        return false;
    }
    true
}
