use std::collections::HashSet;

use rust::reader::PuzzleInput;

pub fn part_one(signal: String) -> i32{
    let mut c = 4;
    for window in signal.as_bytes().windows(4) {
        if HashSet::<u8>::from_iter(window.iter().map(|el| *el)).len() == 4 {
            return c;
        }
        c += 1;
    }
    0
}

pub fn wrap_part_one(input: PuzzleInput) {
    let input = input.as_string();
    dbg!{part_one(input)};
}

#[test]
fn test_part_one() {
    println!("{}", part_one(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")));
    println!("{}", part_one(String::from("nppdvjthqldpwncqszvftbrmjlhg")));
    println!("{}", part_one(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")));
    println!("{}", part_one(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")));
}

#[test]
fn run_part_one() {
    let input = PuzzleInput::from_file("resources/day6.txt").unwrap().as_string();
    dbg!{part_one(input)};
}

pub fn part_two(signal: String) -> i32{
    let mut c = 14;
    for window in signal.as_bytes().windows(14) {
        if HashSet::<u8>::from_iter(window.iter().map(|el| *el)).len() == 14 {
            return c;
        }
        c += 1;
    }
    0
}

#[test]
fn run_part_two() {
    let input = PuzzleInput::from_file("resources/day6.txt").unwrap().as_string();
    dbg!{part_two(input)};
}