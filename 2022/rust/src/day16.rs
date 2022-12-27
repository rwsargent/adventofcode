use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::alpha1, combinator::{map, opt}, multi::separated_list0,
    sequence::tuple, IResult,
};
use petgraph::{Graph, stable_graph::NodeIndex, dot::{Dot, Config}};

use crate::reader::PuzzleInput;

#[derive(Debug, Clone)]
struct Node {
    name: String,
    _flow_rate: u32,
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
            _flow_rate: flow_rate,
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
    println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]))
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