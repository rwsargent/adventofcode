use std::collections::HashSet;

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::{IResult, multi::separated_list1};
use nom::character::complete::{self as cc};
use crate::coord::Coord;
use crate::reader::PuzzleInput;

// type Sand = Coord;

// impl Sand {
//     fn drop(cavern: HashSet<Coord>, bottom: i32) -> Result<Coord, String> {
//         let sand = Coord{x:500, y:0};

//         Ok()
//     }
// }

fn parse_pair(input: &str) -> IResult<&str, Coord> {
    map(tuple((cc::i32, tag(","), cc::i32)), |(x, _, y)| Coord{x, y})(input)
}

fn parse_paths(i: &str) -> IResult<&str, Vec<Coord>> {
    separated_list1(tag(" -> "), parse_pair)(i)
}
fn build_grid(paths: Vec<Vec<Coord>>) -> HashSet<Coord> {
    let mut grid:HashSet<Coord> = HashSet::new();
    for path in paths {
        for (start, stop) in path.iter().tuple_windows(){
            let all_between = start.all_between(stop);
            grid.extend(all_between);
            grid.insert(*stop);
        }
    }
    grid
}
pub fn part_one(input: PuzzleInput) -> u32 {
    let paths: Vec<_> = input.as_lines()
        .map(parse_paths)
        .filter_map(|r| r.ok())
        .map(|(_i, p)| p)
        .collect();

    let mut grid = build_grid(paths);
    let bottom = grid.iter().map(|c| c.y).max().unwrap();
    let mut sand_count = 0;
    'simulation: loop {
        let mut sand = Coord{x: 500, y: 0};
        loop {
            if sand.y > bottom {break 'simulation};
            // try down;
            if !grid.contains(&sand.down()) {
                sand = sand.down();
                continue;
            }
            // try down-left
            if !grid.contains(&sand.sw()) {
                sand = sand.sw();
                continue;
            }

            // try down-right
            if !grid.contains(&sand.se()) {
                sand = sand.se();
                continue;
            }

            // final resting place
            grid.insert(sand);
            sand_count += 1;
            break;
        }
    }
    sand_count
}

pub fn draw_grid(grid: &HashSet<Coord>, xs: (i32, i32), ys: (i32, i32)) {
    for y in  ys.0..=ys.1 {
        for x in xs.0..=xs.1 {
            if grid.contains(&Coord{x, y}) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
} 

#[test]
fn test_draw_grid() {
    let paths: Vec<_> = PuzzleInput::from_string(r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#).as_lines()
        .map(parse_paths)
        .filter_map(|r| r.ok())
        .map(|(_i, p)| p)
        .collect();

    let grid = build_grid(paths);
    draw_grid(&grid, (494, 503), (0, 9))
}

#[test]
fn test_part_one() {
    let input = PuzzleInput::from_string(r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#);
    dbg!(part_one(input));
}

#[test]
fn run_part_one() {
    dbg!(part_one(PuzzleInput::from_file("resources/day14.txt").unwrap()));
}

fn part_two(input: PuzzleInput) -> u32 {
    let paths: Vec<_> = input.as_lines()
        .map(parse_paths)
        .filter_map(|r| r.ok())
        .map(|(_i, p)| p)
        .collect();

    let mut grid = build_grid(paths);
    let bottom = grid.iter().map(|c| c.y).max().unwrap() + 2;

    // find bottom left 
    let mut cursor = Coord {x: 500, y: 0};
    while cursor.y < bottom {
        cursor = cursor.sw();
    }
    let bot_left = cursor;

    // find bot_right;
    cursor = Coord {x: 500, y: 0};
    while cursor.y < bottom {
        cursor = cursor.se();
    }
    let bot_right = cursor;

    grid.extend(bot_left.all_between(&bot_right));
    grid.insert(bot_right);

    let mut sand_count = 0;
    'simulation: loop {
        let mut sand = Coord{x: 500, y: 0};
        loop {
            if grid.contains(&sand) {
                break 'simulation;
            }
            // try down;
            if !grid.contains(&sand.down()) {
                sand = sand.down();
                continue;
            }
            // try down-left
            if !grid.contains(&sand.sw()) {
                sand = sand.sw();
                continue;
            }

            // try down-right
            if !grid.contains(&sand.se()) {
                sand = sand.se();
                continue;
            }

            // final resting place
            grid.insert(sand);
            sand_count += 1;
            break;
        }
    }
    sand_count

}

#[test]
fn draw_part_two() {
    let input = PuzzleInput::from_string(r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#);
    let paths: Vec<_> = input.as_lines()
        .map(parse_paths)
        .filter_map(|r| r.ok())
        .map(|(_i, p)| p)
        .collect();

    let mut grid = build_grid(paths);
    let bottom = grid.iter().map(|c| c.y).max().unwrap() + 2;

    // find bottom left 
    let mut cursor = Coord {x: 500, y: 0};
    while cursor.y < bottom {
        cursor = cursor.sw();
    }
    let bot_left = cursor;

    // find bot_right;
    cursor = Coord {x: 500, y: 0};
    while cursor.y < bottom {
        cursor = cursor.se();
    }
    let bot_right = cursor;

    grid.extend(bot_left.all_between(&bot_right));
    grid.insert(bot_right);

    draw_grid(&grid, (bot_left.x - 2, bot_right.x + 2), (0, bottom));

}
#[test]
fn test_part_two() {
    let input = PuzzleInput::from_string(r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#);
    dbg!(part_two(input));
}

#[test]
fn run_part_two() {
    dbg!(part_two(PuzzleInput::from_file("resources/day14.txt").unwrap()));
}