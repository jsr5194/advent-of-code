use std::fs;

fn get_input() -> DiagnosticReport {
	let filename = "./src/day3/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: DiagnosticReport = DiagnosticReport::from(contents_str);
	contents
}

/// run Day 3 Part 1
pub fn run_part_1() {
	// define results
	let mut gamma_rate: u32 = 0;
	let mut epsilon_rate: u32 = 0;

	// input
	let report = get_input();

	let mut mask: u32 = 1;
	for i in 0..report.depth {
		let mut zeroes: u32 = 0;
		let mut ones: u32 = 0;
		for entry in &report.entries {
			if ((entry & mask) >> i) == 1 {
				ones += 1;
			} else {
				zeroes += 1;
			}
		}

		if ones > zeroes {
			gamma_rate = gamma_rate | (1 << i);
		} else {
			epsilon_rate = epsilon_rate | (1 << i);
		}
		mask = mask << 1;
	}

	println!("[*] Day 3 Part 1 Result: {}", gamma_rate * epsilon_rate);
}

/// run Day 3 Part 2
pub fn run_part_2() {
	let full_report = get_input();

	let mut mask = 0;

	// oxygen generator rating
	mask = 1 << full_report.depth-1;

	let mut cur_report = full_report.clone();

	for bit_pos in (0..full_report.depth).rev() {
		mask = 1 << bit_pos;
		let mut zeroes = 0;
		let mut ones = 0;

		for entry in &cur_report.entries {
			if ((entry & mask) >> bit_pos) == 1 {
				ones += 1;
			} else {
				zeroes += 1;
			}
		}

		if ones >= zeroes {
			cur_report.entries.retain(|&entry| (entry & mask) >> bit_pos == 1);
		} else {
			cur_report.entries.retain(|&entry| (entry & mask) >> bit_pos == 0);
		}

		if cur_report.entries.len() == 1 {
			break;
		}
	}

	if cur_report.entries.len() != 1 {
		panic!("unexpected number of entries left: {}", cur_report.entries.len());
	}
	let o2_rating = cur_report.entries[0];


	// carbon dioxide generator rating
	mask = 1 << full_report.depth-1;

	let mut cur_report = full_report.clone();

	for bit_pos in (0..full_report.depth).rev() {
		mask = 1 << bit_pos;
		let mut zeroes = 0;
		let mut ones = 0;

		for entry in &cur_report.entries {
			if ((entry & mask) >> bit_pos) == 1 {
				ones += 1;
			} else {
				zeroes += 1;
			}
		}

		if zeroes <= ones {
			cur_report.entries.retain(|&entry| (entry & mask) >> bit_pos == 0);
		} else {
			cur_report.entries.retain(|&entry| (entry & mask) >> bit_pos == 1);
		}

		if cur_report.entries.len() == 1 {
			break;
		}
	}

	if cur_report.entries.len() != 1 {
		panic!("unexpected number of entries left: {}", cur_report.entries.len());
	}
	let co2_rating = cur_report.entries[0];

	println!("[*] Day 3 Part 2 Result: {}", o2_rating * co2_rating);
}


#[derive(Debug, Default, Clone, PartialEq)]
struct DiagnosticReport {
	entries: Vec<u32>,
	depth: usize,
}

impl From<String> for DiagnosticReport {
	fn from(report_str: String) -> Self {
		let mut report = DiagnosticReport::default();
		report.depth = report_str.lines().next().unwrap().chars().count();
		for line in report_str.lines() {
			report.entries.push(u32::from_str_radix(line, 2).unwrap())
		}

		report
	}
}



