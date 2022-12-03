use std::{str::FromStr, fmt::Display};

use crate::{puzzle::Puzzle, reader::PuzzleInput};
use itertools::Itertools;
use strum_macros::EnumString;


pub struct DayTwo {
    rounds: Vec<String>
}

#[derive(Debug, PartialEq, EnumString)]
enum RPS {
    #[strum(serialize = "A", serialize = "X")]
    Rock,

    #[strum(serialize = "B", serialize = "Y")]
    Paper,

    #[strum(serialize = "C", serialize = "Z")]
    Scissors, 
}

#[derive(Debug, PartialEq, EnumString,Clone, Copy)]
enum Outcome{
    #[strum(serialize = "Z")]
    Win,
    #[strum(serialize = "X")]
    Lose,
    #[strum(serialize = "Y")]
    Tie
}

impl Outcome {
    fn points(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Tie => 3,
            Outcome::Lose => 0,
        }
    }
}

impl RPS {
    fn points(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3
        }
    }

    fn loses_to(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }

    fn beats(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }
}

fn get_throw(shape: RPS, outcome: Outcome) -> RPS {
    match outcome {
        Outcome::Tie => shape,
        Outcome::Win => shape.loses_to(),
        Outcome::Lose => shape.beats(),
    }
}
impl DayTwo {
    pub fn from_input(input: PuzzleInput) -> Self {
        DayTwo{
            rounds: input.as_string_vec()
        }
    }
}

fn round_to_enums(round: &String) -> (RPS, RPS) {
    round.split(" ").filter_map(|throw| RPS::from_str(throw).ok()).collect_tuple().unwrap()
}

fn calculate_score(round: (RPS, RPS)) -> i32 {
    let (oppo, player) = round;
    player.points() + match (oppo, player) {
        (RPS::Rock, RPS::Paper) => 6,
        (RPS::Rock, RPS::Scissors) => 0,
        (RPS::Paper, RPS::Rock) => 0,
        (RPS::Paper, RPS::Scissors) => 6,
        (RPS::Scissors, RPS::Rock) => 6,
        (RPS::Scissors, RPS::Paper) => 0,
        (RPS::Rock, RPS::Rock) => 3,
        (RPS::Paper, RPS::Paper) => 3,
        (RPS::Scissors, RPS::Scissors) => 3,
    }
}

fn round_to_enums2(line: &String) -> (RPS, Outcome) {
    let parts: Vec<&str> =  line.split(" ").collect();
    (RPS::from_str(parts[0]).unwrap(), Outcome::from_str(parts[1]).unwrap())
}


impl Puzzle<Box<dyn Display>> for DayTwo {
    fn run_part_one(&self) -> Result<Box<dyn Display>, String> {
        Ok(Box::new(self.rounds.iter()
            .map(round_to_enums)
            .map(calculate_score)
            .sum::<i32>()))
            
    }

    fn run_part_two(&self) -> Result<Box<dyn Display>, String> {
        let total = self.rounds.iter()
            .map(round_to_enums2)
            .map(|round| (round.1, get_throw(round.0, round.1.clone())))
            .map(|outcome| outcome.0.points() + outcome.1.points())
            .sum::<i32>();
        Ok(Box::new(total))
    }
}

#[test]
fn test_day_one() {
    let d2 = DayTwo{
        rounds: vec![
            String::from("A Y"),
            String::from("B X"),
            String::from("C Z"),
            ]
    };
    assert_eq!(*d2.run_part_one().unwrap().to_string(), 15.to_string())
}

#[test]
fn test_day_two() {
    let d2 = DayTwo{
        rounds: vec![
            String::from("A Y"),
            String::from("B X"),
            String::from("C Z"),
            ]
    };
    assert_eq!(*d2.run_part_two().unwrap().to_string(), 12.to_string())
}