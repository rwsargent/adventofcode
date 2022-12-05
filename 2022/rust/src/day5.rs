use text_io::scan;

use crate::reader::PuzzleInput;

#[allow(dead_code)]
fn part_one(instructions: Vec<String>, mut stacks: Vec<Vec<char>>) -> Vec<Vec<char>>{
    for instruction in instructions {
        let (amount, from, to): (usize, usize, usize);
        scan!(instruction.bytes() => "move {} from {} to {}", amount, from, to);

        for _ in 0..amount {
            match stacks[from - 1].pop() {
                Some(out) => stacks[to - 1].push(out),
                None => panic!("Nothing in stack {} on instruction {}", from, instruction)
            }
        }
    }
    stacks
}

#[test]
fn test_part_one() {
    let instr = vec!["move 1 from 0 to 1".to_owned()];
    let stacks = vec![vec!['A', 'B'], vec!['Z', 'Y']];
    let result = part_one(instr, stacks);
    println!{"{:?}", result}
}

#[allow(dead_code)]
fn init_test_stacks() -> Vec<Vec<char>> {
    return vec![
        vec!['Z', 'J', 'G'],
        vec!['Q', 'L', 'R', 'P', 'W', 'F', 'V', 'C'],
        vec!['F', 'P', 'M', 'C', 'L', 'G', 'R'],
        vec!['L', 'F', 'B', 'W', 'P', 'H', 'M'],
        vec!['G', 'C', 'F', 'S', 'V', 'Q'],
        vec!['W', 'H', 'J', 'Z', 'M', 'Q', 'T', 'L'],
        vec!['H', 'F', 'S', 'B', 'V'],
        vec!['F', 'J', 'Z', 'S'],
        vec!['M', 'C', 'D', 'P', 'F', 'H', 'B', 'T'],
    ]
}

#[test]
fn run_part_one() {
    let instructions = PuzzleInput::from_file("resources/day5-instructions.txt").unwrap();
    let stacks = part_one(instructions.as_string_vec(), init_test_stacks());
    for stack in stacks {
        print!("{}", stack.last().unwrap())
    }
}

#[allow(dead_code)]
fn part_two(instructions: Vec<&str>, mut stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for instruction in instructions {
        let (amount, from, to): (usize, usize, usize);
        scan!(instruction.bytes() => "move {} from {} to {}", amount, from, to);

        let tail_range = stacks[from - 1].len() - amount..;
        let mut tail:Vec<char> = stacks[from - 1].drain(tail_range).collect();
        stacks[to - 1].append(&mut tail);
    }
    stacks
}
#[test]
fn run_part_two() {
    let instructions = PuzzleInput::from_file("resources/day5-instructions.txt").unwrap();
    let stacks = part_two(instructions.as_strs(), init_test_stacks());
    for stack in stacks {
        print!("{}", stack.last().unwrap())
    }
    println!()
}