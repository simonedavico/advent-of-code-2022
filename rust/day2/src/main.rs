enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn play_round(opponent_move: Shape, my_move: Shape) -> i32 {
    let outcome = match (opponent_move, &my_move) {
        (Shape::Rock, Shape::Paper) => Outcome::Win,
        (Shape::Rock, Shape::Scissors) => Outcome::Loss,
        (Shape::Scissors, Shape::Rock) => Outcome::Win,
        (Shape::Scissors, Shape::Paper) => Outcome::Loss,
        (Shape::Paper, Shape::Scissors) => Outcome::Win,
        (Shape::Paper, Shape::Rock) => Outcome::Loss,
        (_, _) => Outcome::Draw,
    };

    outcome as i32 + my_move as i32
}

fn parse_opponent_move(opponent_move: &str) -> Shape {
    match opponent_move {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        mv => panic!("Unrecognized opponent move! ({mv})"),
    }
}

fn parse_my_move(opponent_move: &str) -> Shape {
    match opponent_move {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        mv => panic!("Unrecognized my move! ({mv})"),
    }
}

// part one
fn compute_score_for_moves() -> i32 {
    STRATEGY
        .lines()
        .map(|line| {
            let (opp, my) = line.split_once(" ").unwrap();
            play_round(parse_opponent_move(opp), parse_my_move(my))
        })
        .sum()
}

// part two
fn parse_outcome(outcome: &str) -> Outcome {
    match outcome {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        o => panic!("Unrecognized outcome! ({o})"),
    }
}

fn compute_score_from_outcome(opponent_move: Shape, outcome: Outcome) -> i32 {
    let my_move = match (opponent_move, &outcome) {
        (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Rock, Outcome::Loss) => Shape::Scissors,
        (Shape::Scissors, Outcome::Win) => Shape::Rock,
        (Shape::Scissors, Outcome::Loss) => Shape::Paper,
        (Shape::Paper, Outcome::Win) => Shape::Scissors,
        (Shape::Paper, Outcome::Loss) => Shape::Rock,
        (mv, Outcome::Draw) => mv,
    };

    outcome as i32 + my_move as i32
}

fn compute_score_for_move_and_outcome() -> i32 {
    STRATEGY
        .lines()
        .map(|line| {
            let (opp, outc) = line.split_once(" ").unwrap();
            compute_score_from_outcome(parse_opponent_move(opp), parse_outcome(outc))
        })
        .sum()
}

