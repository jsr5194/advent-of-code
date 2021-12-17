use std::collections::HashMap;
use std::fmt;
use std::fs;

fn get_input() -> Vec<u32> {
	let filename = "./src/day12/input_test_1.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Vec<u32> = contents_str.lines().map(|x| x.parse::<u32>().expect("[!] ERROR: could not convert to u32")).collect();
	contents
}

/// run Day 12 Part 1
pub fn run_part_1() {
	println!("[*] Day 12 Part 1 Result: TODO");
}

/// run Day 12 Part 2
pub fn run_part_2() {
	println!("[*] Day 12 Part 2 Result: TODO");
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Cave {
	name: String,
	adjacent_caves: Vec<String>,
	is_large: bool,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct RoughMap {

}