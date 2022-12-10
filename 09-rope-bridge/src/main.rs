use std::{collections::HashMap, fs};

#[derive(Debug, Default)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn up(&mut self) {
        self.y += 1;
    }

    fn down(&mut self) {
        self.y -= 1;
    }

    fn right(&mut self) {
        self.x += 1;
    }

    fn left(&mut self) {
        self.x -= 1;
    }

    fn follow(&self, other: &Knot) -> Option<(i32, i32)> {
        let x = other.x - self.x;
        let y = other.y - self.y;

        if x.abs() > 1 || y.abs() > 1 {
            // Tail has to move
            let move_x = if x == 0 {
                0
            } else if x.is_positive() {
                1
            } else {
                -1
            };
            let move_y = if y == 0 {
                0
            } else if y.is_positive() {
                1
            } else {
                -1
            };
            Some((move_x, move_y))
            // println!("follow: x {}, y {}", move_x, move_y);
        } else {
            None
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut head = Knot::default();
    // let mut tail = Knot::default();
    let mut tails: Vec<Knot> = (0..9).into_iter().map(|_| Knot::default()).collect();

    let mut map: HashMap<(i32, i32), usize> = HashMap::new();
    let mut last_map: HashMap<(i32, i32), usize> = HashMap::new();
    map.insert((0, 0), 0);
    last_map.insert((0, 0), 0);

    for line in input.lines() {
        let instruction: Vec<&str> = line.split(" ").collect();
        let i: usize = instruction[1].parse().unwrap();

        match instruction[0] {
            "U" => {
                for _ in 0..i {
                    head.up();
                    run_follows(&head, &mut tails, &mut map, &mut last_map);
                }
            }
            "D" => {
                for _ in 0..i {
                    head.down();
                    run_follows(&head, &mut tails, &mut map, &mut last_map);
                }
            }
            "R" => {
                for _ in 0..i {
                    head.right();
                    run_follows(&head, &mut tails, &mut map, &mut last_map);
                }
            }
            "L" => {
                for _ in 0..i {
                    head.left();
                    run_follows(&head, &mut tails, &mut map, &mut last_map);
                }
            }
            _ => {}
        }
        // println!("{} --------", n);
        // println!("ins: {} {}", instruction[0], instruction[1]);
        // println!("Head: x {}, y {}", head.x, head.y);
        // println!("Tail: x {}, y {}", tail.x, tail.y);
    }

    println!("Part 1: {}", map.len());
    println!("Part 2: {}", last_map.len());
}

fn run_follows(
    head: &Knot,
    tails: &mut Vec<Knot>,
    map: &mut HashMap<(i32, i32), usize>,
    last_map: &mut HashMap<(i32, i32), usize>,
) {
    if let Some((x, y)) = tails[0].follow(&head) {
        tails[0].x += x;
        tails[0].y += y;
        map.insert((tails[0].x, tails[0].y), 0);
    }
    for i in 1..8 {
        if let Some((x, y)) = tails[i].follow(&tails[i - 1]) {
            tails[i].x += x;
            tails[i].y += y;
        }
    }
    if let Some((x, y)) = tails[8].follow(&tails[7]) {
        tails[8].x += x;
        tails[8].y += y;
        last_map.insert((tails[8].x, tails[8].y), 0);
    }
}
