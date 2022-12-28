use std::{io, ops::IndexMut};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct Move {
    quantity: usize,
    from_stack: usize,
    to_stack: usize,
}

pub trait CrateMover {
    // TODO: this should return a Result<(), BadMoveError>
    fn interpret_move(&self, a_move: &Move, stacks: &mut Vec<Vec<&str>>) -> ();
}

#[derive(Debug)]
enum MoveCratesError {
    BadInput
}

struct CrateMover9000;
struct CrateMover9001;

impl CrateMover for CrateMover9000 {
    fn interpret_move(&self, a_move: &Move, stacks: &mut Vec<Vec<&str>>) -> () {
        let remove_from = stacks.get_mut(a_move.from_stack - 1).unwrap();
        let slice_start_index = remove_from.len() - a_move.quantity;
        let to_add = remove_from.drain(slice_start_index..).collect::<Vec<_>>();
        let add_to = stacks.get_mut(a_move.to_stack - 1).unwrap();

        for item in to_add.iter().rev() {
            add_to.push(item);
        }
    }
}

impl CrateMover for CrateMover9001 {
    fn interpret_move(&self, a_move: &Move, stacks: &mut Vec<Vec<&str>>) -> () {
        let remove_from = stacks.get_mut(a_move.from_stack - 1).unwrap();
        let slice_start_index = remove_from.len() - a_move.quantity;
        let to_add = remove_from.drain(slice_start_index..).collect::<Vec<_>>();
        let add_to = stacks.get_mut(a_move.to_stack - 1).unwrap();

        for item in to_add.iter() {
            add_to.push(item);
        }
    }
}

fn main() -> io::Result<()> {
    let result_9000 = move_crane(&CrateMover9000 {});
    let result_9001 = move_crane(&CrateMover9001 {});
    println!("{}", result_9000.unwrap());
    println!("{}", result_9001.unwrap());
    Ok(())
}

// see https://oswalt.dev/2021/06/polymorphism-in-rust/ for signature explanation
fn move_crane(crate_mover: &dyn CrateMover) -> Result<String, MoveCratesError> {
    if let Some((crates, moves)) = CRATES.split_once("\n\n") {
        let mut stacks = parse_stacks(crates);
        let moves = parse_moves(moves).unwrap();

        moves.iter().for_each(|m| {
            crate_mover.interpret_move(m, &mut stacks);
        });

        Ok(stacks
            .iter()
            .map(|s| *s.last().unwrap())
            .collect::<Vec<_>>()
            .join(""))
    } else {
        Err(MoveCratesError::BadInput)
    }
}

fn parse_moves(moves: &str) -> Result<Vec<Move>, MoveCratesError> {
    lazy_static! {
        static ref MOVES: Regex =
            Regex::new(r"move (?P<qty>\d+) from (?P<from_stack>\d) to (?P<to_stack>\d)").unwrap();
    }

    moves
        .lines()
        .map(|m| {
            MOVES
                .captures(m)
                .map_or(Err(MoveCratesError::BadInput), |moves_captures| {
                    Ok(moves_captures)
                })
                .and_then(|capture| {
                    if let (Some(quantity), Some(from_stack), Some(to_stack)) = (
                        capture
                            .name("qty")
                            .and_then(|c| c.as_str().parse::<usize>().ok()),
                        capture
                            .name("from_stack")
                            .and_then(|c| c.as_str().parse::<usize>().ok()),
                        capture
                            .name("to_stack")
                            .and_then(|c| c.as_str().parse::<usize>().ok()),
                    ) {
                        Ok(Move {
                            quantity,
                            from_stack,
                            to_stack,
                        })
                    } else {
                        Err(MoveCratesError::BadInput)
                    }
                })
        })
        // transforms Vec<Result<..., ...>> into Result<Vec<...>, ...>
        .into_iter()
        .collect::<Result<Vec<Move>, MoveCratesError>>()
}

