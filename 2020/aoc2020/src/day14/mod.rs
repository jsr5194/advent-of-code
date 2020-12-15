use std::fmt;
use std::fs;

fn get_input() -> BitmaskCpu {
	let filename = "./src/day14/input.txt";
	let contents = fs::read_to_string(filename).unwrap();
	let mut cpu = BitmaskCpu::default();
	for line in contents.lines() {
		cpu.program.push(Instruction::from(line));
	}

	cpu
}

pub fn run_part1() {
	let mut cpu = get_input();
	cpu.run();

	let mut result = 0;
	for byte in &cpu.memory {
		result += byte.value;
	}

	println!("Day 14 Part 1: {}", result);
}

pub fn run_part2() {

}

#[derive(Debug, Clone, PartialEq)]
struct BitmaskCpu {
	program: Vec<Instruction>,
	memory: Vec<Byte>,
	or_mask: usize,
	and_mask: usize,
}

impl Default for BitmaskCpu {
	fn default() -> Self {
		BitmaskCpu {
			program: vec![],
			memory: vec![Byte::default(); 0xFFFF],
			or_mask: 0,
			and_mask: 0,
		}
	}
}

impl fmt::Display for BitmaskCpu {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut output = String::default();
		for idx in 0..self.memory.len() {
			if idx % 16 == 0 {
				output += "\n";
			}
			if self.memory[idx].value == 0 {
				output += " ..........";
			} else {
				output += format!(" {:010X}", self.memory[idx].value).as_str();
			}
		}
		write!(f, "{}", output)
	}
}

impl BitmaskCpu {
	fn run(&mut self) {
		for instr in &self.program {
			match instr {
				Instruction::Mask {or_mask, and_mask} => {
					self.or_mask = *or_mask;
					self.and_mask = *and_mask;
				},
				Instruction::Mem {addr, value} => {
					let mut new_value = value | self.or_mask;
					new_value = new_value & !self.and_mask;
//					if new_value & self.and_mask > 0 {
//						new_value = new_value & self.and_mask;
//					}
					println!("OR Mask:  {:?}", self.or_mask);
					println!("XOR Mask: {:?}", self.and_mask);
					println!("Instr:    {:?}", instr);
					println!("Before Mem: {:?}", self.memory.get(*addr).unwrap().value);
					self.memory.get_mut(*addr).unwrap().value = new_value;
					println!("After Mem: {:?}\n", self.memory.get(*addr).unwrap().value);
				},
			};
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
	Mask {or_mask: usize, and_mask: usize},
	Mem {addr: usize, value: usize},
}

impl From<&str> for Instruction {
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
			
			let or_mask = usize::from_str_radix(&or_mask_str, 2).unwrap();
			let and_mask = usize::from_str_radix(&and_mask_str, 2).unwrap();

			Instruction::Mask{
				or_mask: or_mask.clone(),
				and_mask: and_mask.clone(),
			}
		} else {
			Instruction::Mem{
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
struct Byte {
	value: usize,
}
