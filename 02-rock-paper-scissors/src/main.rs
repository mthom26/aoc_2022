use std::fs;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

fn shape_score(shape: &Shape) -> usize {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn game_score(opponent: &Shape, player: &Shape) -> usize {
    match player {
        Shape::Rock => match opponent {
            Shape::Rock => 3,
            Shape::Paper => 0,
            Shape::Scissors => 6,
        },
        Shape::Paper => match opponent {
            Shape::Rock => 6,
            Shape::Paper => 3,
            Shape::Scissors => 0,
        },
        Shape::Scissors => match opponent {
            Shape::Rock => 0,
            Shape::Paper => 6,
            Shape::Scissors => 3,
        },
    }
}

fn outcome_score(outcome: &Outcome) -> usize {
    match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Loss => 0,
    }
}

fn outcome_player_score(opponent: &Shape, desired_outcome: &Outcome) -> usize {
    match opponent {
        Shape::Rock => match desired_outcome {
            Outcome::Win => shape_score(&Shape::Paper),
            Outcome::Draw => shape_score(&Shape::Rock),
            Outcome::Loss => shape_score(&Shape::Scissors),
        },
        Shape::Paper => match desired_outcome {
            Outcome::Win => shape_score(&Shape::Scissors),
            Outcome::Draw => shape_score(&Shape::Paper),
            Outcome::Loss => shape_score(&Shape::Rock),
        },
        Shape::Scissors => match desired_outcome {
            Outcome::Win => shape_score(&Shape::Rock),
            Outcome::Draw => shape_score(&Shape::Scissors),
            Outcome::Loss => shape_score(&Shape::Paper),
        },
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut score_1 = 0;
    let mut score_2 = 0;

    for line in input.lines() {
        let chars: Vec<&str> = line.split(" ").collect();

        let opponent = match chars[0] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!(),
        };
        let (player, outcome) = match chars[1] {
            "X" => (Shape::Rock, Outcome::Loss),
            "Y" => (Shape::Paper, Outcome::Draw),
            "Z" => (Shape::Scissors, Outcome::Win),
            _ => panic!(),
        };

        score_1 += shape_score(&player);
        score_1 += game_score(&opponent, &player);

        score_2 += outcome_score(&outcome);
        score_2 += outcome_player_score(&opponent, &outcome);
    }

    println!("Total Score Part 1: {}", score_1);
    println!("Total Score Part 2: {}", score_2);
}
