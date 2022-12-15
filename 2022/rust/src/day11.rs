use std::collections::{VecDeque, HashMap};

use itertools::Itertools;

pub struct Monkey {
    items: VecDeque<i64>,
    op: Box<dyn Fn(i64) -> i64>,
    test: i64,
    pass: usize,
    fail: usize,
    inspect_count: usize,
}


pub fn part_one(mut monkies: Vec<Monkey>) {
    let clamp:i64 = monkies.iter().map(|m| m.test).product();
    for _round in 0..10_000 {
        for monkey_idx in 0..monkies.len() {
            let mut passes = HashMap::new();
            {
                let monkey = &mut monkies[monkey_idx];
                monkey.inspect_count += monkey.items.len();
                while let Some(item) = monkey.items.pop_front() {
                    let updated = (monkey.op)(item); // Change to  / 3 for part one
                    let receiver = if updated % monkey.test == 0 {
                        monkey.pass
                    } else {
                        monkey.fail
                    };
                    passes.entry(receiver).or_insert(Vec::new()).push(updated);
                }
            }
            // Split out here to fight borrow checker
            for (k, v) in passes {
                monkies[k].items.extend(v.iter().map(|i| i % clamp));
            }
        }
    }

    dbg!(monkies.iter()
        .map(|m| m.inspect_count)
        .sorted()
        .rev()
        .take(2)
        .product::<usize>());
}

#[test]
fn test_part_one() {
    let monkies = vec![Monkey{
        items: VecDeque::from([79, 98]),
        op: Box::new(|old| old * 19),
        test: 23,
        pass: 2,
        fail: 3,
        inspect_count: 0,
    },Monkey{
        items: VecDeque::from([54, 65, 75, 74]),
        op: Box::new(|old| old + 6),
        test: 19,
        pass: 2,
        fail: 0,
        inspect_count: 0,
    },Monkey{
        items: VecDeque::from([79, 60, 97]),
        op: Box::new(|old| old * old),
        test: 13,
        pass: 1,
        fail: 3,
        inspect_count: 0,
    },Monkey{
        items: VecDeque::from([74]),
        op: Box::new(|old| old + 3),
        test: 17,
        pass: 0,
        fail: 1,
        inspect_count: 0,
    },];

    part_one(monkies);
    // part_tw(monkeys);
}

#[test]
fn run_part_one() {
    let monkies = vec![Monkey{
        items: VecDeque::from([91, 66]),
        op: Box::new(|old| old * 13),
        test: 19,
        pass: 6,
        fail: 2,
        inspect_count: 0,
    },
    Monkey{
        items: VecDeque::from([78, 97, 59]),
        op: Box::new(|old| old + 7),
        test: 5 ,
        pass: 0,
        fail: 3,
        inspect_count: 0, 
    },Monkey{
        items: VecDeque::from([57, 59, 97, 84, 72, 83, 56, 76]),
        op: Box::new(|old| old + 6),
        test: 11,
        pass: 5,
        fail: 7,
        inspect_count: 0, 
    },Monkey{
        items: VecDeque::from([81, 78, 70, 58, 84]),
        op: Box::new(|old| old + 5),
        test: 17,
        pass: 6,
        fail: 0,
        inspect_count: 0, 
    },Monkey{
        items: VecDeque::from([60]),
        op: Box::new(|old| old + 8),
        test: 7,
        pass: 1,
        fail: 3,
        inspect_count: 0, 
    },
    Monkey{
        items: VecDeque::from([57, 69, 63, 75, 62, 77, 72]),
        op: Box::new(|old| old * 5),
        test: 13,
        pass: 7,
        fail: 4,
        inspect_count: 0, 
    },Monkey{
        items: VecDeque::from([73, 66, 86, 79, 98, 87]),
        op: Box::new(|old| old * old),
        test: 3,
        pass: 5,
        fail: 2,
        inspect_count: 0, 
    },Monkey{
        items: VecDeque::from([95, 89, 63, 67]),
        op: Box::new(|old| old + 2),
        test: 2,
        pass: 1,
        fail: 4,
        inspect_count: 0, 
    }];

    part_one(monkies);
}