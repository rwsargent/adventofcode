use crate::{puzzle::{Puzzle}, reader};

pub struct DayOne {
    calories: Vec<Vec<i32>>
}

impl DayOne {
    pub fn new() -> Self {
        let input = reader::PuzzleInput::from_file("resources/day1.txt").expect("Expected file at resources/day1.txt").as_string();
        let calories = input.split("\n\n")
            .map(|group| group.split("\n")
            .filter_map(|calorie| calorie.parse::<i32>().ok()).collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>();
        DayOne {
            calories
        }
    }
}

impl Puzzle<i32> for DayOne {
    fn run_part_one(&self) -> Result<i32, String> {
        let max = self.calories.iter().map(|elf| elf.iter().sum::<i32>()).max();
        max.ok_or(String::from("No max found"))
    }

    fn run_part_two(&self) -> Result<i32, String> {
       let mut calories = self.calories.iter()
        .map(|elf| elf.iter().sum::<i32>())
        .collect::<Vec<i32>>();
        calories.sort_by(|a, b| b.cmp(a));
        Ok(calories.iter().take(3).sum())
    }
}

#[test]
fn test_day_1() {
    let p = DayOne {
        calories: vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000], vec![7000, 8000, 9000]]
    };
    let max = p.run_part_one().unwrap();
    assert_eq!(max, 24000);
}

#[test]
fn test_day_2() {
    let p = DayOne {
        calories: vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000], vec![7000, 8000, 9000], vec![10000]]
    };
    let top_three = p.run_part_two().unwrap();
    assert_eq!(top_three, 45000)
}