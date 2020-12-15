use std:: collections::HashMap;
use std::fmt;
use std::fs;

fn get_v1_input() -> BitmaskCpuV1 {
	let filename = "./src/day14/input.txt";
	let contents = fs::read_to_string(filename).unwrap();
	let mut cpu = BitmaskCpuV1::default();
	for line in contents.lines() {
		cpu.program.push(V1Instruction::from(line));
	}

	cpu
}

fn get_v2_input() -> BitmaskCpuV2 {
	let filename = "./src/day14/test_input.txt";
	let contents = fs::read_to_string(filename).unwrap();
	println!("contents: {:?}", contents);
	let mut cpu = BitmaskCpuV2::default();
	for line in contents.lines() {
		cpu.program.push(V2Instruction::from(line));
	}

	cpu
}

pub fn run_part1() {
	let mut cpu = get_v1_input();
	cpu.run();

	let mut result = 0;
	for byte in &cpu.memory {
		result += byte.value;
	}

	println!("Day 14 Part 1: {}", result);
}

pub fn run_part2() {
	let mut cpu = get_v2_input();
	cpu.run();

	let mut result = 0;
	for key in cpu.memory.keys() {
		result += cpu.memory[key].value;
	}

	println!("Day 14 Part 2: STILL NEED TO FINISH");
}

#[derive(Debug, Clone, PartialEq)]
struct BitmaskCpuV1 {
	program: Vec<V1Instruction>,
	memory: Vec<Byte>,
	masks: Vec<Mask>,
}

impl Default for BitmaskCpuV1 {
	fn default() -> Self {
		BitmaskCpuV1 {
			program: vec![],
			memory: vec![Byte::default(); 0xFFFF],
			masks: vec![],
		}
	}
}

impl BitmaskCpuV1 {
	fn run(&mut self) {
		for instr in &self.program {
			match instr {
				V1Instruction::Mask {masks} => {
					self.masks = masks.clone();
				},
				V1Instruction::Mem {addr, value} => {
					let mut new_value = value.clone();
					for mask in &self.masks {
						match mask.operation {
							Operation::OR  => new_value = new_value | mask.mask,
							Operation::NAND => new_value = new_value & !mask.mask,
							_ => panic!("Invalid mask operation"),
						}
					}

					self.memory.get_mut(*addr).unwrap().value = new_value;
				},
			};
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
struct BitmaskCpuV2 {
	program: Vec<V2Instruction>,
	memory: HashMap<usize, Byte>,
	masks: Vec<Mask>,
}

impl Default for BitmaskCpuV2 {
	fn default() -> Self {
		BitmaskCpuV2 {
			program: vec![],
			memory: HashMap::new(),
			masks: vec![],
		}
	}
}

impl BitmaskCpuV2 {
	fn run(&mut self) {
		for instr in &self.program {
			match instr {
				V2Instruction::Mask {masks} => {
					self.masks = masks.clone();
				},
				V2Instruction::Mem {addr, value} => {
					let mut new_addr = addr.clone();

					for mask in &self.masks {
						match mask.operation {
							Operation::OR  => {
								new_addr = new_addr | mask.mask;
								//println!("New Addr: {:?}", new_addr);
								if self.memory.contains_key(&new_addr) {
									self.memory.get_mut(&new_addr).unwrap().value = *value;
								} else {
									self.memory.insert(new_addr, Byte{value: *value});
								}
							},
							_ => panic!("Invalid mask operation"),
						}
					}
				},
			};
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
enum V1Instruction {
	Mask {masks: Vec<Mask>},
	Mem {addr: usize, value: usize},
}

impl From<&str> for V1Instruction {
	fn from(instr_line: &str) -> Self {
		let split_instr: Vec<String> = instr_line.split(" = ").map(|x| x.to_string()).collect();
		if instr_line.contains("mask") { 
			let mut or_mask_str = String::default();
			let mut and_mask_str = String::default();
			for c in split_instr[1].chars() {
				if c == 'X' {
					or_mask_str.push('0');
					and_mask_str.push('0');
				} else {
					if c == '0' {
						and_mask_str.push('1');
					} else {
						and_mask_str.push('0');
					}
					or_mask_str.push(c);
				}
			}
			
			let mut masks: Vec<Mask> = vec![];
			masks.push(Mask{operation: Operation::OR, mask: usize::from_str_radix(&or_mask_str, 2).unwrap()});
			masks.push(Mask{operation: Operation::NAND, mask: usize::from_str_radix(&and_mask_str, 2).unwrap()});

			V1Instruction::Mask{
				masks: masks.clone(),
			}
		} else {
			V1Instruction::Mem{
				addr: split_instr[0]
					.split("[")
					.map(|x| x.to_string())
					.collect::<Vec<String>>()[1]
						.split("]")
						.map(|x| x.to_string())
						.collect::<Vec<String>>()[0]
						.parse::<usize>()
						.unwrap(),
				value: split_instr[1].parse::<usize>().unwrap(),
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
enum V2Instruction {
	Mask {masks: Vec<Mask>},
	Mem {addr: usize, value: usize},
}

impl From<&str> for V2Instruction {
	fn from(instr_line: &str) -> Self {
		let split_instr: Vec<String> = instr_line.split(" = ").map(|x| x.to_string()).collect();
		if instr_line.contains("mask") { 
			let mut new_masks: Vec<String> = vec![];
			new_masks.push(String::default());
			for c in split_instr[1].chars() {
				match c {
					'0' => {
						for new_mask in &mut new_masks {
							*new_mask += "0";
						}
					},
					'1' => {
						for new_mask in &mut new_masks {
							*new_mask += "1";
						}
					},
					'X' => {
						let mut dup_new_mask = String::default();
						for new_mask in &mut new_masks {
							*new_mask += "0";

							dup_new_mask = new_mask.clone();
							dup_new_mask += "1";
						}
						new_masks.push(dup_new_mask);
					},
					_ => panic!("invalid mask character"),
				}
			}

			let mut masks: Vec<Mask> = vec![];
			for mask in new_masks {
				masks.push(Mask{operation: Operation::OR, mask: usize::from_str_radix(&mask, 2).unwrap()});
			}

//			for mask in &masks {
//				println!("Mask: {:#036b}", mask.mask);
//			}

			V2Instruction::Mask{
				masks: masks.clone(),
			}
		} else {
			V2Instruction::Mem{
				addr: split_instr[0]
					.split("[")
					.map(|x| x.to_string())
					.collect::<Vec<String>>()[1]
						.split("]")
						.map(|x| x.to_string())
						.collect::<Vec<String>>()[0]
						.parse::<usize>()
						.unwrap(),
				value: split_instr[1].parse::<usize>().unwrap(),
			}
		}

	}
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Mask {
	operation: Operation,
	mask: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
	OR,
	AND,
	NAND,
	XOR,
	NONE,
}

impl Default for Operation {
	fn default() -> Self {
		Operation::NONE
	}
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Byte {
	value: usize,
}
