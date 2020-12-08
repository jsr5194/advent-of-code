use crate::lib::handheld::Handheld;
use crate::lib::handheld::Instruction;
use std::fs;

fn get_input() -> Vec<Instruction> {
	let filename = "./src/day8/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] Could not read file");
	let contents: Vec<String> = contents_str.split("\n").map(|x| String::from(x)).collect();
	let mut program: Vec<Instruction> = vec![];

	for instruction in contents {
		program.push(Instruction::from(instruction));
	}
	program
}



pub fn run_part1() {
	// parse the raw input
	let program = get_input();

	// build our handheld emulator
	let mut handheld = Handheld::default();

	// run the program
	handheld.run(program);
	
	// print our results
	println!("Day 8 Part 1 Result: {}", handheld.r0);
}

pub fn run_part2() {
	// parse the raw input
	let program = get_input();

	// build our handheld emulator
	let mut handheld = Handheld::default();

	for idx in 0..program.len() {
		// reset the state of the cpu
		handheld.reset();

		let swap_made;
		let mut swapped_program = program.clone();

		match swapped_program[idx] {
			Instruction::ACC{executed: _, value: _} => {
				swap_made = false;
			}
			Instruction::JMP{executed: _, value: _} => {
				swapped_program[idx] = Instruction::NOP{executed: false, value: swapped_program[idx].get_value()};
				swap_made = true;
			}
			Instruction::NOP{executed: _, value: _} => {
				swapped_program[idx] = Instruction::JMP{executed: false, value: swapped_program[idx].get_value()};
				swap_made = true;
			}
		}

		if swap_made {
			// run the program
			handheld.run(swapped_program);

			// break when we find the program that works
			if !handheld.error {
				break;
			}
		}
	}

	// print our results
	println!("Day 8 Part 2 Result: {}", handheld.r0);

}