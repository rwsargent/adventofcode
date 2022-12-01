use std::fmt::Display;

pub trait Puzzle<D> 
where D: Display {
    fn run_part_one(&self) -> Result<D, String>;
    fn run_part_two(&self) -> Result<D, String>;
}

// pub struct EmptyPuzzle {}