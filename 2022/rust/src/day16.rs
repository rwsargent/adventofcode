use std::collections::{HashMap, BTreeSet};

use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::alpha1, combinator::{map, opt}, multi::separated_list0,
    sequence::tuple, IResult,
};
use petgraph::{Graph, stable_graph::NodeIndex, dot::{Dot, Config}};

use crate::reader::PuzzleInput;

#[derive(Debug, Clone)]
struct Node {
    name: String,
    id: u32,
    flow_rate: u32,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct MemoKey {
    valves_open: BTreeSet<String>,
    current: NodeIndex,
    time_remaining: u32,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct MemoKeyTwo {
    valves_open: u64,
    current: NodeIndex,
    time_remaining: u32,
    player: bool
}

fn parse_neighbors(input: &str) -> IResult<&str, Vec<&str>> {
    map(tuple((tag(" tunnel"), opt(tag("s")), tag(" lead"), opt(tag("s")), tag(" to valve"), opt(tag("s")), tag(" "), separated_list0(tag(", "), alpha1))),
        |(_, _, _, _, _, _, _,  ns)| ns)(input)
}


fn parse_node(input: &str) -> IResult<&str, Node> {
    // Valve AA has flow rate=0;
    map(
        tuple((
            tag("Valve "),
            alpha1,
            tag(" has flow rate="),
            nom::character::complete::u32,
            tag(";"),
        )),
        |(_, name, _, flow_rate, _)| Node {
            name: String::from(name),
            id: 0,
            flow_rate: flow_rate,
        },
    )(input)
}

fn parse_line(line: &str) -> IResult<&str, (Node, Vec<&str>)> {
    tuple((parse_node, parse_neighbors))(line)
}

fn make_graph(input: PuzzleInput) -> Graph<Node, ()> {
    let mut g: Graph<_, _, _, _> = Graph::<Node, ()>::new();
    let nodes_neighbors: Vec<(Node, Vec<&str>)> = input.as_lines()
        .map(parse_line)
        .map(|r| r.unwrap().1)
        .sorted_by(|(l, _), (r, _)| r.flow_rate.cmp(&l.flow_rate))
        .enumerate()
        .map(|(i, (node, neighbors))| (Node{
            name: node.name,
            id: i as u32,
            flow_rate: node.flow_rate,
        }, neighbors))
        .collect();

    let mut temp_map : HashMap<String, (Node, &Vec<&str>, NodeIndex)> = HashMap::new();
    for (node, neighbors) in &nodes_neighbors {
        let node_index = g.add_node((*node).clone());
        temp_map.insert(node.name.clone(), ((*node).clone(), neighbors, node_index));
    }

    // add edges
    for n_idx in g.node_indices() {
        let n = g[n_idx].clone();
        for neighbor in temp_map.get(&n.name).unwrap().1
            .iter()
            .map(|neighbor| temp_map.get(&String::from(*neighbor)).unwrap().2) {
            g.add_edge(n_idx, neighbor, ());
        }
    }
    g
}

pub fn part_one(input: PuzzleInput) {
    let g = make_graph(input);
    let start = g.node_indices().find(|n_idx| g[*n_idx].name == "AA".to_owned()).unwrap();
    dbg!(search(start, 30, BTreeSet::new(), &g, &mut HashMap::new()));
}

fn search(current: NodeIndex, time_remaining: u32, valves: BTreeSet<String>, g: & Graph<Node, ()>, memo: & mut HashMap<MemoKey, u32>) -> u32 {
    if time_remaining == 0 {
        return 0;
    }

    let memo_key = MemoKey{
        valves_open: valves,
        current,
        time_remaining,
    };

    if memo.contains_key(&memo_key) {
        return memo[&memo_key]
    }

    let added_flow = g[current].flow_rate * (time_remaining - 1);
    let mut answer = 0;
    if g[current].flow_rate > 0 && !memo_key.valves_open.contains(&g[current].name) {
        let mut updated_valves = memo_key.valves_open.clone();
        updated_valves.insert(g[current].name.clone());
        answer = answer.max(added_flow + search(current, time_remaining -1, updated_valves, g, memo));
    }
    for neighbor in g.neighbors(current) {
        answer = answer.max(search(neighbor, time_remaining -1, memo_key.valves_open.clone(), g, memo));
    }

    memo.insert(memo_key, answer);
    answer
}

#[test]
fn test_draw_graph() {
    part_one(PuzzleInput::from_file("resources/day16-test.txt").unwrap());
}


#[test]
fn test_parse_line() {
    let _r = parse_line("Valve HH has flow rate=22; tunnel leads to valve GG");
    // dbg!(r);
}

#[test]
fn run_part_one() {
    part_one(PuzzleInput::from_file("resources/day16.txt").unwrap());
}



fn two_player_search(current: NodeIndex, time_remaining: u32, valves: u64, is_player_one: bool, g: & Graph<Node, ()>, memo: & mut HashMap<MemoKeyTwo, u32>) -> u32 {
    if time_remaining == 0 {
        if is_player_one {
            let start = g.node_indices().find(|n_idx| g[*n_idx].name == "AA".to_owned()).unwrap();
            return two_player_search(start, 26, valves, false, g, memo);
        }
        return 0;
    }

    let memo_key = MemoKeyTwo{
        valves_open: valves,
        current,
        time_remaining,
        player: is_player_one,
    };

    if memo.contains_key(&memo_key) {
        return memo[&memo_key]
    }

    let mut answer = 0;
    if g[current].flow_rate > 0 && !((memo_key.valves_open & (1 << &g[current].id)) > 0) {
        let added_flow = g[current].flow_rate * (time_remaining - 1);
        let updated_valves = memo_key.valves_open | ( 1 << &g[current].id);
        answer = answer.max(added_flow + two_player_search(current, time_remaining -1, updated_valves, is_player_one, g,  memo));
    }
    for neighbor in g.neighbors(current) {
        answer = answer.max(two_player_search(neighbor, time_remaining -1, memo_key.valves_open, is_player_one, g, memo));
    }

    memo.insert(memo_key, answer);
    answer
}

pub fn part_two(input: PuzzleInput) -> u32 {
    let g = make_graph(input);
    let start = g.node_indices().find(|n_idx| g[*n_idx].name == "AA".to_owned()).unwrap();
    let mut memo = HashMap::new();

    two_player_search(start, 26, 0, true, &g, &mut memo)
}
#[test]

fn test_part_two() {
    dbg!(part_two(PuzzleInput::from_file("resources/day16-test.txt").unwrap()));
}

#[test]
fn run_part_two() {
    part_two(PuzzleInput::from_file("resources/day16.txt").unwrap());
}

#[test]
fn show_cavern() {
    let real = make_graph(PuzzleInput::from_file("resources/day16.txt").unwrap());
    println!("");
    println!("{:?}", Dot::with_config(&real, &[Config::EdgeNoLabel]));

}