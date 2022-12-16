use std::{collections::HashMap, fs::File};
use std::io::Write;

use crate::{reader::PuzzleInput, coord::{Coord}};

use itertools::Itertools;
use petgraph::{prelude::Graph, stable_graph::{NodeIndex}};
use petgraph::algo::{dijkstra, astar};


pub fn part_one(input: PuzzleInput) -> i32 {
    let (start, goal, elevation_map) = normalize_input(input);

    let mut g = Graph::<Coord, ()>::new();
    let mut nodes = HashMap::new();
    // Add nodes to graph
    for x in 0..elevation_map.len() {
        for y in  0..elevation_map[x].len() {
            let c = Coord::from_usize(x, y);
            nodes.insert(c, g.add_node(c));
        }
    }
    
    let mut starters = Vec::new();
    // add edges to graph
    for x in 0..elevation_map.len() {
        for y in  0..elevation_map[x].len() {
            let elevation = &elevation_map[x][y];
            let current = Coord{x: x as i32, y: y as i32};
            if *elevation == String::from("a") {
                starters.push(current.clone());
            }
            let node = nodes.get(&Coord::from_usize(x, y)).unwrap();
            let ns = current.cardinal_neighbors()
                .iter()
                .filter(|c| c.x >= 0 && c.x < elevation_map.len() as i32  && c.y >= 0 && c.y < elevation_map[0].len() as i32)
                .filter(|c| (elevation_map[c.x as usize][c.y as usize].as_bytes()[0] as i32) - (elevation.as_bytes()[0] as i32) <= 1)
                .filter_map(|c| nodes.get(c))
                .map(|n| (*node, *n))
                .collect_vec();
            g.extend_with_edges(ns);
        }
    }
    
    // Run BFS
    let start_node = nodes.get(&start).unwrap();
    let goal_node = nodes.get(&goal).unwrap();


    let goal_neighbors:Vec<NodeIndex> = g.neighbors(*goal_node).collect_vec();
    dbg!(goal_neighbors);
    // let path = astar(&g, start_node, |finish| {
    //     let t = finish == goal_node;
    //     if t {
    //         println!("We found the goal");
    //     }
    //     t
    // }, |_| 1, |_| 0);
    let path = dijkstra(&g, *start_node, None, |_| 1);

    let costs = path.iter().map(|(k, v)| (g[*k], v)).collect::<HashMap<Coord, &u32>>();
    
    let mut file = File::create("out.txt").unwrap();
    for x in 0i32..(elevation_map.len() as i32) {
        for y in 0i32..(elevation_map[0].len() as i32) {
            if (Coord{x, y}) == start {
                write!(file, "(S:  0)").unwrap();
            } else if (Coord{x: i32::try_from(x).unwrap(), y: i32::try_from(y).unwrap()}) == goal {
                let cost = costs.get(&Coord { x, y}).unwrap_or(&&0u32);
                write!(file, "(E:{:3})", **cost).unwrap();
            } else { 
                let cost = costs.get(&Coord { x, y}).unwrap_or(&&0u32);
                write!(file, "({}:{:3})", elevation_map[x as usize][y as usize], **cost).unwrap(); 
            }
        }
        write!(file, "\n").unwrap();
    }
    // path.unwrap().0
    // let cost = path.get(&NodeIndex::from(1447)).unwrap();
    // *cost
    
    0
}

fn normalize_input(input: PuzzleInput) -> (Coord, Coord, Vec<Vec<String>>) {
    let (mut start, mut goal) = (None, None);
    let mut grid = input.as_string_grid();
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == "S" {
                start = Some(Coord::from_usize(x, y));
                grid[x][y] = String::from("a");
            } else if grid[x][y] == "E" {
                goal = Some(Coord::from_usize(x, y));
                grid[x][y] = String::from("z");
            }
        }
    }
    (start.unwrap(), goal.unwrap(), grid)
}

#[test]
fn test_part_one() {
    let input = PuzzleInput::from_string(
        r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#);
        dbg!(part_one(input));
}

#[test]
fn run_part_one() {
    let input = PuzzleInput::from_file("resources/day12.txt").unwrap();
    dbg!(part_one(input));
}

pub fn part_two(input: PuzzleInput) -> i32 {
    let (_start, goal, elevation_map) = normalize_input(input);

    let mut g = Graph::<Coord, ()>::new();
    let mut nodes = HashMap::new();
    // Add nodes to graph
    for x in 0..elevation_map.len() {
        for y in  0..elevation_map[x].len() {
            let c = Coord::from_usize(x, y);
            nodes.insert(c, g.add_node(c));
        }
    }
    
    let mut starters = Vec::new();
    // add edges to graph
    for x in 0..elevation_map.len() {
        for y in  0..elevation_map[x].len() {
            let elevation = &elevation_map[x][y];
            let current = Coord{x: x as i32, y: y as i32};
            if *elevation == String::from("a") {
                starters.push(current.clone());
            }
            let node = nodes.get(&Coord::from_usize(x, y)).unwrap();
            let ns = current.cardinal_neighbors()
                .iter()
                .filter(|c| c.x >= 0 && c.x < elevation_map.len() as i32  && c.y >= 0 && c.y < elevation_map[0].len() as i32)
                .filter(|c| (elevation_map[c.x as usize][c.y as usize].as_bytes()[0] as i32) - (elevation.as_bytes()[0] as i32) <= 1)
                .filter_map(|c| nodes.get(c))
                .map(|n| (*node, *n))
                .collect_vec();
            g.extend_with_edges(ns);
        }
    }
    
    // Run BFS
    let goal_node = nodes.get(&goal).unwrap();

    starters.iter()
        .filter_map(|s| nodes.get(s))
        .filter_map(|n| {
            astar(&g,
                *n, 
                |finish| finish == *goal_node, 
                |_| 1, 
                |cur| {
                    *(&g[cur].dist(&g[*goal_node]).trunc()) as i32
                })
            })
        .map(|p| p.0)
        .min()
        .unwrap()
        // dbg!(paths[0].len());
        // 0
} 

#[test]
fn test_part_two() {
    let input = PuzzleInput::from_string(
        r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#);
        dbg!(part_two(input));
}

#[test]
fn run_part_two() {
    let input = PuzzleInput::from_file("resources/day12.txt").unwrap();
    dbg!(part_two(input));
}