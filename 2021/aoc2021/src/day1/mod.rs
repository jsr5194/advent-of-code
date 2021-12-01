use std::fs;

fn get_input() -> Vec<u32> {
	let filename = "./src/day1/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Vec<u32> = contents_str.lines().map(|x| x.parse::<u32>().expect("[!] ERROR: could not convert to u32")).collect();
	return contents;
}

/// run Day 1 Part 1
pub fn run_part_1() {
	let report = get_input();

	let mut increases: u32 = 0;
	let mut last_value: u32 = 0;
	let mut first_run: bool = true;
	for entry in report {
		if !first_run {
			if entry > last_value {
				increases += 1;
			}
		} else {
			first_run = false;
		}
		last_value = entry;
	}

	println!("[*] Day 1 Part 1 Result: {}", increases)
}

/// run Day 1 Part 2
pub fn run_part_2() {
	let report = get_input();

	let mut increases: u32 = 0;
	let mut last_value: u32 = 0;
	let mut v0: u32 = 0;
	let mut v1: u32 = 0;
	let mut v2: u32 = 0;
	let mut run: u32 = 0;
	for entry in report {
		if run > 2 {
			let cur_value: u32 = v0 + v1 + v2;
			if cur_value > last_value {
				increases += 1;
			}
			last_value = cur_value;
			v0 = v1;
			v1 = v2;
			v2 = entry;
		} else if run == 0 {
			v0 = entry;
		} else if run == 1 {
			v1 = entry;
		} else if run == 2 {
			v2 = entry;
		}
		run += 1;
	}
	println!("[*] Day 1 Part 2 Result: {}", increases)
}