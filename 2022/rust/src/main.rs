mod reader;
mod day1;
mod puzzle;

use std::{env, error::Error};

use puzzle::Puzzle;

fn main() -> Result<(), Box<dyn Error>>{
    let day = env::args().skip(1).take(1).next().ok_or("No day argument")?;
    
    let puzzle = match day.as_str() {
        "1" => day1::DayOne::new(),
        _ => panic!("Uknown day")
    };

    println!{"{}", puzzle.run_part_one()?}
    println!{"{}", puzzle.run_part_two()?} 

    Ok(())
}