fn main() {
    println!("Score for moves: {}", compute_score_for_moves());
    println!(
        "Score for move and outcome: {}",
        compute_score_for_move_and_outcome()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_score_for_strategy() {
        assert_eq!(compute_score_for_moves(), 10595);
    }

    #[test]
    fn computes_score_for_move_and_outcome() {
        assert_eq!(compute_score_for_move_and_outcome(), 9541);
    }
}

const STRATEGY: &str = "A Y
A Z
A X
B X
A Y
B Y
B Y
A X
A Z
A X
A X
A X
B X
B X
B X
B X
C Z
B Z
B Y
B X
A X
A Y
B X
B X
B X
B Z
B X
B X
B X
B Z
B Z
A X
A X
C X
B X
B X
B X
A X
B X
B X
A Z
B X
B X
B X
B Z
B X
A X
B X
B Z
B X
B X
B Z
A Z
B X
B X
B X
A X
C Z
A X
A X
C Y
A X
B X
B X
B X
A X
B X
B X
A X
C X
B Y
A Z
A Y
B Z
B Z
C X
B Y
A X
B Y
A Y
B X
C X
B X
B X
C X
B X
A X
B X
B X
A Y
A X
B X
B Z
A Z
B Z
B Y
A Y
B Z
B X
B X
B X
A Z
B X
B X
A X
A Y
B X
B Z
B X
B X
B Y
B X
C X
B Z
C X
B Z
B X
A Y
B X
A Z
C Y
B Z
B Y
B X
A X
A Y
A X
B Z
B Z
A Z
B X
B Z
B X
A Y
B Z
B X
B Z
C X
B X
B X
B X
B Z
B Y
C Z
A X
B Y
B X
B X
B Z
B X
B Z
B X
A X
B Z
B Z
B Z
B Y
B X
C Z
B X
B Y
A X
B X
B Z
A X
B X
B X
B Z
B X
B Z
B Y
B Y
B Z
A X
B X
B Y
A Y
A Y
C X
B Z
B X
B X
A Y
C Y
B X
A Z
A Y
B Z
B X
B X
B Z
A Y
B X
B Z
A Y
B Z
B X
B X
A X
B Z
A X
B X
B Z
A X
B X
A X
A X
B X
A Y
B Z
B X
B Y
B X
B X
A Y
A X
C X
A Z
B Y
B Z
C X
B X
B Z
B Z
B Z
B X
B X
C X
C X
C Y
A Z
B X
A X
A Z
A X
B Y
A Z
B Z
B X
B Y
B X
C Z
A Y
B X
A X
B Z
B Z
A Z
A X
A X
B X
B Y
B Y
B Z
B X
B Z
A X
B X
B X
A X
A Y
A Z
B Z
B X
B Z
B X
A X
C Z
B X
A Z
B X
B X
B Z
B X
B Z
B X
B X
A Z
B Z
B X
C X
B X
A X
B X
C X
B X
B X
B X
B X
B X
A X
A Z
A X
B Z
C Z
B X
A Z
C Y
B Z
B Z
B X
A Z
B X
A Z
B Y
B X
A X
B X
B X
B X
A Z
B X
C X
B Y
C X
B Y
B X
B X
B Z
A X
B X
A Y
A Y
A Y
A Y
A Y
B X
A X
B X
A Z
A Z
B Y
A Z
A Y
A X
B X
A Z
A X
A X
B Z
B Z
A Z
B Z
B X
B X
A X
B X
B X
B X
A X
B X
C Z
A Z
A X
A X
B X
B X
B X
B X
A X
A X
A X
B Y
B Y
B X
A X
A Y
A Y
B X
A X
B X
A X
A Z
B Z
B X
B X
A Y
C X
C X
A Y
B X
B Z
B X
B X
A Z
C X
C X
B X
B X
C X
B X
B X
B X
B Y
B X
A Y
B X
A Y
B Z
B X
B X
B X
B X
A X
B X
B X
B X
B X
B Y
B Y
B Z
A Y
B X
B Z
A X
B X
A X
A Y
B Y
B X
B X
C Z
B X
A Z
C Y
B Z
C Z
A X
B X
A X
A Z
A X
A X
A Y
B X
B X
B Y
A X
B Z
B Z
B X
B Y
C X
B X
B X
B X
A X
C X
A Y
B Z
B X
B Z
B Y
C Y
A X
C Z
A X
B X
A Y
B Z
C X
B Z
B X
A X
A Z
B X
C Y
A X
B X
B X
B Z
B Z
B X
A X
A X
B X
B Z
A X
A Z
A Z
A Y
A Z
B X
C Z
A Y
B X
B Z
B X
B X
B X
B Z
A X
B X
A X
B Y
B Y
B X
C Z
A X
A X
A X
B X
B X
B X
B X
A Z
B X
B X
B Y
A X
B Z
C X
A Y
B X
A Y
A Y
B X
C X
C X
B Z
B Z
B Z
B X
B Y
A Y
B Z
B X
C Z
B Z
A Z
A X
A Z
A X
B X
B Z
B X
B X
A Z
B Z
C Y
B Z
B X
B X
A X
A X
C X
B X
C Z
B Z
B X
A Y
B Z
B X
B X
A Z
A X
B X
A X
A X
A X
A Z
C Y
B X
B X
A X
B X
B Y
B Y
B X
B X
B X
C Z
B X
B X
C Z
B Z
A X
C Y
A X
A X
A X
A X
B X
A X
C Y
B Z
A X
B X
B Y
C X
B Z
A X
B X
B Y
B X
C Y
A X
B Z
B Y
B X
B X
B Z
B X
B Y
B Z
B X
C X
B Y
B X
B Z
B Z
B Z
B X
B X
B X
B Y
A Y
B X
B X
B X
B Z
B Y
C X
C X
C Z
B X
A X
B X
B Z
B Z
B Z
A Z
A Y
B X
A X
A Z
B Y
A X
B X
A Y
A X
B X
B Z
A X
A X
A Z
B X
B X
A X
B Y
B X
B Z
A Y
B X
A X
B X
B X
B X
A Y
B X
B X
B Z
C X
C Y
B X
A X
B Y
B X
B X
B Y
B X
B Z
B Y
C X
B Y
B X
C Y
A Z
C X
B X
B Z
A X
B X
B X
B X
B X
A Y
C Z
B X
B X
B Z
A Y
B X
B Y
A X
C X
B X
A Y
A X
B X
B X
B X
B Y
A X
C X
B X
A X
B X
B Z
C Z
A X
A X
B X
A X
C X
A X
B Z
B X
B X
B X
A Y
A Z
B X
C X
B Y
B Z
A Z
A Y
B Y
A Y
B X
B X
C X
B X
A X
B X
B Z
A Y
B Z
A X
B X
A Y
B X
B X
B X
B Z
B X
B X
A X
B X
B X
B X
B X
C X
A Y
A Z
B Z
A X
A Z
A X
B Z
B Z
B X
B X
B X
A X
B X
B Y
A X
B X
C X
A Z
B X
B X
C Z
B X
B Z
A Y
A X
A Y
A X
B X
B Y
B Z
B X
B X
B X
B Z
B Z
A X
A X
B X
B Z
A X
A Y
A Y
A X
B X
A Z
B X
A Y
B Z
A X
C X
B X
B X
A X
B X
C X
B Z
B X
B X
C X
A Y
B X
B X
B Z
B X
A Y
B Z
C Z
B Y
B X
A Y
B X
B X
B X
B X
C X
A Y
B X
B Z
B X
B X
C Z
A X
B X
A Y
A X
B X
B X
A X
A Z
C X
A X
A Y
A Y
B Z
A X
B Z
B X
B Y
A Z
A X
A X
A Y
C Z
B X
A Y
B Z
B X
B X
B Z
B X
A Z
C Z
B Z
A Z
B X
A X
B Z
A X
B X
B Z
B X
A X
B Y
A Z
A Z
B X
B Z
A Y
B Z
A Y
B X
C X
A Y
B Z
A X
B X
B X
C X
B Y
B Z
B Z
B X
A X
A X
B X
A X
C Y
A Y
A X
A Z
A X
A X
B X
B X
B X
B Y
C Y
A X
B X
B X
B X
B X
B X
C Y
A Z
B X
A X
A X
A Z
A Y
C X
A Z
B X
B X
B X
A X
B Z
B X
C Z
A Y
B X
A Y
B Y
B X
A Y
B Z
B X
B Z
B X
A Y
B Y
B Y
A X
B X
C Z
B Z
B Z
B X
A Z
C X
B Z
C Y
B X
A Y
A X
B X
A X
B Z
A Y
B Y
B Y
B X
C X
C X
A Y
A Z
B X
B X
B X
B X
C X
B X
C X
B X
B Z
A X
A X
B X
A Y
B Z
B X
A X
B X
B X
A Z
C X
B Z
C X
A X
B Z
C X
B X
B Z
A X
C X
A Y
A X
A X
A Y
B Y
B X
B X
B X
B Y
B X
A Y
B X
A X
B X
C X
B X
B X
B X
B X
B X
A X
C X
C Z
B X
B X
B Z
B Z
A X
B Y
B X
B X
A Y
B X
B Z
A X
A X
C X
B X
B X
C X
B X
B X
B X
A X
A X
C X
B X
B X
B Z
B Z
A Z
B X
A X
A X
B X
B Y
A X
B X
B X
A X
B X
B X
B X
B X
A Y
A X
B X
A X
B X
A Z
A Y
B X
B X
A Z
B Z
B X
B X
B X
B X
A X
B X
B X
B X
A Y
C Y
A X
B Z
A X
B X
B X
B X
B Y
C X
A X
B Z
B X
C X
A X
A X
A Y
B Z
B X
B X
B Z
C Z
B X
B Z
B X
A X
A Y
A Y
B Z
B X
C Y
B X
B Z
A Y
B Z
A Y
B Y
B X
C Z
B X
A X
A Z
B Z
C X
A Y
C X
A Y
B Y
B X
C X
A X
A Y
A Y
B X
A Z
A Z
A Z
B X
A Z
B Z
A X
B X
A Y
A Z
A X
B X
A Y
B X
B X
B Z
A X
B X
B Y
C Y
B Y
B X
C Y
A X
C Y
A Z
A Y
B Z
B X
A X
C X
B X
C Z
C Y
B Z
B X
C X
B Z
B X
C X
B Z
B Z
C X
B X
B Z
A X
A Y
C Y
B X
A Y
A Y
A Z
B X
B Z
A Y
B Y
A Z
B X
B Y
A Y
B Y
A X
A Y
B Z
A Y
B Z
B Z
B X
B X
C Z
A Y
A Y
B Y
B Z
B Z
A X
B X
A Y
C Z
B X
B Z
B X
B Z
B X
C X
A X
B Z
A X
B Z
C X
B X
A X
A X
A Y
B X
B Z
B X
A X
B Z
B Z
B Y
B Z
C Y
B X
B X
B X
A X
B X
C Z
B X
C Z
A X
B Y
B X
C Y
C X
B X
B X
B Z
B Z
B X
B X
B Z
B X
B Y
A X
B X
A X
C Z
B X
B Z
B X
B X
B X
A X
A X
A Y
B Z
C X
B X
C Y
A Y
B Y
B Y
A X
B Z
B X
A Y
B Z
B Z
A X
B Z
B Y
C X
C X
B X
B X
B Y
B Y
B Y
A X
B X
B X
A Y
B Z
A X
A Z
A Y
A Z
B Z
B Z
A Y
A X
A X
B X
B Z
A Y
B X
A Y
B X
B X
A Y
B X
B X
B Z
C X
A Y
B X
B X
C Y
B Y
B X
B X
B X
A X
B X
A Z
B X
A Z
B Z
A Y
B X
C X
B X
A X
A Z
B Z
B Z
B X
B X
C X
B X
B Z
B X
B Z
B X
B X
A Y
B X
B Y
B X
B X
B Y
B X
B Z
B X
B Z
A X
A Y
B Y
A X
B X
A X
B X
A X
A Y
B Y
B Y
B X
B X
B Z
A X
B X
A X
A X
A Y
B X
A Y
A X
A Y
B Y
B Z
B X
A X
B X
B X
A Y
A Z
B X
A Z
B X
B X
C Z
B Z
B Z
B Y
A Z
B Z
A Z
B Y
B X
C Z
A Z
B Z
A X
B X
B X
B X
C Y
C Z
B Z
C Z
B X
B X
B X
B X
B X
A X
A Z
B X
A X
B X
C Z
B Z
B X
A Z
A X
B X
A X
B X
A Y
B X
A X
A Y
C X
B X
C X
A Z
C Z
B X
C X
B Z
A X
B X
B X
B Z
A Y
A Z
B Z
B X
A X
C Y
B Z
B Z
B X
B X
A X
B X
A Y
B X
B Y
A Z
B X
B X
A Z
A X
B Y
B Z
B X
B X
C X
B Z
A X
B X
A Z
B X
B X
B X
B X
B X
B X
B Z
A X
B X
B X
B X
B X
B Z
B Z
A Y
B X
C Y
B Z
B Z
A Z
B Z
C Y
B X
B Y
A X
A X
A Z
B X
B Y
C X
B Z
B X
B X
B X
B Z
B X
B X
C Y
C X
A X
B X
B Y
B X
B X
A Y
A X
B X
A X
B X
A Z
B X
C Y
C Y
A Y
B Z
C Y
C Y
C X
C Y
C Z
C X
B X
B X
B X
B Y
B X
A Y
C Y
B X
B X
A Z
B X
B X
B X
C Z
B Y
A Z
C Z
B X
B Z
A X
B X
B Z
B X
B X
B Z
A Y
B Z
A X
A X
B X
B X
B X
B X
A Y
A Z
B X
B Z
B Z
B Z
B X
B X
B Y
A Y
B X
C Y
B Z
B Y
B Z
B X
C Y
B Z
A X
B X
A X
B Z
B Y
B X
B Z
C Y
A Y
B X
C Y
C Y
A X
A X
A X
A X
B X
B X
A Y
A Z
B Y
B Z
B X
B X
B X
B Y
B Y
A X
B Y
B X
B Z
B X
B Z
B X
A X
B X
B X
B Y
B X
B Z
B Z
B X
B X
B X
B X
B Z
B X
B Z
A Z
B X
B X
B X
B Y
A X
B Z
A X
C Y
B Y
B Z
C X
C X
B Z
B Z
C X
A X
B X
A X
B X
B X
A Z
B X
A Y
A X
A X
B Y
A Y
A Y
C X
A X
B X
B Y
B Y
A Y
A X
C Y
A Y
B Z
B X
B Y
B Z
A Y
A Y
B Z
A X
C X
A Z
B X
B X
B X
B X
A X
A Y
B X
B X
B Z
A Y
A Y
C Z
B Z
B Y
A Y
A X
C X
A Y
B X
C X
A X
A Z
B X
A X
B X
B Z
C X
B X
A Y
B X
B Z
B X
B X
B X
C X
B X
C Y
B Z
B Z
B X
B X
B Z
C X
B X
B X
A Z
B X
A Y
B X
B Y
B Z
A X
A Z
A X
B X
B Z
B Z
B X
A Y
B Z
B X
C Y
A X
A Y
A Y
B Z
B X
A X
A Y
C X
A Y
A X
B X
B X
A X
C X
B Z
A X
B Y
B X
A X
C Z
C X
B Z
A X
B X
B Y
B X
B X
A Y
B Y
A X
A X
A Z
B X
A Z
A X
B X
A X
B Z
A X
A X
A X
B X
A Y
B X
B Z
C Z
B X
A X
B X
C Z
B X
B Z
A X
C X
B X
B X
A X
B X
B Z
A X
C X
B Z
B X
B X
B X
B X
B X
A Y
C Y
B Z
C Z
B X
A X
A X
A Y
B X
B X
C Y
B X
C Z
A X
B X
A Y
B X
B X
A X
C X
B X
C Y
B X
C Y
A X
B X
B Y
C Y
B Z
B X
A X
B X
B X
B Y
A X
B X
A Z
A Y
A X
A Z
B X
A Y
C Y
B Z
C Y
B X
B X
A Y
C X
C Z
B Z
B X
B X
A Y
B X
B Y
A X
A X
B X
B Z
A Y
A Y
A Y
B X
B X
C X
B Y
B X
A Y
A X
C X
B X
B Z
A Z
B Z
A Y
B Z
A X
B X
B X
B X
B Y
C Z
B Y
B X
A X
B X
A X
A Z
B Z
B X
B X
B X
C X
C Z
B Z
A X
B X
B X
A Y
B X
B Z
A X
B X
A X
A X
C Y
B Z
A X
A X
B Y
B X
A Y
B Y
C X
B Z
A X
A Z
B X
B X
A X
B X
A Y
A X
C X
B X
A X
C X
B Z
B X
A X
A Z
B X
B X
B Z
B X
B Z
C X
B Z
A X
A X
B X
B X
B Y
B X
B X
B Z
A Y
A X
C X
A X
B X
A Z
B X
A X
A X
B X
A X
B X
B X
A Y
A Y
A X
B X
B Z
C X
A X
B X
B X
B Z
B X
B X
A Z
B Z
B Z
B X
B Y
C X
B Z
A X
B X
B X
B Z
A Y
B X
B X
B X
B X
B X
C X
C Y
A Z
B X
C Y
A Y
C X
B X
B X
B Z
B X
B X
A X
A X
A Z
C Y
B X
B X
A Y
B Z
B Z
A X
A Z
C X
B X
B X
B X
B X
A X
C X
B Z
B X
A X
A X
A Y
B X
B X
A Z
C X
B X
A X
B Z
A X
B Y
B X
B X
A Y
C Y
B Z
A X
A Z
A Y
C Y
B X
C X
B X
B Y
B X
B Z
A X
C Y
A Y
B Z
B X
A X
C Z
B X
B Z
A X
C Y
B Y
B X
B X
A Y
B X
B Z
B X
B Z
B X
B Z
A Y
A X
B X
B X
A Y
C X
A X
C X
B X
B X
B X
A X
B Z
B Z
C Z
B Z
A X
A X
B Z
C X
A Z
A Y
B Y
B X
A X
A Y
A X
A Z
B X
B X
B Z
B Z
A Y
B X
B X
B Z
B X
A X
C X
B X
B X
B Y
A X
B X
B X
C Z
B X
B Z
B X
B Y
B X
B X
A X
A Y
A Y
B X
A X
A Z
B Z
B Z
B X
B X
B Z
B X
A Z
B X
A X
C X
B X
B X
B Z
B X
A X
B Z
B X
B Z
C Y
B X
B X
B X
B Z
C X
B X
B X
B Y
B Z
B X
C Y
A Y
C X
A X
B X
A X
B Z
B X
B X
A Y
C Z
A X
B Y
A X
B Z
B X
C Z
B X
C Z
A X
B X
A X
B X
A Y
B X
B Z
B X
B Z
C Z
B X
B X
A Z
A X
A X
B X
A X
B X
B X
B X
B X
A X
B Z
B X
A X
B Z
C Y
C Z
A X
B X
B X
B Z
B Z
A X
A Y
B X
A Y
B X
B X
C X
B Z
B Y
B Z
B Z
A Y
C X
B Y
B X
B X
A Z
B X
B Y
C X
B Y
C Y
B X
B X
C Y
A Z
C X
B X
C X
A Y
B X
C Z
B Z
B X
B Y
A Y
A X
B X
C Y
C X
A X
B X
B X
A X
B X
B X
C X
C Y
A Z
C X
A X
C X
B X
A Y
B X
A Y
C X
B X
C Z
C X
B Z
B X
B X
A X
A X
A Y
B Y
B Y
A Y
C Z
A Y
A X
B Z
B X
B X
C X
B X
C X
A Y
B Z
B X
A Y
A Y
C X
A Y
B Z
B Z
A X
B X
A Y
B Y
B X
A Z
B X
B X
B Z
C X
B X
C X
B X
B X
C X
A X
A Y
B X
A Y
C Z
C X
A Z
B X
B X
B X
A X
B X
B Y
B X
B Z
B X
A Y
B X
B X
B X
B X
B Y
B Y
B Z
C X
A Y
C X
B Y
B X
B Y
B X
A X
B Z
B Z
B X
A Z
C X
A X
B Z
B Y
A Y
B X
B X
B X
A X
B X
A Y
A X
B X
B Z
B X
A X
A Y
B X
B X
A X
B Z
A Z
A X
A Y
C X
A Z
B X
B X
A X
B Z
B Z
A X
A Y
B X
B Z
B X
A Y
B Y
A Y
B X
B X
A X
B X
A X
B X
C Z
A X
B X
C Z
A X
A X
B X
A X
A X
C Z
B Z
B X
B Z
A X
B Z
B X
B X
B X
A Z
B X
B X
B Z
A X
A Y
B X
A X
C X
A Y
A X
B X
B X
B Y
A Y
B X
A Y
A Z
B Z
B Z
B Z
B X
B X
A Y
A Z
B X
B X
B X
B X
B X
A Z
A Z
B X
B X
B X
B X
A X
B X
A X
B X
B Z
C X
A X
B X
C Z
B X
B X
B X
A X
A Z
B Z
C X
A X
B X
C X
B X
A X
C Z
A Z
A Y
B X
C X
B X
C X
B X
B Y
B Z
A X
B X
B X
A X
A Z
B Z";