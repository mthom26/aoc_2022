use std::fs;

struct CRT {
    pixels: [char; 240],
}

impl CRT {
    fn draw(&mut self, tick: i32, x: i32) {
        // Need adjusted tick or only the first line will render properly
        let adjusted_tick = adjusted_tick(tick);
        if [x, x + 1, x + 2].contains(&adjusted_tick) {
            self.pixels[(tick - 1) as usize] = '#';
        } else {
            self.pixels[(tick - 1) as usize] = '.';
        }
    }

    fn render(&self) {
        for (i, p) in self.pixels.iter().enumerate() {
            print!("{}", p);
            if [40, 80, 120, 160, 200, 240].contains(&(i + 1)) {
                print!("\n");
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut tick: i32 = 1;
    let mut x: i32 = 1;
    let mut total_signal = 0;

    let mut crt = CRT { pixels: [' '; 240] };

    for line in input.lines() {
        let ins: Vec<&str> = line.split(" ").collect();
        crt.draw(tick, x);
        if ins[0] == "noop" {
            tick += 1;
        } else {
            let num: i32 = ins[1].parse().unwrap();
            tick += 1;
            if check_cycle(tick) {
                total_signal += tick * x;
            }
            crt.draw(tick, x);
            tick += 1;
            x += num;
        }
        if check_cycle(tick) {
            total_signal += tick * x;
        }
    }

    println!("Part 1: {}", total_signal);
    println!();
    crt.render();
}

fn check_cycle(tick: i32) -> bool {
    [20, 60, 100, 140, 180, 220].contains(&tick)
}

fn adjusted_tick(tick: i32) -> i32 {
    if tick > 200 {
        tick - 200
    } else if tick > 160 {
        tick - 160
    } else if tick > 120 {
        tick - 120
    } else if tick > 80 {
        tick - 80
    } else if tick > 40 {
        tick - 40
    } else {
        tick
    }
    // tick % 40
}
