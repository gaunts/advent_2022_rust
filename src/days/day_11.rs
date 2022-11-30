use std::collections::VecDeque;

use itertools::Itertools;

#[derive(Debug)]
enum Operation {
    Mul(usize),
    Add(usize),
    Pow
}

impl Operation {
    pub fn new(line: Vec<&str>) -> Operation {
        match line[4] {
            "+" => Operation::Add(line[5].parse().unwrap()),
            "*" => match line[5] {
                "old" => Operation::Pow,
                _ => Operation::Mul(line[5].parse().unwrap())
            }
            _ => panic!("not cool : {}", line[4])
        }
    }
}

#[derive(Debug)]
struct MonkeyTest {
    value: usize,
    true_target: usize,
    false_target: usize
}

#[derive(Debug)]
struct Monkey {
    items_inspected: usize,
    held_items: VecDeque<usize>,
    operation: Operation,
    test: MonkeyTest
}

impl Monkey {
    pub fn inspect(&mut self, modulo: Option<usize>) -> Option<(usize,  usize)> {
        if self.held_items.len() == 0 {
            return None;
        }

        self.items_inspected += 1;
        let mut item = self.held_items.pop_front().unwrap();
        item = match self.operation {
            Operation::Add(v) => item + v,
            Operation::Mul(v) => item * v,
            Operation::Pow => item * item
        };

        item = match modulo {
            Some(modulo) => item % modulo,
            None => item / 3
        };

        if item % self.test.value == 0 {
            Some((item, self.test.true_target))
        } else {
            Some((item, self.test.false_target))
        }
    }
}

fn get_numbers(line: Vec<&str>) -> VecDeque<usize> {
    line.iter().skip(2).map(|s| s.replace(",", "").parse::<usize>().unwrap()).collect()
}

fn run_part(monkeys: &mut Vec<Monkey>, _part: u8) {
    let max_rounds = if _part == 1 { 20 } else { 10000 };
    let mut modulo = 1;
    for value in  monkeys.iter().map(|m| m.test.value) {
        modulo *= value;
    }
    for _ in 0..max_rounds {
        for i in 0..monkeys.len() {
            while let Some((value, target)) = monkeys[i].inspect(if _part == 1 { None } else { Some(modulo) }) {
                monkeys[target].held_items.push_back(value);
            }
        }
    }

    let mut inspects = monkeys.iter().map(|m| m.items_inspected).sorted().rev();
    println!("{}", inspects.next().unwrap() * inspects.next().unwrap());
}

pub fn run(_input: &String) {
    let mut monkeys: Vec<Monkey> = vec!();
    let mut lines = _input.lines().map(|line| line.trim().split(" ").collect::<Vec<&str>>());

    while lines.next().is_some() {
        monkeys.push(Monkey {
            held_items: get_numbers(lines.next().unwrap()),
            items_inspected: 0,
            operation: Operation::new(lines.next().unwrap()),
            test: MonkeyTest {
                value: lines.next().unwrap()[3].parse().unwrap(),
                true_target: lines.next().unwrap()[5].parse().unwrap(),
                false_target: lines.next().unwrap()[5].parse().unwrap()
            }
        });
        lines.next(); // stupid blank line
    }

    run_part(&mut monkeys, 1);
    run_part(&mut monkeys, 2);
}
