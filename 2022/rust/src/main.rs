mod reader;
mod puzzle;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day21;
pub mod coord;
use std::{env, error::Error, fmt::Display};

use puzzle::Puzzle;
use crate::reader::PuzzleInput;

fn main() -> Result<(), Box<dyn Error>>{
    let day = env::args().skip(1).take(1).next().ok_or("No day argument")?;
    
    let puzzle: Box<dyn Puzzle<Box<dyn Display>>> = match day.as_str() {
        "1" => Box::new(day1::DayOne::new()),
        "2" => Box::new(day2::DayTwo::from_input(PuzzleInput::from_file("resources/day2.txt")?)),
        _ => panic!("Uknown day")
    };

    println!{"{}", puzzle.run_part_one()?}
    println!{"{}", puzzle.run_part_two()?} 

    let result = day16::part_two(PuzzleInput::from_file("resources/day16.txt").unwrap());
    println!("Answer: {}", result);

    Ok(())
}
