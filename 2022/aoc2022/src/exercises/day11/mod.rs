use log::info;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fmt;

pub fn process_input(filedata: &String) -> Troop {
    let mut monkeys: VecDeque<RefCell<Monkey>> = VecDeque::new();
    let mut monkey_idx = 0;
    let mut filedata_lines = filedata.lines();
    loop {
        if let Some(line) = filedata_lines.next() {
            if line == "" {
                // monkey seperator - just pass
            } else if line.chars().next().unwrap() == 'M' {
                let cur_id = line
                    .split(" ")
                    .nth(1)
                    .unwrap()
                    .split(":")
                    .nth(0)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let mut items: VecDeque<Item> = VecDeque::new();
                let mut raw_items = filedata_lines
                    .next()
                    .unwrap()
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split(",")
                    .map(|x| x.trim().parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                for item in raw_items {
                    items.push_back(Item { worry: item });
                }

                let op = Operation::from(filedata_lines.next().unwrap());

                let mut test_lines: Vec<&str> = vec![];
                test_lines.push(filedata_lines.next().unwrap());
                test_lines.push(filedata_lines.next().unwrap());
                test_lines.push(filedata_lines.next().unwrap());

                monkeys.push_back(RefCell::new(Monkey {
                    items: items.clone(),
                    operation: op,
                    test: Test::from(test_lines),
                    inspection_count: 0,
                }));
            } else {
                unreachable!("Monkey line not found")
            }
        } else {
            break;
        }
    }
    Troop { monkeys: monkeys }
}

pub fn run_part1(filedata: &String) -> usize {
    let mut troop = process_input(filedata);
    let result = part1(&mut troop);
    info!("Day 11 Part 1 Answer: {}", result);
    result
}

pub fn part1(troop: &mut Troop) -> usize {
    for round in 0..20 {
        troop.process_items(1);
    }
    let mut first = 0;
    let mut second = 0;
    for monkey_idx in 0..troop.monkeys.len() {
        let cur_count = troop.monkeys[monkey_idx].borrow().inspection_count;
        if cur_count > first {
            second = first;
            first = cur_count;
        } else if cur_count > second {
            second = cur_count;
        }
    }
    first * second
}

pub fn run_part2(filedata: &String) -> usize {
    let mut troop = process_input(filedata);
    let result = part2(&mut troop);
    info!("Day 11 Part 2 Answer: {}", result);
    result
}

pub fn part2(troop: &mut Troop) -> usize {
    for round in 0..10000 {
        troop.process_items(2);
    }
    let mut first = 0;
    let mut second = 0;
    for monkey_idx in 0..troop.monkeys.len() {
        let cur_count = troop.monkeys[monkey_idx].borrow().inspection_count;
        if cur_count > first {
            second = first;
            first = cur_count;
        } else if cur_count > second {
            second = cur_count;
        }
    }
    println!("{} {}", first, second);
    first * second
}

#[cfg(test)]
mod tests {
    use crate::common::read_file;
    use crate::exercises::day11::run_part1;
    use crate::exercises::day11::run_part2;
    #[test]
    fn test() {}
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
struct Item {
    worry: usize,
}

#[derive(Debug, Clone)]
enum OperationType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
enum Operand {
    Old,
    Value(usize),
}

#[derive(Debug, Clone)]
struct Operation {
    left: Operand,
    right: Operand,
    op_type: OperationType,
}

impl From<&str> for Operation {
    fn from(line: &str) -> Self {
        let mut raw_operation = line.split("=").map(|x| x.trim()).collect::<Vec<&str>>()[1]
            .split(" ")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        // left
        let mut left: Operand;
        if raw_operation[0] == "old" {
            left = Operand::Old;
        } else {
            left = Operand::Value(raw_operation[0].trim().parse::<usize>().unwrap());
        }

        // operation type
        let mut op_type: OperationType;
        match raw_operation[1] {
            "+" => op_type = OperationType::Add,
            "-" => op_type = OperationType::Subtract,
            "*" => op_type = OperationType::Multiply,
            "/" => op_type = OperationType::Divide,
            _ => unreachable!("invalid raw operation type detected"),
        }

        // right
        let mut right: Operand;
        if raw_operation[2] == "old" {
            right = Operand::Old;
        } else {
            right = Operand::Value(raw_operation[2].trim().parse::<usize>().unwrap());
        }

        Operation {
            left: left,
            right: right,
            op_type: op_type,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Test {
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
}

impl From<Vec<&str>> for Test {
    fn from(lines: Vec<&str>) -> Self {
        Test {
            divisible_by: lines[0]
                .split("by")
                .nth(1)
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap(),
            if_true: lines[1]
                .split("monkey")
                .nth(1)
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap(),
            if_false: lines[2]
                .split("monkey")
                .nth(1)
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    test: Test,
    inspection_count: usize,
}

#[derive(Debug, Clone)]
pub struct Troop {
    monkeys: VecDeque<RefCell<Monkey>>,
}

impl fmt::Display for Troop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut msg = String::default();
        for monkey_idx in 0..self.monkeys.len() {
            msg = format!("\n{}Monkey {}: ", msg, monkey_idx);
            for item in &self.monkeys[monkey_idx].borrow().items {
                msg = format!("{} {},", msg, item.worry);
            }
            msg = format!("{}\n", msg);
        }
        write!(f, "{}", msg)
    }
}

impl Troop {
    fn process_items(&mut self, round: usize) {
        for monkey_idx in 0..self.monkeys.len() {
            //            let monkey = self.monkeys[monkey_idx];
            let items_len = self.monkeys[monkey_idx].borrow().items.len();
            let mut item_idx = 0;
            loop {
                // check the items len now that the size has changed
                if item_idx >= self.monkeys[monkey_idx].borrow().items.len() {
                    break;
                }

                // inspect item
                let mut left: usize;
                match &self.monkeys[monkey_idx].borrow().operation.left {
                    Operand::Old => {
                        left = self.monkeys[monkey_idx].borrow().items[item_idx]
                            .worry
                            .clone()
                    }
                    Operand::Value(value) => left = value.clone(),
                }
                let mut right: usize;
                match &self.monkeys[monkey_idx].borrow().operation.right {
                    Operand::Old => {
                        right = self.monkeys[monkey_idx].borrow().items[item_idx]
                            .worry
                            .clone()
                    }
                    Operand::Value(value) => right = value.clone(),
                }
                let cur_op_type = self.monkeys[monkey_idx].borrow().operation.op_type.clone();
                match cur_op_type {
                    OperationType::Add => {
                        self.monkeys[monkey_idx].borrow_mut().items[item_idx].worry = left + right;
                    }
                    OperationType::Subtract => {
                        self.monkeys[monkey_idx].borrow_mut().items[item_idx].worry = left - right;
                    }
                    OperationType::Multiply => {
                        self.monkeys[monkey_idx].borrow_mut().items[item_idx].worry = left * right;
                    }
                    OperationType::Divide => {
                        self.monkeys[monkey_idx].borrow_mut().items[item_idx].worry = left / right;
                    }
                }

                // process relief for lack of damage
                if round == 1 {
                    self.monkeys[monkey_idx].borrow_mut().items[item_idx].worry /= &3;
                }

                // make a copy of the item before moving forward
                let cur_item = self.monkeys[monkey_idx].borrow().items[item_idx].clone();

                // test item
                if &self.monkeys[monkey_idx].borrow().items[item_idx].worry
                    % &self.monkeys[monkey_idx].borrow().test.divisible_by
                    == 0
                {
                    let div = self.monkeys[monkey_idx].borrow().test.divisible_by.clone();
                    self.monkeys[monkey_idx].borrow_mut().items[item_idx].worry /= &div;
                    let new_monkey_idx = self.monkeys[monkey_idx].borrow().test.if_true.clone();
                    self.monkeys[new_monkey_idx]
                        .borrow_mut()
                        .items
                        .push_back(cur_item);

                    self.monkeys[monkey_idx].borrow_mut().items.pop_front();
                } else {
                    let div = self.monkeys[monkey_idx].borrow().test.divisible_by.clone();
                    self.monkeys[monkey_idx].borrow_mut().items[item_idx].worry /= &div;
                    let new_monkey_idx = self.monkeys[monkey_idx].borrow().test.if_false.clone();
                    self.monkeys[new_monkey_idx]
                        .borrow_mut()
                        .items
                        .push_back(cur_item);

                    self.monkeys[monkey_idx].borrow_mut().items.pop_front();
                }

                self.monkeys[monkey_idx].borrow_mut().inspection_count += 1;
            }
        }
    }
}
