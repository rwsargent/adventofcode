use crate::reader::PuzzleInput;
use std::{collections::HashSet};

trait Priority {
    fn priority(self) -> i32;
}

impl Priority for char {
    fn priority(self) -> i32 {
        if self.is_ascii_lowercase() {
            self as i32 - 'a' as i32 + 1i32
        } else {
            self as i32 - 'A' as i32 + 27i32
        }
    }
}

#[allow(dead_code)]
fn part_one(input: PuzzleInput) {
    let lines = input.as_string_vec();
    let mut sum = 0;
    for line in lines {
        let mut compartment_one = HashSet::new();
        let mut compartment_two = HashSet::new();
        let mut i = 0;
        for char in line.chars() {
            if i < line.len() / 2 {
                compartment_one.insert(char.priority());
            } else {
                compartment_two.insert(char.priority());
            }
            i += 1;
        }
        
        for item in compartment_one.iter() {
            if compartment_two.contains(item) {
                sum += item;
            }
        }
    }
    println!{"part one: {}", sum};
}

#[allow(dead_code)]
fn part_one_functional(input: PuzzleInput) {
    let lines = input.as_string_vec();
    let result = lines.iter().fold(0, |acc, line| {
        let top: HashSet<i32> = HashSet::from_iter(line.chars().take(line.len() / 2).map(|c| c.priority()));
        let bottom: HashSet<i32> = HashSet::from_iter(line.chars().skip(line.len() / 2).map(|c| c.priority()));
        acc + *top.intersection(&bottom).next().unwrap()
    });
    println!{"{}", result};
}


#[test]
fn test_part_one() {
    let input = PuzzleInput::from_string(r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#);
part_one_functional(input);
}

#[test]
fn run_part_one(){
    let input = PuzzleInput::from_file("resources/day3.txt");
    part_one(input.unwrap());
}



fn to_set(line: &String) -> HashSet<i32> {
    line.chars().fold(HashSet::new(), |mut s, c| {
        s.insert(c.priority());
        s
    })
}

#[allow(dead_code)]
fn part_two(input: PuzzleInput) {
    let lines = input.as_string_vec();
    let mut i = 0;
    let mut sum = 0;
    while i < lines.len() {
        let one = to_set(&lines[i]);
        let two = to_set(&lines[i + 1]);
        let three = to_set(&lines[i + 2]);

        for item in one.iter() {
            if two.contains(item) && three.contains(item) {
                sum += item;
            }
        }
        i += 3
    }
    println!{"part two: {}", sum};
}

#[test]
fn test_part_two() {
    let input = PuzzleInput::from_string(r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#);
    part_two(input);
}

#[test]
fn run_part_two() {
    let input = PuzzleInput::from_file("resources/day3.txt");
    part_two(input.unwrap())   
}