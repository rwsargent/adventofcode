use std::{fs, error::Error, str::Lines};

use itertools::Itertools;

pub struct PuzzleInput {
    input: String
}

impl PuzzleInput {
    pub fn from_file(filepath: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(filepath)?;

        Ok(PuzzleInput{
            input: contents
        })
    }

    pub fn from_string(rawstring: &str) -> Self{
        PuzzleInput{
            input: String::from(rawstring)
        }
    }

    // pub fn from_string(input: String) -> Self {
    //     PuzzleInput {
    //         input
    //     }
    // }

    pub fn as_string(&self) -> String {
        self.input.clone()
    }

    pub fn as_lines(&self) -> Lines {
        self.input.lines()
    }

    pub fn as_string_vec(&self) -> Vec<String> {
        self.input.lines().map(String::from).collect()
    }

    pub fn as_strs(&self) -> Vec<&str> {
        self.input.lines().collect()
    }
    
    pub fn as_groups(&self) -> Vec<&str> {
        self.input.split("\n\n").collect_vec()
    }
    // pub fn as_byte_grid(&self) -> Vec<&[u8]> {
    //     self.input.lines()
    //         .map(|s| s.as_bytes())
    //         .collect()
    // }

    pub fn as_string_grid(&self) -> Vec<Vec<String>> {
        self.input.lines()
            .map(|l| l.chars().map(String::from).collect_vec())
            .collect()
    }
    // pub fn as_ints(&self) -> Vec<i32> {
    //     self.input.split("\n")
    //         .filter_map(|line| line.trim().parse().ok())
    //         .collect()
    // }
}