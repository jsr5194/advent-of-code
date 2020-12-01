use std::fs;

fn get_input() -> Vec<u32> {
	let filename = "./src/day1/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Vec<u32> = contents_str.lines().map(|x| x.parse::<u32>().expect("[!] ERROR: could not convert to u32")).collect();
	return contents;
}

/// run Day 1 Part 1
pub fn run_part1() {
	// get the file contents
	let contents:Vec<u32> = get_input();

	// 
	let mut result = 0;
	for i in 0..contents.len() {
		for j in 1..contents.len() {
			if contents[i] + contents[j] == 2020 {
				result = contents[i] * contents[j];
				break;
			}
		}
	}

	println!("[*] Day 1 Part 1 Result: {}", result);
}

/// run Day 1 Part 2
pub fn run_part2() {
	// get the file contents
	let contents: Vec<u32> = get_input();

	// need three entries that sum to 2020
	let mut result = 0;
	for i in 0..contents.len() {
		for j in 1..contents.len() {
			for k in 2..contents.len() {
				if contents[i] + contents[j] + contents[k] == 2020 {
					result = contents[i] * contents[j] * contents[k];
					break;
				}
			}
		}
	}

	println!("[*] Day 1 Part 2 Result: {}", result);
}