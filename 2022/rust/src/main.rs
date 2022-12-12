mod reader;
mod day1;
mod puzzle;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
pub mod cursor;
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

    Ok(())
}
