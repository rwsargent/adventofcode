use itertools::Itertools;

use crate::reader::PuzzleInput;

pub fn part_one(input: PuzzleInput) -> i64 {
    let mut encrpyted = input
        .as_lines()
        .filter_map(|n| n.parse::<i64>().ok())
        .enumerate()
        .collect::<Vec<_>>();
    let modulo = (encrpyted.len() - 1) as i64;

    for idx in 0..encrpyted.len() {
        // Find num with linear search
        let (current_pos, element) = encrpyted
            .iter()
            .find_position(|(position, _num)| *position == idx)
            .unwrap();

        // calculate new position
        // let new_pos = ((current_pos as i64 + element.1 + modulo) % modulo) as usize;
        let new_pos = if element.1 > 0 {
            (current_pos as i64 + element.1) % modulo
        } else {
            // If negative, add to position and take a modulo,
            // which can result in any number between (-modulo, modulo).
            // Add modulo to get between [0, 2*modulo), the modulo again 
            // to get [0, modulo)
            (((current_pos as i64 + element.1) % modulo) + modulo) % modulo
        } as usize;

        let mut mid = encrpyted
            .iter()
            .take(current_pos)
            .chain(encrpyted.iter().skip(current_pos + 1))
            .map(|e| *e)
            .collect::<Vec<_>>();
        mid.splice(new_pos..new_pos, [*element]);
        encrpyted = mid;
    }

    let (zero_pos, _) = encrpyted.iter().find_position(|(_pos, num)| *num == 0).unwrap();
    encrpyted.rotate_left(zero_pos);

    let len = encrpyted.len();
    encrpyted[(1000 % len) as usize].1 + encrpyted[(2000 % len) as usize].1 + encrpyted[(3000 % len) as usize].1
}

#[test]
fn test_part_one() {
    let test_input = PuzzleInput::from_string(r#"1
2
-3
3
-2
0
4"#);
    dbg!(part_one(test_input));
}

#[test]
fn run_part_one() {
    dbg!(part_one(PuzzleInput::from_file("resources/day20.txt").unwrap()));
}

pub fn part_two(input: PuzzleInput) -> i64 {
    // 811589153
    let mut encrpyted = input
        .as_lines()
        .filter_map(|n| n.parse::<i64>().ok())
        .map(|n| n * 811589153)
        .enumerate()
        .collect::<Vec<_>>();
    let modulo = (encrpyted.len() - 1) as i64;

    for _ in 0..10 {

        for idx in 0..encrpyted.len() {
            // Find num with linear search
            let (current_pos, element) = encrpyted
                .iter()
                .find_position(|(position, _num)| *position == idx)
                .unwrap();
    
            // calculate new position
            // let new_pos = ((current_pos as i64 + element.1 + modulo) % modulo) as usize;
            let new_pos = if element.1 > 0 {
                (current_pos as i64 + element.1) % modulo
            } else {
                // If negative, add to position and take a modulo,
                // which can result in any number between (-modulo, modulo).
                // Add modulo to get between [0, 2*modulo), the modulo again 
                // to get [0, modulo)
                (((current_pos as i64 + element.1) % modulo) + modulo) % modulo
            } as usize;
    
            let mut mid = encrpyted
                .iter()
                .take(current_pos)
                .chain(encrpyted.iter().skip(current_pos + 1))
                .map(|e| *e)
                .collect::<Vec<_>>();
            mid.splice(new_pos..new_pos, [*element]);
            encrpyted = mid;
        }
    }

    let (zero_pos, _) = encrpyted.iter().find_position(|(_pos, num)| *num == 0).unwrap();
    encrpyted.rotate_left(zero_pos);

    let len = encrpyted.len();
    encrpyted[(1000 % len) as usize].1 + encrpyted[(2000 % len) as usize].1 + encrpyted[(3000 % len) as usize].1
}


#[test]
fn test_part_two() {
    let test_input = PuzzleInput::from_string(r#"1
2
-3
3
-2
0
4"#);
    dbg!(part_two(test_input));    
}


#[test]
fn run_part_two() {
    dbg!(part_two(PuzzleInput::from_file("resources/day20.txt").unwrap()));
}
