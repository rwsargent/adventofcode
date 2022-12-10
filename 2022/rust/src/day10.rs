use crate::reader::PuzzleInput;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    NoOp,
    AddX(i32),
}

fn from_instructions(instructions: Vec<&str>) -> Vec<Op> {
    instructions.iter().map(|instr| {
        let parts: Vec<&str> = instr.split(" ").collect();
        if parts.len() == 1 {
            vec![Op::NoOp]
        } else {
            vec![Op::NoOp, Op::AddX(parts[1].parse().unwrap())]
        }
    }).flatten().collect()
}

fn part_one(input: PuzzleInput) -> i32 {
    let mut x = 1;
    from_instructions(input.as_strs())
        .iter().map(|op| {
            match op {
                Op::NoOp => x,
                Op::AddX(val) => {
                    let temp = x;
                    x+= val;
                    temp
                }
            }
        })
        .enumerate()
        .filter(|(cycle, _x)| (((*cycle as i32) + 1) - 20) % 40 == 0)
        .fold(0, |acc, (cycle, x)| acc + (((cycle as i32) + 1) * x))
}


#[test]
fn small_test_part_one() {
    let input = PuzzleInput::from_file("resources/day10-test.txt").unwrap();
    dbg!(part_one(input));
}

#[test]
fn run_part_one() {
    let input = PuzzleInput::from_file("resources/day10.txt").unwrap();
    dbg!(part_one(input));
}

fn cycle_x(input: Vec<&str>) -> Vec<(usize, i32)>{
    let mut x = 1;
    from_instructions(input)
        .iter().map(|op| {
            match op {
                Op::NoOp => x,
                Op::AddX(val) => {
                    let temp = x;
                    x+= val;
                    temp
                }
            }
        })
        .enumerate()
        .map(|(idx, x)| (idx+1, x))
        .collect()
}

fn part_two(input: PuzzleInput) {
    let xs = cycle_x(input.as_strs());
    for row in 0..6 {
        for pixel in 0..40 {
            let idx: usize = (row*40) + pixel;
            let sprite = xs[idx].1-1..=xs[idx].1+1;
            if sprite.contains(&(pixel as i32)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!()
    }
}

#[test]
fn test_part_two() {
    part_two(PuzzleInput::from_file("resources/day10-test.txt").unwrap());
}


#[test]
fn run_part_two() {
    part_two(PuzzleInput::from_file("resources/day10.txt").unwrap());
}