use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    coord::{Coord, Direction},
    reader::PuzzleInput,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum PieceType {
    Dash,
    Plus,
    Pipe,
    Square,
    L,
}
#[derive(PartialEq, Eq)]
struct Piece {
    blocks: Vec<Coord>,
}

impl Piece {
    fn new(piece_type: &PieceType, height: i32) -> Self {
        match piece_type {
            PieceType::Dash => Piece {
                blocks: vec![
                    Coord {
                        x: 0,
                        y: 0 + height,
                    },
                    Coord {
                        x: 1,
                        y: 0 + height,
                    },
                    Coord {
                        x: 2,
                        y: 0 + height,
                    },
                    Coord {
                        x: 3,
                        y: 0 + height,
                    },
                ],
            },
            PieceType::Plus => Piece {
                blocks: vec![
                    Coord {
                        x: 1,
                        y: 0 + height,
                    },
                    Coord {
                        x: 0,
                        y: 1 + height,
                    },
                    Coord {
                        x: 1,
                        y: 1 + height,
                    },
                    Coord {
                        x: 2,
                        y: 1 + height,
                    },
                    Coord {
                        x: 1,
                        y: 2 + height,
                    },
                ],
            },
            PieceType::L => Piece {
                blocks: vec![
                    Coord {
                        x: 2,
                        y: 2 + height,
                    },
                    Coord {
                        x: 2,
                        y: 1 + height,
                    },
                    Coord {
                        x: 2,
                        y: 0 + height,
                    },
                    Coord {
                        x: 1,
                        y: 0 + height,
                    },
                    Coord {
                        x: 0,
                        y: 0 + height,
                    },
                ],
            },
            PieceType::Square => Piece {
                blocks: vec![
                    Coord {
                        x: 0,
                        y: 0 + height,
                    },
                    Coord {
                        x: 1,
                        y: 1 + height,
                    },
                    Coord {
                        x: 1,
                        y: 0 + height,
                    },
                    Coord {
                        x: 0,
                        y: 1 + height,
                    },
                ],
            },
            PieceType::Pipe => Piece {
                blocks: vec![
                    Coord {
                        x: 0,
                        y: 0 + height,
                    },
                    Coord {
                        x: 0,
                        y: 1 + height,
                    },
                    Coord {
                        x: 0,
                        y: 2 + height,
                    },
                    Coord {
                        x: 0,
                        y: 3 + height,
                    },
                ],
            },
        }
    }

    fn moove<F>(&mut self, direction: &Direction, is_valid: F) -> bool
    where
        F: Fn(&Coord) -> bool,
    {
        let moved: Vec<_> = self
            .blocks
            .iter()
            .map(|b| b.in_direction(&direction))
            .filter(|b| is_valid(b))
            .collect();
        if moved.len() != self.blocks.len() {
            return false;
        }
        self.blocks = moved;
        true
    }
}

const ORDER: [PieceType; 5] = [
    PieceType::Dash,
    PieceType::Plus,
    PieceType::L,
    PieceType::Pipe,
    PieceType::Square,
];

#[allow(dead_code)]
fn draw_board(board: &HashSet<Coord>, height: i32, piece: Option<&Piece>) {
    for y in 0..height {
        print!("|");
        for x in 0..7 {
            let pixel = Coord {
                x,
                y: height - y - 1,
            };
            match piece {
                Some(c) => {
                    if c.blocks.iter().find(|b| **b == pixel).is_some() {
                        print!("@");
                        continue;
                    }
                }
                None => (),
            }
            if board.contains(&pixel) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
}

pub fn part_one(input: PuzzleInput) -> i32 {
    let inputs: Vec<_> = input
        .as_string()
        .chars()
        .map(|c| if c == '<' { Direction::W } else { Direction::E })
        .collect();
    let mut max_height = 0;
    let mut board: HashSet<Coord> = HashSet::new();
    let mut inputs_idx: usize = 0;
    // let noop = |_| true;
    for rock in 0..2022 {
        let mut piece = Piece::new(&ORDER[rock % ORDER.len()], max_height + 3);
        piece.moove(&Direction::E, |_| true);
        piece.moove(&Direction::E, |_| true);

        // println!("Rock: {}", rock);
        // draw_board(&board, max_height + 4, Some(&piece));
        loop {
            // apply jet
            piece.moove(&inputs[inputs_idx % inputs.len()], |coord| {
                !board.contains(coord) && coord.x >= 0 && coord.x < 7
            });
            inputs_idx += 1;
            // drop a level
            if !piece.moove(&Direction::S, |coord| {
                !board.contains(coord) && coord.x >= 0 && coord.x < 7 && !(coord.y < 0)
            }) {
                break;
            }
        }
        max_height = max_height.max(piece.blocks.iter().map(|b| b.y).max().unwrap() + 1);
        board.extend(piece.blocks.iter());

        // println!("|~~~~~~~|");
        // draw_board(&board, max_height + 3, None);
    }
    max_height
}

#[test]
fn test_part_one() {
    dbg!(part_one(PuzzleInput::from_string(
        ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
    )));
}

#[test]
fn run_part_one() {
    dbg!(part_one(
        PuzzleInput::from_file("resources/day17.txt").unwrap()
    ));
}

#[derive(PartialEq, Eq)]
struct State {
    last_piece: Vec<i32>,
    jet_index: usize,
    piece_type: PieceType,
}

pub fn part_two(input: PuzzleInput) {
    let inputs: Vec<_> = input
        .as_string()
        .chars()
        .map(|c| if c == '<' { Direction::W } else { Direction::E })
        .collect();
    let mut max_height = 0u64;
    let mut board: HashSet<Coord> = HashSet::new();
    let mut inputs_idx: usize = 0;
    let mut saved_states: Vec<State> = Vec::new();
    for rock in 0..30_000 {
        let mut piece = Piece::new(&ORDER[rock % ORDER.len()], 0  + 3);
        piece.moove(&Direction::E, |_| true);
        piece.moove(&Direction::E, |_| true);

        loop {
            piece.moove(&inputs[inputs_idx % inputs.len()], |coord| {
                !board.contains(coord) && coord.x >= 0 && coord.x < 7
            });
            inputs_idx += 1;
            if !piece.moove(&Direction::S, |coord| {
                !board.contains(coord) && coord.x >= 0 && coord.x < 7 && !(coord.y < 0)
            }) {
                break;
            }
        }
        max_height = max_height.max(piece.blocks.iter().map(|b| b.y as u64).max().unwrap() + 1);
        board.extend(piece.blocks.iter());

        let state = State {
            last_piece: piece.blocks.iter().map(|b| b.x).collect_vec(),
            jet_index: inputs_idx,
            piece_type: *(&ORDER[rock % ORDER.len()]),
        };

        match saved_states.iter().find_position(|s| **s == state) {
            Some((position, _state)) => {
                let cycle_length = rock - position;
                let repeat_count = 1_000_000_000_000u64 / cycle_length as u64;
                println!("Cycle found at {}", position);
                return;
            },
            None => (),
        }
        saved_states.push(state);

    }
    println!("No cycle detected.");
}


#[test]
fn test_part_two() {
    part_two(PuzzleInput::from_string(
        ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
    ));
}