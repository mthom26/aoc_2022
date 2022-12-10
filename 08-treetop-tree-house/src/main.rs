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

    for row in 1..height - 1 {
        for col in 1..width - 1 {
            let vis = check_visibility(row, col, &grid);
            if vis {
                visible += 1;
            }
            // println!("---");
            // println!("{} {} {}", grid[row-1][col-1], grid[row-1][col], grid[row-1][col+1]);
            // println!("{} {} {}", grid[row][col-1], grid[row][col], grid[row][col+1]);
            // println!("{} {} {}", grid[row+1][col-1], grid[row+1][col], grid[row+1][col+1]);
            // println!("row {} - col {}: vis {}", row, col, vis);
            // break;
        }
        // break;
    }

    println!("Part 1: {}", visible);
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
