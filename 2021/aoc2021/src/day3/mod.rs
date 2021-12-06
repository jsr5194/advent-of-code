use std::fs;

fn get_input() -> Vec<u32> {
	let filename = "./src/day3/input_test.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Vec<u32> = contents_str.lines().map(|x| u32::from_str_radix(x, 2).unwrap()).collect();
	contents
}

/// run Day 3 Part 1
pub fn run_part_1() {
	// define results
	let mut gamma_rate: u32 = 0;
	let mut epsilon_rate: u32 = 0;

	// set max check depth
	let max_depth: u32 = 5;

	// input
	let report = get_input();
	println!("contents: {:?}", report);

	let mut mask: u32 = 1;
	for i in max_depth..0 {
		let mut zeroes: u32 = 0;
		let mut ones: u32 = 0;
		for entry in &report {
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

	println!("gamma_rate: {:?}", gamma_rate);
	println!("epsilon_rate: {:?}", epsilon_rate);

	// input.txt 845186
	println!("[*] Day 3 Part 1 Result: {}", gamma_rate * epsilon_rate);
}

/// run Day 3 Part 2
pub fn run_part_2() {
	// define results
	let mut o2: u32 = 0;
	let co2: u32 = 0;

	// set max check depth
	let max_depth: u32 = 5;

	// input
	let mut report = get_input();

	// oxygen
	let mut mask: u32 = 1;
	for i in max_depth..0 {
		let mut zeroes: u32 = 0;
		let mut ones: u32 = 0;
		for entry in &report {
			if ((entry & mask) >> i) == 1 {
				ones += 1;
			} else {
				zeroes += 1;
			}
		}

		let mut new_report: Vec<u32> = vec![];
		if ones >= zeroes {
			for entry in &report {
				if ((entry & mask) >> i) == 1 {
					new_report.push(entry.clone());
				}
			}
		}
		report = new_report.clone();
		println!("report len {:?}", report.len());
		if report.len() == 1 {
			o2 = report[0];
		}


		mask = mask << 1;
	}

	println!("o2: {:?}", o2);
	println!("co2: {:?}", co2);

	println!("[*] Day 3 Part 2 Result: TODO");
}
