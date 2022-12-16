use std::{collections::{HashSet}, str::FromStr,};
use strum_macros::{EnumString};

use crate::{reader::PuzzleInput, coord::Coord};
#[derive(Debug, EnumString)]
enum Command {
    #[strum(serialize = "U")]
    Up,
    #[strum(serialize = "D")]
    Down,
    #[strum(serialize = "L")]
    Left,
    #[strum(serialize = "R")]
    Right,
}

pub fn part_one(input: PuzzleInput) -> usize {
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut head: Coord = Default::default();
    let mut tail: Coord = Default::default();
    visited.insert(tail.clone());

    for line in input.as_lines() {
        let mut split = line.split(" ");
        let (command, amount) = (Command::from_str(split.next().unwrap()).unwrap(), split.next().map(|s| s.parse::<i32>()).unwrap().unwrap());
        for _ in 0..amount {
            head = match command {
                Command::Up => head.up(),
                Command::Down => head.down(),
                Command::Left => head.left(),
                Command::Right => head.right(),
            };

            let d = head.dist(&tail);
            if  d > std::f64::consts::SQRT_2 {
                let neighbors = tail.neighbors();
                // Move to the neighbor who minimizes tails distance from head
                tail = neighbors.iter()
                    .map(|n| (n.dist(&head), n))
                    .fold((f64::MAX, &Default::default()), |acc, n| {
                        if n.0 < acc.0 { n } else { acc }
                    }).1.clone();
            }
            visited.insert(tail.clone());
        }
    }
    visited.len()
}

#[test]
fn test_part_one() {
    let input = PuzzleInput::from_string(r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#);

    dbg!(part_one(input));
}

#[test]
fn run_part_one() {
    let input = PuzzleInput::from_file("resources/day9.txt").unwrap();
    dbg!(part_one(input));
}

pub fn part_two(input: PuzzleInput) -> usize {
    let mut rope: Vec<Coord> = (0..10).map(|_| Coord{x:0, y:0}).collect();
    let mut visited:HashSet<Coord> = HashSet::new();
    visited.insert(rope.last().unwrap().clone());

    for line in input.as_lines() {
        let mut split = line.split(" ");
        let (command, amount) = (Command::from_str(split.next().unwrap()).unwrap(), split.next().map(|s| s.parse::<i32>()).unwrap().unwrap());
        for _ in 0..amount {
            rope[0] = match command {
                Command::Up => rope[0].up(),
                Command::Down => rope[0].down(),
                Command::Left => rope[0].left(),
                Command::Right => rope[0].right(),
            };
            for t_idx in 1..rope.len() {
                let (head, tail) = (rope[t_idx - 1], rope[t_idx]);
                let d = head.dist(&tail);
                if  d > std::f64::consts::SQRT_2 {
                    let neighbors = tail.neighbors();
                    // Move to the neighbor who minimizes tail's distance from head
                    let step = neighbors.iter()
                        .map(|n| (n.dist(&head), n))
                        .fold((f64::MAX, &Default::default()), |acc, n| {
                            if n.0 < acc.0 { n } else { acc }
                        }).1.clone();
                    rope[t_idx] = step;
                }
            }
            visited.insert(rope.last().unwrap().clone());
        }
    }
    visited.len()
}

#[test]
fn test_part_two() {
    let input = PuzzleInput::from_string(r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#);
    dbg!(part_two(input));
}


#[test]
fn run_part_two() {
    let input = PuzzleInput::from_file("resources/day9.txt").unwrap();
    dbg!(part_two(input));
}