fn parse_stacks(crates: &str) -> Vec<Vec<&str>> {
    lazy_static! {
        static ref INDEXES: Regex = Regex::new(r"\s+(\d)").unwrap();
        static ref CRATES_LINE: Regex = Regex::new(r"(\s{3}|\[([A-Z])\])\s?").unwrap();
    }

    let num_of_stacks = crates
        .lines()
        .last()
        .map(|l| INDEXES.captures_iter(l).collect::<Vec<_>>().len())
        .unwrap();

    let mut stacks: Vec<Vec<&str>> = vec![Vec::<&str>::new(); num_of_stacks];

    for line in crates.lines().rev().skip(1) {
        for (index, capture) in CRATES_LINE.captures_iter(line).enumerate() {
            if let Some(item) = capture.get(2).map(|c| c.as_str()) {
                stacks.index_mut(index).push(item);
            }
        }
    }
    stacks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(move_crane(&CrateMover9000 {}).unwrap(), "VQZNJMWTR");
    }

    #[test]
    fn part_2() {
        assert_eq!(move_crane(&CrateMover9001 {}).unwrap(), "NLCDCLVMQ");
    }
}

const CRATES: &str = "            [L] [M]         [M]
        [D] [R] [Z]         [C] [L]
        [C] [S] [T] [G]     [V] [M]
[R]     [L] [Q] [B] [B]     [D] [F]
[H] [B] [G] [D] [Q] [Z]     [T] [J]
[M] [J] [H] [M] [P] [S] [V] [L] [N]
[P] [C] [N] [T] [S] [F] [R] [G] [Q]
[Z] [P] [S] [F] [F] [T] [N] [P] [W]
 1   2   3   4   5   6   7   8   9 

