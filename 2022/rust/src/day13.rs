use std::{cmp::Ordering, fmt::Debug};

use crate::{reader::PuzzleInput};
use nom::{IResult, multi::separated_list0, bytes::complete::tag, branch::alt, sequence::{delimited}, Parser};

#[derive(PartialEq, Eq, PartialOrd)]
pub enum Packet {
    Value(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Value(l), Packet::Value(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => l.cmp(r),
            (Packet::Value(v), Packet::List(_)) => Packet::List(vec![Packet::Value(*v)]).cmp(other),
            (Packet::List(l), Packet::Value(r)) => l.cmp(&vec![Packet::Value(*r)]),
        }
    }
}

impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{}", v),
            Self::List(l) => f.debug_list().entries(l.iter()).finish(),
        }
    }
}

pub fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]")).map(|v| Packet::List(v)),
        nom::character::complete::i32.map(|n| Packet::Value(n))
    ))(input)
}


pub fn part_one(input: PuzzleInput) -> usize {
    input.as_groups()
        .iter()
        .map(|g| g.split("\n").collect::<Vec<&str>>())
        .map(|gs| (gs[0], gs[1]))
        .map(|(l, r)| (parse_packet(l).unwrap().1, parse_packet(r).unwrap().1))
        .map(|(l, r)| l.cmp(&r))
        .enumerate()
        .filter(|(_, o)| *o == Ordering::Less)
        .map(|(idx, _)| idx + 1)
        .sum()
}

#[test]
fn test_part_one() {
    // assert!(parse_packet("[1,1,3,1,1]").unwrap().1.cmp(&parse_packet("[1,1,5,1,1]").unwrap().1) == Ordering::Less);
    // assert!(parse_packet("[[1],[2,3,4]]").unwrap().1.cmp(&parse_packet("[[1],4]").unwrap().1) == Ordering::Less);
    // assert!(parse_packet("[9]").unwrap().1.cmp(&parse_packet("[8,7,6]").unwrap().1) == Ordering::Greater);
    // assert!(parse_packet("[[4,4],4,4]").unwrap().1.cmp(&parse_packet("[[4,4],4,4,4]").unwrap().1) == Ordering::Less);
    // assert!(parse_packet("[7,7,7,7]").unwrap().1.cmp(&parse_packet("[7,7,7]").unwrap().1) == Ordering::Greater);
    // assert!(parse_packet("[]").unwrap().1.cmp(&parse_packet("[3]").unwrap().1) == Ordering::Less);
    // assert!(parse_packet("[[[]]]").unwrap().1.cmp(&parse_packet("[[]]").unwrap().1) == Ordering::Greater);
    // assert!(parse_packet("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap().1.cmp(&parse_packet("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap().1) == Ordering::Greater);

    dbg!(part_one(PuzzleInput::from_file("resources/day13-test.txt").unwrap()));
}

#[test]
fn run_part_one() {
    dbg!(part_one(PuzzleInput::from_file("resources/day13.txt").unwrap()));
}

pub fn part_two(input: PuzzleInput) -> usize {
    let two = Packet::List(vec!(Packet::List(vec!(Packet::Value(2)))));
    let six = Packet::List(vec!(Packet::List(vec!(Packet::Value(6)))));

    let mut packets = input.as_lines()
    .filter_map(|l| parse_packet(l).ok())
    .map(|(_, p)| p)
    .collect::<Vec<Packet>>();
    packets.push(Packet::List(vec!(Packet::List(vec!(Packet::Value(2))))));
    packets.push(Packet::List(vec!(Packet::List(vec!(Packet::Value(6))))));

    packets.sort_by(|l, r| l.cmp(r));
    // println!("{:#?}", packets);


    packets
        .iter()
        .enumerate()
        .filter(|(_idx, p)| **p == two || **p == six)
        .map(|(idx, _p)| idx + 1)
        .product()
}

#[test]
fn test_part_two() {
    assert!(parse_packet("[[[]]]").unwrap().1.cmp(&parse_packet("[1,1,3,1,1]").unwrap().1) == Ordering::Less);

    dbg!(part_two(PuzzleInput::from_file("resources/day13-test.txt").unwrap()));
}

#[test]

fn run_part_two() {
    dbg!(part_two(PuzzleInput::from_file("resources/day13.txt").unwrap()));
}