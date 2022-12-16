use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};

use crate::{coord::{Coord, Direction}, reader::PuzzleInput};


fn parse_coord(input: &str) -> IResult<&str, Coord> {
    map(tuple((
            preceded(tag("x="), nom::character::complete::i32),
            preceded(tag(", y="), nom::character::complete::i32))), 
        |(x, y)| Coord { x, y })(input)
    }

fn parse_line(input: &str) -> IResult<&str, (Coord, Coord)> {
    tuple((preceded(tag("Sensor at "), parse_coord),
          (preceded(tag(": closest beacon is at "), parse_coord))))(input)
}

pub fn part_one(input: PuzzleInput, row: i32) -> usize {
    let sensors_and_beacons: Vec<(Coord, Coord)> = input.as_lines()
        .map(parse_line)
        .map(|r| r.unwrap().1)
        .collect();

    let mut signals = HashSet::new();
    let mut beacons = HashSet::new();
    for (sensor, beacon) in sensors_and_beacons {
        beacons.insert(beacon);
        let manhattan = sensor.manhattan(&beacon);
        if sensor.y.abs_diff(row) <= manhattan {
            let intersect = manhattan - sensor.y.abs_diff(row);
            for x in (sensor.x - (intersect as i32))..=(sensor.x + (intersect as i32)) {
                signals.insert(x);
            }
        }
    }
    signals.iter().filter(|x| !beacons.contains(&Coord{x: **x, y: row})).count()
}

#[test]
fn test_part_one() {
    let input = PuzzleInput::from_file("resources/day15-test.txt").unwrap();
    dbg!(part_one(input, 10));
}

#[test]
fn run_part_one() {
    let input = PuzzleInput::from_file("resources/day15.txt").unwrap();
    dbg!(part_one(input, 2_000_000));
}

pub fn part_two(input: PuzzleInput, bound: i32) -> usize {
    let sensor_radius: Vec<(Coord, u32)> = input.as_lines()
        .map(parse_line)
        .map(|r| r.unwrap().1)
        .map(|(s, b)| (s, s.manhattan(&b)))
        .collect();

    for (sensor, radius) in &sensor_radius {
        let distance = radius + 1;
        let mut cursor = Coord {x: sensor.x, y: sensor.y - distance as i32};

        // walk the perimeter;
        for d in [Direction::SE, Direction::SW, Direction::NW, Direction::NE] {
            for _ in 0..distance {
                cursor = cursor.in_direction(&d);
                if !(0i32..=bound).contains(&cursor.x) || !(0i32..=bound).contains(&cursor.y) {
                    continue;
                }

                let mut out_of_bounds = true;
                for (s, r) in &sensor_radius {
                    out_of_bounds &= s.manhattan(&cursor) > *r;
                }

                if out_of_bounds {
                    return (cursor.x as i64 * 4_000_000i64 + cursor.y as i64) as usize;
                }
            }
        }
    }
    0
}

#[test]
fn test_part_two() {
    let input = PuzzleInput::from_file("resources/day15-test.txt").unwrap();
    dbg!(part_two(input, 20));
}

#[test]
fn run_part_two() {
    let input = PuzzleInput::from_file("resources/day15.txt").unwrap();
    dbg!(part_two(input, 4_000_000));
}