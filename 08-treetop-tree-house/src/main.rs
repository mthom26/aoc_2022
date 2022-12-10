use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut grid = vec![];

    for line in input.lines() {
        let new_row: Vec<usize> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        grid.push(new_row);
    }

    let width = grid[0].len();
    let height = grid.len();

    let mut visible = width * 2 + height * 2 - 4;
    let mut highest_score = 0;

    for row in 1..height - 1 {
        for col in 1..width - 1 {
            let vis = check_visibility(row, col, &grid);
            if vis {
                visible += 1;
            }
            
            let score = get_scenic_score(row, col, &grid);
            if score > highest_score {
                highest_score = score;
            }
            // println!("---");
            // println!("{} {} {}", grid[row-1][col-1], grid[row-1][col], grid[row-1][col+1]);
            // println!("{} {} {}", grid[row][col-1], grid[row][col], grid[row][col+1]);
            // println!("{} {} {}", grid[row+1][col-1], grid[row+1][col], grid[row+1][col+1]);
            // println!("row {} - col {}: vis {}", row, col, vis);
            // println!("score: {}", get_scenic_score(row, col, &grid));
            // break;
        }
        // break;
    }

    println!("Part 1: {}", visible);
    println!("Part 2: {}", highest_score);
}

fn get_scenic_score(row: usize, col: usize, grid: &Vec<Vec<usize>>) -> usize {
    let mut score_left = 0;
    let mut score_right = 0;
    let mut score_up = 0;
    let mut score_down = 0;

    // Left
    for i in (0..col).rev() {
        if grid[row][i] < grid[row][col] {
            score_left += 1;
        } else {
            score_left += 1;
            break;
        }
    }
    // Right
    for i in col + 1..grid[0].len() {
        if grid[row][i] < grid[row][col] {
            score_right += 1;
        } else {
            score_right += 1;
            break;
        }
    }
    // Up
    for i in (0..row).rev() {
        if grid[i][col] < grid[row][col] {
            score_up += 1;
        } else {
            score_up += 1;
            break;
        }
    }
    // Down
    for i in row + 1..grid.len() {
        if grid[i][col] < grid[row][col] {
            score_down += 1;
        } else {
            score_down += 1;
            break;
        }
    }

    // println!(
    //     "scores: L {}, R {}, D {}, U {}",
    //     score_left, score_right, score_down, score_up
    // );
    score_down * score_up * score_right * score_left
}

fn check_visibility(row: usize, col: usize, grid: &Vec<Vec<usize>>) -> bool {
    // println!("left: {}", left(row, col, grid));
    // println!("right: {}", right(row, col, grid));
    // println!("up: {}", up(row, col, grid));
    // println!("down: {}", down(row, col, grid));
    left(row, col, grid) || right(row, col, grid) || up(row, col, grid) || down(row, col, grid)
}

fn left(row: usize, col: usize, grid: &Vec<Vec<usize>>) -> bool {
    for i in 0..col {
        if grid[row][i] >= grid[row][col] {
            return false;
        }
    }
    true
}

fn right(row: usize, col: usize, grid: &Vec<Vec<usize>>) -> bool {
    for i in col + 1..grid[0].len() {
        if grid[row][i] >= grid[row][col] {
            return false;
        }
    }
    true
}

fn up(row: usize, col: usize, grid: &Vec<Vec<usize>>) -> bool {
    for i in 0..row {
        if grid[i][col] >= grid[row][col] {
            return false;
        }
    }
    true
}

fn down(row: usize, col: usize, grid: &Vec<Vec<usize>>) -> bool {
    for i in row + 1..grid.len() {
        if grid[i][col] >= grid[row][col] {
            return false;
        }
    }
    true
}
