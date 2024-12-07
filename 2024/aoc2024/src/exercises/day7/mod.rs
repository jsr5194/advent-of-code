use itertools::Itertools;
use log::info;

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
    Concat,
}

impl Operation {
    fn get_value(&self) -> String {
        match self {
            Operation::Add => String::from("+"),
            Operation::Multiply => String::from("*"),
            Operation::Concat => String::from("||"),
        }
    }
}

#[derive(Debug, Default)]
struct Equation {
    test_value: usize,
    values: Vec<usize>,
    solvable: bool,
}

impl From<&str> for Equation {
    fn from(raw_equation: &str) -> Self {
        let mut split_line = raw_equation.split(": ");
        Equation {
            test_value: split_line.next().unwrap().parse::<usize>().unwrap(),
            values: split_line
                .next()
                .unwrap()
                .split(" ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
            solvable: false,
        }
    }
}

impl Equation {
    fn test_equation(&mut self, include_concat: bool) {
        if include_concat {
            let mut oplists_iter = self.get_operations_iter_all();

            for oplist in oplists_iter {
                let mut oplist_iter = oplist.into_iter();
                let mut msg = String::default();
                let mut cur_result = 0;
                let mut first_run = true;
                for value in &self.values {
                    if first_run {
                        cur_result = *value;
                        first_run = false;
                    } else {
                        let mut modified_value = value;
                        let op = oplist_iter.next();
                        if op.is_some() {
                            match op.unwrap() {
                                Operation::Add => {
                                    cur_result += *value;
                                }
                                Operation::Multiply => {
                                    cur_result *= *value;
                                }
                                Operation::Concat => {
                                    cur_result = format!("{}{}", cur_result, value)
                                        .parse::<usize>()
                                        .unwrap();
                                }
                            }
                        }
                    }
                }
                if cur_result == self.test_value {
                    self.solvable = true;
                    return;
                }
            }
        } else {
            let mut oplists_iter = self.get_operations_iter_add_multiply();

            for oplist in oplists_iter {
                let mut oplist_iter = oplist.into_iter();
                let mut msg = String::default();
                let mut cur_result = 0;
                let mut first_run = true;
                for value in &self.values {
                    if first_run {
                        cur_result = *value;
                        first_run = false;
                    } else {
                        let mut modified_value = value;
                        let op = oplist_iter.next();
                        if op.is_some() {
                            match op.unwrap() {
                                Operation::Add => {
                                    cur_result += *value;
                                }
                                Operation::Multiply => {
                                    cur_result *= *value;
                                }
                                _ => panic!("no other operations should exist"),
                            }
                        }
                    }
                }
                if cur_result == self.test_value {
                    self.solvable = true;
                    return;
                }
            }
        }
    }

    fn get_operations_iter_add_multiply(
        &self,
    ) -> itertools::MultiProduct<std::array::IntoIter<Operation, 2>> {
        // https://stackoverflow.com/questions/71420176/permutations-with-replacement-in-rust
        let sequence_length = self.values.len() - 1;
        let operations = (0..sequence_length)
            .map(move |_| [Operation::Add, Operation::Multiply])
            .multi_cartesian_product();

        operations
    }

    fn get_operations_iter_all(
        &self,
    ) -> itertools::MultiProduct<std::array::IntoIter<Operation, 3>> {
        // https://stackoverflow.com/questions/71420176/permutations-with-replacement-in-rust
        let sequence_length = self.values.len() - 1;
        let operations = (0..sequence_length)
            .map(move |_| [Operation::Add, Operation::Multiply, Operation::Concat])
            .multi_cartesian_product();

        operations
    }
}

#[derive(Debug, Default)]
struct CalibrationEquations {
    equations: Vec<Equation>,
}

impl From<&String> for CalibrationEquations {
    fn from(raw_equations: &String) -> Self {
        let mut ce = CalibrationEquations::default();
        for line in raw_equations.lines() {
            ce.equations.push(Equation::from(line));
        }
        ce
    }
}

impl CalibrationEquations {
    fn get_valid_calibration_results(&mut self, include_concat: bool) -> usize {
        let mut result = 0;
        for e in &mut self.equations {
            e.test_equation(include_concat);
            if e.solvable {
                result += e.test_value;
            }
        }

        result
    }
}

pub fn process_input(filedata: &String) -> CalibrationEquations {
    CalibrationEquations::from(filedata)
}

pub fn run_part1(filedata: &String) -> usize {
    let mut ce = process_input(filedata);
    let include_concat = false;
    let result = ce.get_valid_calibration_results(include_concat);
    println!("Day 7 Part 1 Result: {}", result);
    result
}

pub fn run_part2(filedata: &String) -> usize {
    let mut ce = process_input(filedata);
    let include_concat = true;
    let result = ce.get_valid_calibration_results(include_concat);
    println!("Day 7 Part 2 Result: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use crate::common::read_file;
    use crate::exercises::day7::run_part1;
    use crate::exercises::day7::run_part2;
    #[test]
    fn test() {
        assert_eq!(
            run_part1(&read_file("./src/exercises/day7/input_test.txt")),
            3749
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day7/input.txt")),
            303766880536
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day7/input_test.txt")),
            11387
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day7/input.txt")),
            337041851384440
        )
    }
}
