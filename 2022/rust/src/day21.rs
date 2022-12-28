use core::panic;
use std::{collections::HashMap, ops::{Add, Sub, Mul, Div}, fmt::Display};

use fraction::Ratio;
use itertools::Itertools;
use nom::{IResult, sequence::{tuple, delimited}, bytes::complete::{take, tag}, character::complete::one_of, branch::alt, Parser, combinator::map};
use strum_macros::EnumString;

use crate::{reader::PuzzleInput};

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    shout: Shout,
}
#[derive(Debug, EnumString, Clone)]
enum Oper {
    #[strum(serialize="+")]
    Add,
    #[strum(serialize="*")]
    Mul,
    #[strum(serialize="/")]
    Div, 
    #[strum(serialize="-")]
    Sub,
}


#[derive(Debug, Clone)]
enum Shout {
    Num(i64),
    Math(String, Oper, String),
    Human,
}

impl From<char> for Oper {
    fn from(c: char) -> Self {
        match c {
            '+' => Oper::Add,
            '-' => Oper::Sub,
            '/' => Oper::Div,
            '*' => Oper::Mul,
            _ => panic!("can't happen")
        }    
    }
}

fn parse_line(input: &str) -> Monkey {
    map(tuple((parse_name, tag(": "), parse_shout)), 
        |(name, _, shout)| Monkey{name: name.to_owned(), shout})(input).unwrap().1
}

fn parse_name(input: &str) -> IResult<&str, &str> {
    take(4usize)(input)
}

fn parse_shout(input: &str) -> IResult<&str, Shout> {
    alt(
        (nom::character::complete::i64.map(|n| Shout::Num(n)),
         tuple((parse_name, delimited(nom::character::complete::char(' '), one_of("*/-+"), nom::character::complete::char(' ')), parse_name)).map(|(l, op, right)| Shout::Math(l.to_owned(), Oper::from(op), right.to_owned()))   
        )
    )(input)
}

pub fn part_one(input: PuzzleInput) -> i64 {
    let monkeies = input.as_lines()
        .map(parse_line)
        .fold(HashMap::new(), |mut monkies, monkey| {
            let cloned = monkey.clone();
            monkies.insert(monkey.name, cloned);
            monkies
        });

        call(&String::from("root"), &monkeies)
}

fn call(name: &String, monkies: &HashMap<String, Monkey>) -> i64 {
    let monkey = monkies.get(name).unwrap();
    match &monkey.shout {
        Shout::Num(n) => *n,
        Shout::Math(l, op, r) => 
        match op {
            Oper::Add => call(&l, monkies) + call(&r, monkies),
            Oper::Sub => call(&l, monkies) - call(&r, monkies),
            Oper::Mul => call(&l, monkies) * call(&r, monkies),
            Oper::Div => call(&l, monkies) / call(&r, monkies),

        },
        _ => panic!("Not used in part one"), 
    }
}

#[test]
fn test_part_one() {
    dbg!(part_one(PuzzleInput::from_string(r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#)));
}

#[test]
fn run_part_one() {
    dbg!(part_one(PuzzleInput::from_file("resources/day21.txt").unwrap()));
}

#[derive(Debug)]
enum Rezult {
    Immdiate(Ratio<i64>),
    Variable(Vec<Term>)
}

impl Display for Rezult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rezult::Immdiate(i) => write!(f, "{}", *i),
            Rezult::Variable(t) => write!(f, "{}", t.iter().map(Term::to_string).join(",")),
        }
    }
}

impl Add for Rezult {
    type Output = Rezult;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Rezult::Immdiate(l), Rezult::Immdiate(r)) => Rezult::Immdiate(l + r),
            (Rezult::Immdiate(i), Rezult::Variable(v)) | 
            (Rezult::Variable(v), Rezult::Immdiate(i)) => {
                let mut c = v.clone();
                c.push(Term{op: Oper::Add, value: i});
                Rezult::Variable(c)
            },
            (Rezult::Variable(_), Rezult::Variable(_)) => panic!("We should only have one unknown per monkey"),
        }
    }
}

impl Sub for Rezult {
    type Output = Rezult;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Rezult::Immdiate(l), Rezult::Immdiate(r)) => Rezult::Immdiate(l - r),
            (Rezult::Immdiate(i), Rezult::Variable(v)) |
            (Rezult::Variable(v), Rezult::Immdiate(i)) => {
                let mut c = v.clone();
                c.push(Term{op:Oper::Sub, value: i});
                Rezult::Variable(c)
            },
            (Rezult::Variable(_), Rezult::Variable(_)) => panic!("We should only have one unknown per monkey"),
        }
    }
}

impl Mul for Rezult {
    type Output = Rezult;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Rezult::Immdiate(l), Rezult::Immdiate(r)) => Rezult::Immdiate(l * r),
            (Rezult::Immdiate(i), Rezult::Variable(v)) | 
            (Rezult::Variable(v), Rezult::Immdiate(i)) => {
                let mut c = v.clone();
                c.push(Term{op: Oper::Mul, value: i});
                Rezult::Variable(c)
            },
            (Rezult::Variable(_), Rezult::Variable(_)) => panic!("We should only have one unknown per monkey"),
        }
    }
}

