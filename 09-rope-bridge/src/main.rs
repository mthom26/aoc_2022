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

    fn follow(&mut self, other: &Knot, map: &mut HashMap<(i32, i32), usize>) {
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
            self.x += move_x;
            self.y += move_y;
            map.insert((self.x, self.y), 0);
            // println!("follow: x {}, y {}", move_x, move_y);
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut head = Knot::default();
    let mut tail = Knot::default();
    let mut map: HashMap<(i32, i32), usize> = HashMap::new();
    map.insert((0, 0), 0);

    for line in input.lines() {
        let instruction: Vec<&str> = line.split(" ").collect();
        let i: usize = instruction[1].parse().unwrap();

        match instruction[0] {
            "U" => {
                for _ in 0..i {
                    head.up();
                    tail.follow(&head, &mut map);
                }
            }
            "D" => {
                for _ in 0..i {
                    head.down();
                    tail.follow(&head, &mut map);
                }
            }
            "R" => {
                for _ in 0..i {
                    head.right();
                    tail.follow(&head, &mut map);
                }
            }
            "L" => {
                for _ in 0..i {
                    head.left();
                    tail.follow(&head, &mut map);
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
}
