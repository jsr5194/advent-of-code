use log::info;
use std::fmt;

pub fn process_input(filedata: &String) -> Warehouse {
    Warehouse::from(filedata)
}

pub fn run_part1(filedata: &String) -> String {
    let mut warehouse = process_input(filedata);
    let result = part1(&mut warehouse);
    info!("Day 5 Part 1 Answer: {}", result);
    result
}

pub fn part1(warehouse: &mut Warehouse) -> String {
    //let instructions = warehouse.instructions.clone();
    //for instruction in instructions {
    for instruction in &mut warehouse.instructions {
        for count in 0..=instruction.count {
            let cur_crate = warehouse.stacks[instruction.src]
                .crates
                .pop()
                .unwrap()
                .clone();
            &mut warehouse.stacks[instruction.dst].crates.push(cur_crate);
        }
    }

    let mut result = String::default();
    for stack in &mut warehouse.stacks {
        result = format!(
            "{}{}",
            result,
            stack.crates[stack.crates.len() - 1].contents
        );
    }
    result
}

pub fn run_part2(filedata: &String) -> String {
    let mut warehouse = process_input(filedata);
    let result = part2(&mut warehouse);
    info!("Day 5 Part 2 Answer: {}", result);
    result
}

pub fn part2(warehouse: &mut Warehouse) -> String {
    let instructions = warehouse.instructions.clone();
    for instruction in instructions {
        let mut tmp_stack: Vec<Crate> = vec![];
        for count in 0..=instruction.count {
            tmp_stack.push(
                warehouse.stacks[instruction.src]
                    .crates
                    .pop()
                    .unwrap()
                    .clone(),
            );
        }
        for cur_crate in tmp_stack.iter().rev() {
            &mut warehouse.stacks[instruction.dst]
                .crates
                .push(cur_crate.clone());
        }
    }

    let mut result = String::default();
    for stack in &mut warehouse.stacks {
        result = format!(
            "{}{}",
            result,
            stack.crates[stack.crates.len() - 1].contents
        );
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::common::read_file_notrim;
    use crate::exercises::day5::run_part1;
    use crate::exercises::day5::run_part2;
    #[test]
    fn test() {
        assert_eq!(
            run_part1(&read_file_notrim("./src/exercises/day5/input_test.txt")),
            "CMZ"
        );
        assert_eq!(
            run_part1(&read_file_notrim("./src/exercises/day5/input.txt")),
            "TBVFVDZPN"
        );

        assert_eq!(
            run_part2(&read_file_notrim("./src/exercises/day5/input_test.txt")),
            "MCD"
        );
        assert_eq!(
            run_part2(&read_file_notrim("./src/exercises/day5/input.txt")),
            "VLCWHTDSZ"
        );
    }
}

enum WarehouseParsingState {
    Drawing,
    EmptyLine,
    Instructions,
}

#[derive(Debug, Default, Clone)]
struct Instruction {
    count: usize,
    src: usize,
    dst: usize,
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        // format: move 1 from 2 to 1
        let mut split_instruction: Vec<&str> = input.split(" ").collect::<Vec<&str>>();

        Instruction {
            count: split_instruction[1].parse::<usize>().unwrap() - 1,
            src: split_instruction[3].parse::<usize>().unwrap() - 1,
            dst: split_instruction[5].parse::<usize>().unwrap() - 1,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Crate {
    contents: char,
}

#[derive(Debug, Default, Clone)]
struct Stack {
    id: usize,
    crates: Vec<Crate>,
}

#[derive(Debug, Default, Clone)]
pub struct Warehouse {
    stacks: Vec<Stack>,
    instructions: Vec<Instruction>,
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tallest_stack_height = 0;
        for cur_stack in &self.stacks {
            let mut cur_stack_height = 0;
            for cur_crate in &cur_stack.crates {
                cur_stack_height += 1;
            }
            if cur_stack_height > tallest_stack_height {
                tallest_stack_height = cur_stack_height;
            }
        }

        let mut msg = String::default();
        for col in 0..tallest_stack_height {
            let mut cur_line = String::default();
            for row in 0..self.stacks.len() {
                let mut contents = ' ';
                if self.stacks.len() > row && self.stacks[row].crates.len() > col {
                    contents = self.stacks[row].crates[col].contents;
                }
                cur_line = format!("{} [{}]", cur_line, contents);
            }
            msg = format!("{}\n{}", cur_line, msg);
        }
        write!(f, "{}", msg)
    }
}

impl From<&String> for Warehouse {
    fn from(filedata: &String) -> Self {
        // go through each layer of the crate stacks and convert the strings into types
        let mut warehouse = Warehouse::default();

        // extract the crate stacks drawing into a nested vec
        let mut raw_warehouse: Vec<Vec<String>> = vec![];
        let mut crate_parsing_finished = false;
        let mut state = WarehouseParsingState::Drawing;
        for line in filedata.lines() {
            match state {
                WarehouseParsingState::Drawing => {
                    let crate_mod = 4;
                    let char_idx = 0;
                    let mut line_chars = line.chars();

                    let mut exit_loop = false;
                    let mut cur_stack_level: Vec<String> = vec![];
                    loop {
                        let mut cur_crate = String::default();
                        for i in 0..crate_mod {
                            let c = line_chars.next();
                            if c == None {
                                exit_loop = true;
                                break;
                            }
                            cur_crate = format!("{}{}", cur_crate, c.unwrap()).trim().to_string();
                        }
                        if cur_crate == "1" {
                            state = WarehouseParsingState::EmptyLine;
                            exit_loop = true;
                        } else {
                            cur_stack_level.push(cur_crate);
                        }
                        if exit_loop {
                            break;
                        }
                    }
                    if cur_stack_level.len() > 0 {
                        raw_warehouse.push(cur_stack_level);
                    }
                }
                WarehouseParsingState::EmptyLine => {
                    // reverse the vec and iterate over so that our 'bottom' is index 0
                    for raw_stack_level in raw_warehouse.iter().rev() {
                        let mut idx = 0;
                        for raw_crate in raw_stack_level {
                            if raw_crate != &String::default() {
                                // add another stack
                                if warehouse.stacks.len() == idx {
                                    warehouse.stacks.push(Stack {
                                        id: idx + 1,
                                        crates: vec![],
                                    });
                                }
                                warehouse.stacks[idx].crates.push(Crate {
                                    contents: raw_crate.chars().nth(1).unwrap(),
                                });
                            }
                            idx += 1;
                        }
                    }

                    state = WarehouseParsingState::Instructions;
                }
                WarehouseParsingState::Instructions => {
                    warehouse.instructions.push(Instruction::from(line));
                }
            }
        }

        warehouse
    }
}
