use std::fs;

#[derive(Debug)]
struct Instruction {
    num: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn new(num: usize, from: usize, to: usize) -> Self {
        Self { num, from, to }
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let (mut stacks_1, mut stacks_2, instructions) = {
        let mut stack_strings = vec![];
        let mut instruction_strings = vec![];

        let mut current = &mut stack_strings;

        for line in input.lines() {
            if line.is_empty() {
                current = &mut instruction_strings;
                continue;
            }
            current.push(line);
        }

        // Create stacks
        let mut stacks = vec![];
        for _ in 0..9 {
            stacks.push(vec![]);
        }

        for line in stack_strings.iter().rev().skip(1) {
            for (i, c) in line.split("").skip(2).step_by(4).enumerate() {
                if c != " " {
                    stacks[i].push(c);
                }
            }
        }

        // Create instructions
        let instructions: Vec<Instruction> = instruction_strings
            .iter()
            .map(|line| {
                line.split(" ")
                    .into_iter()
                    .filter_map(|item| item.parse::<usize>().ok())
                    .collect::<Vec<usize>>()
            })
            .map(|ins| Instruction::new(ins[0], ins[1] - 1, ins[2] - 1))
            .collect();

        (stacks.clone(), stacks, instructions)
    };

    for ins in instructions {
        let mut temp_vec = vec![];
        for _ in 0..ins.num {
            // Part 1
            let item = stacks_1[ins.from].pop().unwrap();
            stacks_1[ins.to].push(item);

            // Part 2
            let item_2 = stacks_2[ins.from].pop().unwrap();
            temp_vec.push(item_2);
        }
        temp_vec.reverse();
        stacks_2[ins.to].append(&mut temp_vec);
    }

    println!("Top of stacks Part 1:");
    for stack in stacks_1.iter_mut() {
        print!("{}", stack.pop().unwrap());
    }
    println!();

    println!("Top of stacks Part 2:");
    for stack in stacks_2.iter_mut() {
        print!("{}", stack.pop().unwrap());
    }
    println!();
}
