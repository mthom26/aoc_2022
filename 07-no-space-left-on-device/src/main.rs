use std::{collections::HashMap, fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut path = vec!["root".to_owned(), "/".to_owned()];
    let mut totals: HashMap<String, usize> = HashMap::new();
    totals.insert("root/".to_owned(), 0);

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
                        let mut p = String::from_str(p).unwrap();
                        p.push_str("/");
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

    // Part 2
    let total_used = totals.get("root/").unwrap();
    let unused = 70000000 - total_used;
    let needed = 30000000 - unused;

    // Didn't need the dir here only the size but it was already done...
    let mut dir = "".to_owned();
    let mut dir_size = 0;
    for (p, n) in v.iter() {
        if **n >= needed {
            dir.push_str(p);
            dir_size = **n;
            break;
        }
    }

    println!("Part 1: {:?}", total);
    println!("Part 2: {:?} - {}", dir, dir_size);
}

fn vec_to_strings(path: &Vec<String>) -> Vec<String> {
    let mut s = "root/".to_owned();
    let mut v = vec![];
    for p in path.iter().skip(2) {
        v.push(s.clone());
        s.push_str(p);
    }
    v.push(s);
    v
}
