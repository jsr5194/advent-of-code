use std::fs;

use hex::decode;

use crate::intshark::IntShark;

fn get_input() -> String {
	let filename = "./src/day16/input_test_1.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	contents_str
}

/// run Day 16 Part 1
pub fn run_part_1() {
	let mut intshark = IntShark::default();
	intshark.load(get_input());
	intshark.dissect();
	println!("{:?}", intshark);
	println!("[*] Day 16 Part 1 Result: TODO");
}

/// run Day 16 Part 2
pub fn run_part_2() {
	println!("[*] Day 16 Part 2 Result: TODO");
}