impl Div for Rezult {
    type Output = Rezult;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Rezult::Immdiate(l), Rezult::Immdiate(r)) => Rezult::Immdiate(l / r),
            (Rezult::Variable(v), Rezult::Immdiate(i)) |
            (Rezult::Immdiate(i), Rezult::Variable(v)) => {
                let mut c = v.clone();
                c.push(Term{op: Oper::Div, value: i});
                Rezult::Variable(c)
            }, 
            (Rezult::Variable(_), Rezult::Variable(_)) => panic!("We should only have one unknown per monkey"),
        }
    }
}

#[derive(Debug, Clone)]
struct Term {
    op: Oper,
    value: fraction::Ratio<i64>, 
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", match self.op {
            Oper::Add => "+",
            Oper::Mul => "*",
            Oper::Div => "/",
            Oper::Sub => "-",
        }, self.value)
    }
}

fn call_term(name: &String, monkies: &HashMap<String, Monkey>) -> Rezult {
    let monkey = monkies.get(name).unwrap();
    let s = match &monkey.shout {
        Shout::Human => {
            Rezult::Variable(vec!(Term{op:Oper::Add, value: Ratio::from(0)}))
        },
        Shout::Num(n) => Rezult::Immdiate(Ratio::from(*n)),
        Shout::Math(l, op, r) => {
            match op {
                Oper::Add => call_term(&l, monkies) + call_term(&r, monkies),
                Oper::Sub => call_term(&l, monkies) - call_term(&r, monkies),
                Oper::Mul => call_term(&l, monkies) * call_term(&r, monkies),
                Oper::Div => call_term(&l, monkies) / call_term(&r, monkies),
            }
        }
    };
    // println!("{} -> {}", name, s);
    s
}

pub fn part_two(input: PuzzleInput) -> i64 {
    let mut monkies = input.as_lines()
        .map(parse_line)
        .fold(HashMap::new(), |mut monkies, monkey| {
            let cloned = monkey.clone();
            monkies.insert(monkey.name, cloned);
            monkies
        });
    monkies.entry("humn".to_owned()).and_modify(|h| h.shout = Shout::Human);

    let root = monkies.get(&String::from("root")).unwrap();
    if let Shout::Math(l_monkey, _, r_monkey) = &root.shout {
        let (l, r) = (call_term(&l_monkey, &monkies), call_term(&r_monkey, &monkies));
        match (l, r) {
            (Rezult::Immdiate(i), Rezult::Variable(v)) |
            (Rezult::Variable(v), Rezult::Immdiate(i)) => {
                println!("{} = {}", i, v.iter().map(Term::to_string).join(","));
                let res = v.iter().rev().fold(i, |acc, t| {
                    match t.op {
                        Oper::Add => acc - t.value,
                        Oper::Mul => acc / t.value,
                        Oper::Div => acc * t.value,
                        Oper::Sub => acc + t.value,
                    }
                });
                println!("Answer: {}", res);
                res.to_integer()
            },
            (Rezult::Immdiate(_), Rezult::Immdiate(_)) | 
            (Rezult::Variable(_), Rezult::Variable(_)) => panic!("We only can have one variable"),
        }
    } else {
        0
    }
}

#[test]
fn test_part_two() {
    dbg!(part_two(PuzzleInput::from_string(r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#)));
}

#[test]
fn run_part_two() {
    dbg!(part_two(PuzzleInput::from_file("resources/day21.txt").unwrap()));
}

pub fn bsearch_part_two(input: PuzzleInput) -> i64 {
    let mut monkies = input.as_lines()
        .map(parse_line)
        .fold(HashMap::new(), |mut monkies, monkey| {
            let cloned = monkey.clone();
            monkies.insert(monkey.name, cloned);
            monkies
        });

        
        let (mut low, mut high) = (-7560831729514i64, 7560831729514i64);
        while low <= high {
            let guess = ((high - low) / 2) + low; 
            monkies.entry("humn".to_owned()).and_modify(|h| h.shout = Shout::Num(guess));
            let root = monkies.get(&String::from("root")).unwrap();
            if let Shout::Math(l_monkey, _, r_monkey) = &root.shout {
                let (left_tree, right_tree) = (call(l_monkey, &monkies), call(r_monkey, &monkies));
                println!("{} ? {}", left_tree, right_tree);
                if left_tree < right_tree {
                    high = guess -1;
                } else if left_tree > right_tree {
                    low = guess + 1;
                } else {
                    return guess;
                }
            }
        }
        return 0;
}

#[test]
fn run_bsearch_part_two() {
    dbg!(bsearch_part_two(PuzzleInput::from_file("resources/day21.txt").unwrap()));
}