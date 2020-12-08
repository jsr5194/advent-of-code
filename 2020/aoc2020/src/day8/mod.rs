use crate::lib::handheld::Handheld;
use std::fs;

fn get_input() -> Vec<String> {
	let filename = "./src/day8/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] Could not read file");
	let program: Vec<String> = contents_str.split("\n").map(|x| String::from(x)).collect();
	program
}

pub fn run_part1() {
	// parse the raw input
	let program = get_input();

	// build our handheld emulator
	let mut handheld = Handheld::default();

	// load the program into something easier to parse
	handheld.assemble(program);

	// run the program
	handheld.run();
	
	// print our results
	println!("Day 8 Part 1 Result: {}", handheld.r0);
}

pub fn run_part2() {

}