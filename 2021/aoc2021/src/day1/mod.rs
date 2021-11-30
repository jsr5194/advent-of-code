use std::fs;

fn get_input() -> Vec<u32> {
	let filename = "./src/day1/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Vec<u32> = contents_str.lines().map(|x| x.parse::<u32>().expect("[!] ERROR: could not convert to u32")).collect();
	return contents;
}

/// run Day 1 Part 1
pub fn run_part_1() {
	println!("[*] Day 1 Part 1 Result: TODO");
}

/// run Day 1 Part 2
pub fn run_part_2() {
	println!("[*] Day 1 Part 2 Result: TODO");
}