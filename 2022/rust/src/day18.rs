use std::collections::{HashMap};
use std::hash::Hasher;

use itertools::Itertools;
use nom::IResult;
use core::hash::Hash;


use crate::reader::PuzzleInput;

use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::separated_list1;

#[derive(Debug, PartialEq, PartialOrd)]
struct Triple {
    x: f64,
    y: f64,
    z: f64,
}

impl Hash for Triple {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as u64).hash(state);
        (self.y as u64).hash(state);
        (self.z as u64).hash(state);
    }
}

impl Eq for Triple {}

impl From<(f64, f64, f64)> for Triple {
    fn from(t: (f64, f64, f64)) -> Self {
        Triple {
            x: t.0,
            y: t.1,
            z: t.2,
        }
    }
}

fn parse_line(input: &str) -> IResult<&str, (f64, f64, f64)> {
    map(separated_list1(tag(","), nom::character::complete::i32), |v| (v[0] as f64, v[1] as f64, v[2] as f64))(input)
}

fn parse_line_i64(input: &str) -> IResult<&str, (i64, i64, i64)> {
    map(separated_list1(tag(","), nom::character::complete::i64), |v| (v[0], v[1], v[2]))(input)
}

fn faces(voxel: (f64, f64, f64)) -> Vec<Triple> {
    let units = vec!(Triple::from((0.0, 0.0, 0.5)),
    Triple::from((0.0, 0.0, -0.5)),
    Triple::from((0.5, 0.0, 0.0)),
    Triple::from((-0.5, 0.0, 0.0)),
    Triple::from((0.0, 0.5, 0.0)),
    Triple::from((0.0, -0.5, 0.0)));

    units.iter()
        .map(|dir| ((voxel.0 + dir.x), (voxel.1 + dir.y), (voxel.2 + dir.z)))
        .map(Triple::from)
        .collect_vec()
}

pub fn part_one(input: PuzzleInput) -> usize {
    input.as_lines()
        .map(parse_line)
        .map(|r| r.unwrap().1)
        .flat_map(faces)
        .fold(HashMap::new(), |mut acc, face| {
            *acc.entry(face).or_insert(0) += 1;
            acc
        })
        .iter()
        .filter(|(_face, count)| **count == 1)
        .count()
}

pub fn part_one_tuple(input: PuzzleInput) -> usize {
    input.as_lines()
        .map(parse_line_i64)
        .map(|r| r.unwrap().1)
        .map(|(x, y, z)| (x*2, y*2, z*2))
        .flat_map(|(x, y, z)| {
            [
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ]
        })
        .fold(HashMap::new(), |mut acc, face| {
            *acc.entry(face).or_insert(0) += 1;
            acc
        })
        .iter()
        .filter(|(_, count)| **count == 1)
        .count()
}

#[test]
fn test_two() {
    dbg!(part_one(PuzzleInput::from_string("1,1,1\n2,1,1\n")));
}

#[test]
fn test_part_one() {
    dbg!(part_one(PuzzleInput::from_file("resources/day18-test.txt").unwrap()));
}

#[test]
fn run_part_one() {
    dbg!(part_one(PuzzleInput::from_file("resources/day18.txt").unwrap()));
}

#[test]
fn run_part_one_tup() {
    dbg!(part_one_tuple(PuzzleInput::from_file("resources/day18.txt").unwrap()));

}