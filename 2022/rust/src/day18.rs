use std::collections::{HashSet};

use nom::IResult;

use crate::reader::PuzzleInput;

use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::separated_list1;

fn parse_line_i64(input: &str) -> IResult<&str, (i64, i64, i64)> {
    map(separated_list1(tag(","), nom::character::complete::i64), |v| (v[0], v[1], v[2]))(input)
}

type Triple = (i64, i64, i64);

pub fn part_one(input: PuzzleInput) -> usize {
    let cubes = input.as_lines()
        .map(parse_line_i64)
        .map(|r| r.unwrap().1)
        .collect::<HashSet<_>>();

        cubes.iter()
        .flat_map(|trip| {
            let (x, y, z) = *trip;
            [
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ]
        })
        .filter(|trip| !(&cubes).contains(trip))
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

pub fn part_two(input: PuzzleInput) -> usize {
    let cubes = input.as_lines()
        .map(parse_line_i64)
        .map(|r| r.unwrap().1)
        .collect::<HashSet<_>>();

    let (min, max) = cubes.iter()
        .fold(((i64::MAX, i64::MAX, i64::MAX), (i64::MIN, i64::MIN, i64::MIN)), 
            |(mut min, mut max), cube| {
                min.0 = min.0.min(cube.0);
                min.1 = min.1.min(cube.1);
                min.2 = min.2.min(cube.2);

                max.0 = max.0.max(cube.0);
                max.1 = max.1.max(cube.1);
                max.2 = max.2.max(cube.2);

                (min, max)
            });
    dbg!(&min, &max);
    cubes.iter()
        .flat_map(|trip| {
            let (x, y, z) = *trip;
            [
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ]
        })
        .filter(|trip| !(&cubes).contains(trip) && can_reach_outside(trip, &cubes,&min, &max, &mut HashSet::new()))
        .count()
}

fn can_reach_outside(trip: &Triple, cubes: &HashSet<Triple>, min: &Triple, max: &Triple, visited: &mut HashSet<Triple>) -> bool {
    if trip.0 < min.0 || trip.0 > max.0 ||
        trip.1 < min.1 || trip.0 > max.0 ||
        trip.2 < min.2 || trip.0 > max.0 {
        return true;
    }
    if cubes.contains(trip) || visited.contains(trip){
        return false;
    }
    visited.insert(*trip);

    [
        (trip.0 + 1, trip.1, trip.2),
        (trip.0 - 1, trip.1, trip.2),
        (trip.0, trip.1 + 1, trip.2),
        (trip.0, trip.1 - 1, trip.2),
        (trip.0, trip.1, trip.2 + 1),
        (trip.0, trip.1, trip.2 - 1),
    ].iter().any(|edge| can_reach_outside(edge, cubes, min, max, visited))
}


#[test]
fn test_part_two() {
    dbg!(part_two(PuzzleInput::from_file("resources/day18-test.txt").unwrap()));
}

#[test]
fn run_part_two() {
    dbg!(part_two(PuzzleInput::from_file("resources/day18.txt").unwrap()));
}
