use std::ops::Range;

use text_io::scan;

use crate::reader::PuzzleInput;

#[test]
fn test_scan() {
    let line = "2-4,6-8";
    let (a, b, c, d): (i32, i32, i32, i32);
    scan!(line.bytes() => "{}-{},{}-{}", a, b, c, d);
    println!("a: {}, b:{}, c:{}, d{}", a, b, c, d);
}

fn parse_line(s: &String) -> (Range<i32>, Range<i32>) {
    let (a, b, c, d): (i32, i32, i32, i32);
    scan!(s.bytes() => "{}-{},{}-{}", a, b, c, d);
    (a..b, c..d)
}

fn contains(left: &Range<i32>, right: &Range<i32>) -> bool {
    left.start >= right.start && left.start <= right.end &&
    left.end <= right.end && left.end >= right.start
}

fn overlap(left: &Range<i32>, right: &Range<i32>) -> bool {
    left.start >= right.start && left.start <= right.end ||
    left.end <= right.end && left.end >= right.start
}

fn part_one(input: PuzzleInput) {
    let count = input.as_string_vec().iter()
        .map(parse_line)
        .filter(|rs| contains(&rs.0, &rs.1) || contains(&rs.1, &rs.0))
        .count();
    println!("{}", count);
}

#[test]
fn test_part_one() {
    let input = PuzzleInput::from_string(r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#);
    part_one(input);
}

#[test]
fn run_part_one() {
    let input = PuzzleInput::from_file("resources/day4.txt");
    part_one(input.unwrap());
}

fn part_two(input: PuzzleInput) {
    let count = input.as_string_vec().iter()
        .map(parse_line)
        .filter(|rs| overlap(&rs.0, &rs.1) || overlap(&rs.1, &rs.0))
        .count();
    println!("{}", count);
}

#[test]
fn run_part_two() {
    let input = PuzzleInput::from_file("resources/day4.txt");
    part_two(input.unwrap());
}