move 7 from 3 to 9
move 5 from 8 to 9
move 3 from 9 to 5
move 6 from 9 to 2
move 9 from 9 to 3
move 3 from 7 to 3
move 8 from 2 to 3
move 9 from 3 to 1
move 11 from 3 to 8
move 5 from 6 to 9
move 1 from 6 to 3
move 1 from 2 to 7
move 1 from 4 to 8
move 1 from 3 to 9
move 4 from 4 to 3
move 6 from 8 to 3
move 2 from 8 to 2
move 4 from 9 to 3
move 3 from 2 to 5
move 2 from 5 to 4
move 5 from 3 to 4
move 11 from 1 to 4
move 1 from 7 to 6
move 1 from 3 to 5
move 2 from 1 to 9
move 1 from 1 to 4
move 7 from 5 to 8
move 21 from 4 to 6
move 6 from 6 to 2
move 6 from 8 to 9
move 5 from 8 to 5
move 2 from 2 to 7
move 4 from 3 to 7
move 1 from 2 to 6
move 1 from 2 to 5
move 2 from 2 to 7
move 4 from 3 to 7
move 1 from 4 to 6
move 9 from 5 to 3
move 7 from 3 to 4
move 7 from 7 to 3
move 7 from 4 to 1
move 8 from 3 to 5
move 1 from 3 to 5
move 3 from 8 to 2
move 2 from 2 to 9
move 13 from 9 to 4
move 5 from 5 to 3
move 4 from 7 to 6
move 1 from 7 to 4
move 2 from 4 to 2
move 3 from 3 to 4
move 2 from 5 to 2
move 6 from 1 to 7
move 1 from 2 to 8
move 1 from 3 to 8
move 1 from 1 to 6
move 1 from 3 to 4
move 1 from 2 to 6
move 24 from 6 to 1
move 3 from 2 to 3
move 3 from 3 to 5
move 2 from 8 to 6
move 2 from 5 to 4
move 3 from 5 to 1
move 7 from 4 to 8
move 3 from 8 to 9
move 2 from 9 to 5
move 2 from 6 to 3
move 1 from 9 to 8
move 5 from 7 to 5
move 2 from 3 to 1
move 1 from 7 to 1
move 7 from 4 to 7
move 2 from 4 to 8
move 6 from 8 to 6
move 3 from 6 to 9
move 10 from 5 to 1
move 7 from 7 to 1
move 1 from 4 to 9
move 1 from 6 to 3
move 2 from 9 to 7
move 1 from 4 to 2
move 1 from 9 to 5
move 1 from 8 to 5
move 39 from 1 to 8
move 1 from 2 to 5
move 2 from 6 to 9
move 3 from 9 to 5
move 3 from 1 to 6
move 1 from 7 to 2
move 1 from 3 to 2
move 2 from 6 to 2
move 3 from 2 to 3
move 1 from 6 to 2
move 1 from 1 to 8
move 3 from 1 to 2
move 3 from 2 to 4
move 2 from 4 to 5
move 2 from 3 to 8
move 8 from 5 to 2
move 8 from 8 to 2
move 15 from 2 to 7
move 1 from 1 to 5
move 25 from 8 to 7
move 2 from 2 to 4
move 2 from 4 to 3
move 1 from 8 to 4
move 2 from 4 to 6
move 1 from 2 to 1
move 26 from 7 to 2
move 15 from 2 to 1
move 7 from 8 to 9
move 10 from 1 to 6
move 10 from 7 to 2
move 1 from 8 to 1
move 5 from 9 to 8
move 1 from 8 to 9
move 2 from 6 to 9
move 3 from 7 to 1
move 1 from 7 to 1
move 5 from 9 to 2
move 1 from 3 to 1
move 9 from 6 to 3
move 1 from 6 to 1
move 4 from 2 to 4
move 3 from 4 to 8
move 1 from 4 to 1
move 9 from 3 to 1
move 1 from 7 to 6
move 9 from 2 to 5
move 14 from 1 to 6
move 1 from 3 to 8
move 5 from 2 to 6
move 8 from 1 to 8
move 6 from 6 to 8
move 14 from 6 to 7
move 1 from 1 to 7
move 10 from 5 to 4
move 11 from 8 to 5
move 15 from 7 to 1
move 4 from 5 to 6
move 4 from 8 to 9
move 6 from 5 to 3
move 1 from 6 to 9
move 1 from 1 to 6
move 1 from 5 to 8
move 2 from 6 to 2
move 6 from 1 to 5
move 1 from 5 to 8
move 2 from 5 to 4
move 9 from 2 to 9
move 13 from 9 to 8
move 1 from 2 to 1
move 1 from 4 to 8
move 3 from 3 to 1
move 2 from 4 to 5
move 2 from 1 to 5
move 1 from 9 to 3
move 17 from 8 to 1
move 3 from 3 to 2
move 4 from 5 to 1
move 2 from 2 to 4
move 1 from 6 to 1
move 1 from 2 to 8
move 4 from 4 to 6
move 1 from 5 to 9
move 5 from 6 to 8
move 1 from 5 to 4
move 1 from 5 to 6
move 3 from 8 to 6
move 8 from 4 to 5
move 32 from 1 to 7
move 11 from 7 to 6
move 8 from 5 to 3
move 3 from 8 to 7
move 6 from 3 to 9
move 4 from 3 to 8
move 5 from 8 to 2
move 1 from 8 to 5
move 11 from 6 to 3
move 1 from 5 to 2
move 2 from 8 to 6
move 12 from 7 to 8
move 2 from 6 to 2
move 2 from 6 to 4
move 5 from 2 to 5
move 8 from 7 to 2
move 2 from 7 to 1
move 2 from 7 to 6
move 5 from 5 to 4
move 5 from 4 to 7
move 5 from 8 to 2
move 2 from 9 to 7
move 5 from 8 to 4
move 2 from 7 to 3
move 2 from 9 to 3
move 3 from 7 to 9
move 1 from 1 to 8
move 2 from 6 to 1
move 2 from 9 to 8
move 1 from 7 to 8
move 1 from 2 to 5
move 1 from 7 to 9
move 7 from 4 to 3
move 3 from 3 to 6
move 5 from 8 to 6
move 3 from 9 to 5
move 16 from 3 to 1
move 2 from 9 to 1
move 7 from 1 to 8
move 1 from 1 to 2
move 5 from 8 to 2
move 12 from 1 to 4
move 1 from 3 to 5
move 1 from 2 to 9
move 1 from 9 to 4
move 4 from 6 to 5
move 5 from 6 to 1
move 1 from 6 to 5
move 1 from 1 to 4
move 1 from 4 to 7
move 1 from 3 to 7
move 9 from 4 to 6
move 2 from 7 to 8
move 1 from 3 to 4
move 2 from 8 to 9
move 4 from 8 to 4
move 4 from 2 to 8
move 2 from 9 to 7
move 2 from 7 to 8
move 10 from 2 to 4
move 1 from 2 to 1
move 5 from 4 to 7
move 1 from 1 to 3
move 3 from 8 to 7
move 6 from 7 to 2
move 3 from 2 to 7
move 1 from 6 to 7
move 5 from 5 to 8
move 4 from 1 to 3
move 4 from 3 to 1
move 8 from 4 to 2
move 1 from 3 to 2
move 2 from 7 to 2
move 2 from 6 to 3
move 4 from 7 to 2
move 4 from 5 to 7
move 14 from 2 to 7
move 3 from 2 to 1
move 3 from 8 to 2
move 1 from 5 to 7
move 6 from 2 to 4
move 2 from 2 to 7
move 2 from 3 to 6
move 6 from 8 to 2
move 4 from 6 to 4
move 2 from 6 to 9
move 4 from 4 to 2
move 2 from 4 to 8
move 10 from 7 to 2
move 18 from 2 to 6
move 2 from 2 to 6
move 2 from 9 to 2
move 2 from 8 to 5
move 1 from 2 to 9
move 1 from 2 to 9
move 1 from 5 to 7
move 1 from 2 to 6
move 2 from 9 to 2
move 6 from 7 to 3
move 7 from 6 to 8
move 5 from 7 to 2
move 1 from 7 to 4
move 1 from 5 to 7
move 4 from 8 to 7
move 5 from 2 to 3
move 1 from 7 to 5
move 2 from 2 to 8
move 9 from 4 to 3
move 13 from 6 to 8
move 10 from 3 to 1
move 1 from 5 to 2
move 3 from 6 to 8
move 5 from 1 to 2
move 1 from 1 to 8
move 2 from 4 to 3
move 17 from 8 to 6
move 5 from 6 to 3
move 3 from 1 to 2
move 9 from 6 to 5
move 2 from 6 to 8
move 5 from 5 to 9
move 3 from 9 to 8
move 3 from 1 to 3
move 3 from 7 to 5
move 6 from 5 to 8
move 7 from 2 to 4
move 1 from 6 to 3
move 1 from 1 to 5
move 4 from 4 to 5
move 2 from 2 to 9
move 3 from 1 to 3
move 4 from 5 to 8
move 1 from 4 to 5
move 6 from 8 to 7
move 1 from 5 to 2
move 4 from 9 to 2
move 2 from 5 to 9
move 2 from 1 to 8
move 2 from 4 to 9
move 6 from 7 to 5
move 3 from 5 to 2
move 3 from 2 to 5
move 10 from 8 to 3
move 2 from 8 to 5
move 3 from 2 to 5
move 6 from 5 to 1
move 4 from 5 to 6
move 1 from 7 to 5
move 23 from 3 to 7
move 2 from 5 to 9
move 2 from 1 to 5
move 2 from 6 to 3
move 6 from 3 to 1
move 1 from 1 to 7
move 4 from 3 to 1
move 1 from 8 to 5
move 2 from 9 to 2
move 3 from 3 to 8
move 2 from 6 to 8
move 12 from 1 to 3
move 1 from 9 to 7
move 3 from 5 to 9
move 9 from 3 to 8
move 1 from 1 to 7
move 1 from 9 to 4
move 3 from 3 to 6
move 3 from 2 to 1
move 3 from 8 to 6
move 1 from 4 to 2
move 1 from 2 to 9
move 1 from 2 to 7
move 20 from 7 to 5
move 3 from 7 to 3
move 3 from 1 to 3
move 5 from 8 to 1
move 5 from 1 to 5
move 4 from 5 to 2
move 3 from 2 to 6
move 3 from 8 to 7
move 1 from 2 to 6
move 2 from 8 to 6
move 2 from 7 to 5
move 2 from 3 to 6
move 12 from 5 to 1
move 6 from 5 to 7
move 12 from 6 to 8
move 4 from 9 to 3
move 4 from 5 to 8
move 3 from 1 to 5
move 4 from 7 to 4
move 3 from 5 to 9
move 7 from 1 to 6
move 1 from 1 to 3
move 6 from 7 to 6
move 1 from 1 to 3
move 10 from 3 to 6
move 10 from 6 to 2
move 2 from 9 to 5
move 4 from 6 to 5
move 9 from 6 to 1
move 16 from 8 to 7
move 3 from 8 to 7
move 1 from 8 to 1
move 7 from 2 to 1
move 1 from 5 to 9
move 1 from 6 to 1
move 2 from 2 to 1
move 3 from 1 to 4
move 1 from 6 to 8
move 7 from 4 to 1
move 1 from 8 to 2
move 22 from 1 to 8
move 18 from 7 to 9
move 6 from 5 to 2
move 2 from 2 to 7
move 2 from 1 to 5
move 4 from 7 to 6
move 1 from 5 to 6
move 2 from 8 to 2
move 3 from 2 to 6
move 1 from 5 to 6
move 15 from 9 to 6
move 6 from 9 to 5
move 1 from 9 to 8
move 1 from 2 to 9
move 5 from 5 to 9
move 9 from 8 to 6
move 3 from 2 to 7
move 12 from 8 to 9
move 1 from 7 to 5
move 1 from 5 to 7
move 3 from 7 to 1
move 17 from 6 to 3
move 1 from 2 to 6
move 2 from 1 to 4
move 16 from 6 to 4
move 7 from 4 to 6
move 1 from 5 to 7
move 8 from 4 to 5
move 9 from 9 to 8
move 16 from 3 to 7
move 1 from 1 to 5
move 3 from 5 to 1
move 5 from 6 to 2
move 3 from 1 to 7
move 3 from 6 to 7
move 3 from 9 to 3
move 5 from 8 to 5
move 11 from 5 to 7
move 2 from 3 to 7
move 1 from 2 to 1
move 1 from 3 to 6
move 17 from 7 to 9
move 1 from 3 to 2
move 3 from 4 to 6
move 1 from 1 to 2
move 1 from 6 to 4
move 14 from 7 to 6
move 15 from 9 to 6
move 4 from 8 to 7
move 1 from 4 to 7
move 7 from 9 to 5
move 5 from 2 to 9
move 7 from 5 to 1
move 3 from 1 to 7
move 29 from 6 to 4
move 1 from 2 to 4
move 18 from 4 to 2
move 3 from 1 to 4
move 1 from 1 to 7
move 18 from 2 to 4
move 3 from 6 to 5
move 15 from 4 to 1
move 1 from 5 to 1
move 1 from 5 to 4
move 9 from 4 to 1
move 5 from 1 to 3
move 9 from 1 to 5
move 2 from 4 to 3
move 5 from 5 to 6
move 3 from 7 to 9
move 7 from 7 to 5
move 6 from 4 to 6
move 2 from 3 to 7
move 6 from 5 to 8
move 2 from 8 to 4
move 1 from 8 to 9
move 9 from 6 to 2
move 3 from 9 to 3
move 1 from 2 to 1
move 6 from 7 to 4
move 2 from 2 to 8
move 3 from 9 to 5
move 5 from 4 to 8
move 1 from 6 to 9
move 1 from 3 to 1
move 1 from 3 to 4
move 1 from 6 to 5
move 1 from 9 to 3
move 10 from 8 to 7
move 3 from 9 to 2
move 7 from 2 to 4
move 6 from 5 to 7
move 4 from 5 to 8
move 7 from 3 to 2
move 3 from 7 to 1
move 9 from 1 to 5
move 5 from 7 to 9
move 7 from 1 to 4
move 11 from 4 to 2
move 4 from 8 to 3
move 5 from 4 to 7
move 4 from 4 to 1
move 1 from 3 to 6
move 12 from 7 to 4
move 2 from 1 to 8
move 5 from 9 to 7
move 7 from 5 to 6
move 1 from 1 to 4
move 1 from 9 to 8
move 1 from 4 to 7
move 1 from 8 to 9
move 5 from 7 to 9
move 2 from 7 to 5
move 2 from 6 to 3
move 5 from 2 to 7
move 1 from 7 to 8
move 1 from 1 to 6
move 3 from 5 to 1";
