use crate::reader::PuzzleInput;
use std::cmp::max;

type Grid = Vec<Vec<i8>>;

macro_rules! dist {
    (($x1:expr, $y1:expr), ($x2:expr, $y2:expr)) => {
        (((($x1 - $x2) * ($x1 - $x2)) + (($y1 - $y2) * ($y1 - $y2))) as f32).sqrt().trunc() as i32
    };
}

fn build_grid(input: PuzzleInput) -> Grid {
    let mut grid: Vec<Vec<i8>> = Vec::new();
    for line in input.as_string_vec() {
        grid.push(line.split("")
            .filter_map(|height| height.parse::<i8>().ok())
            .collect())
    }
    grid
}

fn new_grid(width: usize, height: usize) -> Grid {
    (0..height).map(|_| (0..width).map(|_| 0).collect()).collect()
}

#[allow(dead_code)]
fn fill_left(trees: &Grid) -> Grid {
    let mut left_memo = new_grid(trees[0].len(), trees.len());
    for r in 0..trees.len() {
        left_memo[r][0] = -1;
        for c in 1..trees[r].len() {
            let prev_memo = *left_memo[r].get( c - 1).unwrap_or(&-1);
            let prev_tree = *trees[r].get( c - 1).unwrap_or(&-1);
            left_memo[r][c] = max(prev_tree, prev_memo);
        }
    }
    left_memo
}

#[allow(dead_code)]
fn fill_right(trees: &Grid) -> Grid {
    let mut right_memo = new_grid(trees[0].len(), trees.len());
    for r in 0..trees.len() {
        for c in (0..trees[r].len()).rev() {
            let prev_right = *right_memo[r].get(c + 1).unwrap_or(&-1);
            let prev_tree = *trees[r].get(c+1).unwrap_or(&-1);
            right_memo[r][c] = max(prev_tree, prev_right);
        }
    }
    right_memo
}

#[allow(dead_code)]
fn fill_top(trees: &Grid) -> Grid {
    let mut top_memo = new_grid(trees[0].len(), trees.len());
    for c in 0..trees[0].len() {
        top_memo[0][c] = -1;
        for r in 1..trees.len() {
            let prev_top = top_memo.get(r-1).map_or(-1, |row| row[c]);
            let prev_tree = trees.get(r-1).map_or(-1, |row| row[c]);
            top_memo[r][c] = max(prev_tree, prev_top);
        }
    }
    top_memo
}

#[allow(dead_code)]
fn fill_bottom(trees: &Grid) -> Grid {
    let mut bot_memo = new_grid(trees[0].len(), trees.len());
    for c in 0..trees[0].len() {
        for r in (0..trees.len()).rev() {
            let prev_bot = bot_memo.get(r+1).map_or(-1, |row| row[c]);
            let prev_tree = trees.get(r+1).map_or(-1, |row| row[c]);
            bot_memo[r][c] = max(prev_tree, prev_bot);
        }
    }
    bot_memo
}

#[allow(dead_code)]
fn part_one(input: PuzzleInput) -> i32 {
    let trees = build_grid(input);
    let (left_memo, right_memo, top_memo, bot_memo) = (fill_left(&trees), fill_right(&trees), fill_top(&trees), fill_bottom(&trees));
    let mut visible_trees = 0;

    for r in 0..trees.len()  {
        for c in 0..trees[0].len() {
            let tree = trees[r][c];
            if tree > left_memo[r][c] || tree > right_memo[r][c] ||  tree > top_memo[r][c] ||  tree > bot_memo[r][c] {
                visible_trees += 1;
            }
        }
    }
    visible_trees
}

#[allow(dead_code)]
fn print_grid(g: &Grid) {
    for i in 0..g.len() {
        for j in 0..g[0].len() {
            print!("{}", g[i][j])
        }
        println!()
    }
}
#[test]
fn test_part_one() {
    let input = PuzzleInput::from_string(
r#"30373
25512
65332
33549
35390"#);
        println!("{}", part_one(input));
    }

#[test]
fn run_part_one() {
    dbg!(part_one(PuzzleInput::from_file("resources/day8.txt").unwrap()));
}

fn calculate_score(trees: &Grid, r: isize, c: isize, dir: (isize, isize)) -> i32 {
    let mut score = 0;
    let main_tree = trees[r as usize][c as usize];
    let mut cursor = (r + dir.0, c + dir.1);
    while let Some(tree) = trees.get(cursor.0 as usize).map(|row| row.get(cursor.1 as usize).unwrap_or(&-1)){
        // println!("Calculating position : {:?}", cursor);
        if *tree == -1 {
            return score;
        } else if *tree >= main_tree {
            return dist!((r, c), (cursor.0, cursor.1));
        }
        cursor = (cursor.0 + dir.0, cursor.1 + dir.1);
        score += 1;
    }
    score
}

fn part_two(input: PuzzleInput) {
    let trees = build_grid(input);
    let mut best_tree_score = -1;
    for r in 0isize..trees.len() as isize {
        for c in 0isize..trees.len() as isize {
            let left = calculate_score(&trees, r, c, (0, -1));
            let right = calculate_score(&trees, r, c, (0, 1));
            let top = calculate_score(&trees, r, c, (-1, 0));
            let bot = calculate_score(&trees, r, c, (1, 0));
            let tree_score = left * right * top * bot;
            best_tree_score = max(tree_score, best_tree_score);
            // print!("{}", tree_score)
        }
        println!();
    }
    dbg!(best_tree_score);
}

#[test]
fn test_part_two() {
    let input = PuzzleInput::from_string(
        r#"30373
        25512
        65332
        33549
        35390"#);
    part_two(input)
}

#[test]
fn run_part_two() {
    part_two(PuzzleInput::from_file("resources/day8.txt").unwrap());
}