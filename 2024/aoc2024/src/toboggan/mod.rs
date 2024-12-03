use regex::Regex;

#[derive(Debug, Default, Clone)]
pub struct Toboggan {
    pub mem: Vec<Instruction>,
    pub results: Vec<u32>,
    pub execute: bool,
    pub ignore_execute_flag: bool,
}

impl From<&String> for Toboggan {
    fn from(raw_data: &String) -> Self {
        let re = Regex::new(Instruction::get_regex_needle().as_str()).unwrap();
        Toboggan {
            mem: re
                .find_iter(raw_data)
                .map(|m| Instruction::from(m.as_str()))
                .collect::<Vec<Instruction>>(),
            results: vec![],
            execute: true,
            ignore_execute_flag: false,
        }
    }
}

impl Toboggan {
    pub fn run(&mut self) {
        for ins in &mut self.mem {
            match ins {
                Instruction::MUL { v1, v2 } => {
                    if self.execute || self.ignore_execute_flag {
                        &mut self.results.push(*v1 * *v2);
                    }
                }
                Instruction::DONT => {
                    self.execute = false;
                }
                Instruction::DO => {
                    self.execute = true;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    MUL { v1: u32, v2: u32 }, // r"mul\(\d{1,3},\d{1,3}\)"
    DO,                       // r"do\(\)"
    DONT,                     // r"don\'t\(\)"
}

impl From<&str> for Instruction {
    fn from(raw_ins: &str) -> Self {
        let mut raw_ins_splitter = raw_ins.split("(");
        let mut raw_ins_type = raw_ins_splitter.next().unwrap();
        let mut raw_ins_data = raw_ins_splitter.next().unwrap().split(")").next().unwrap();
        match raw_ins_type {
            "mul" => {
                let mut values_splitter = raw_ins_data.split(",");
                return Instruction::MUL {
                    v1: values_splitter.next().unwrap().parse::<u32>().unwrap(),
                    v2: values_splitter
                        .next()
                        .unwrap()
                        .split(")")
                        .next()
                        .unwrap()
                        .parse::<u32>()
                        .unwrap(),
                };
            }
            "don't" => {
                return Instruction::DONT;
            }
            "do" => {
                return Instruction::DO;
            }
            _ => panic!("could not get a good type"),
        }
    }
}

impl Instruction {
    pub fn get_regex_needle() -> String {
        let mut needle = String::default();
        needle.push_str(r"(");
        needle.push_str(r"mul\(\d{1,3},\d{1,3}\)");
        needle.push_str(r"|");
        needle.push_str(r"don\'t\(\)");
        needle.push_str(r"|");
        needle.push_str(r"do\(\)");
        needle.push_str(r")");
        needle
    }
}
