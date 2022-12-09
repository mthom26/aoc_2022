use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut path = vec!["root"];
    let mut totals: HashMap<String, usize> = HashMap::new();
    totals.insert("root".to_owned(), 0);

    for line in input.lines().skip(2) {
        let parts: Vec<&str> = line.split(" ").collect();

        match parts[0] {
            "$" => match parts[1] {
                "cd" => match parts[2] {
                    ".." => {
                        path.pop();
                        // println!("{}", vec_to_str(&path));
                    }
                    p => {
                        path.push(p);
                        // println!("{}", vec_to_str(&path));
                    }
                },
                _ => { /* `ls` - Nothing to do */ }
            },
            "dir" => { /* Nothing to do */ }
            num => {
                let n = num.parse::<usize>().unwrap();
                let keys = vec_to_strings(&path);

                for k in keys {
                    match totals.get(&k) {
                        Some(val) => {
                            totals.insert(k, n + val);
                        }
                        None => {
                            totals.insert(k, n);
                        }
                    }
                }
            }
        }
    }

    let mut v: Vec<(&String, &usize)> = totals.iter().map(|pair| pair).collect();
    v.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    let total: usize = v
        .iter()
        .filter(|(_, n)| **n <= 100000)
        .map(|(_, n)| **n)
        .sum();

    println!("Part 1: {:?}", total);
}

fn vec_to_strings(path: &Vec<&str>) -> Vec<String> {
    let mut s = "root".to_owned();
    let mut v = vec![];
    for p in path.iter().skip(1) {
        v.push(s.clone());
        s.push_str(p);
    }
    v.push(s);
    v
}
