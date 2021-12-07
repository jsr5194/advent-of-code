use std::fs;

fn get_input() -> Navy {
	let filename = "./src/day7/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Navy = Navy::from(contents_str);
	contents
}

/// run Day 7 Part 1
pub fn run_part_1() {
	let mut navy = get_input();

	// calculate the fuel consumption for each possible location
	let mut lowest_fuel_consumption: u32 = u32::MAX;
	for pos in navy.lo_pos..navy.hi_pos {
		let mut cur_fuel_consumption: u32 = 0;
		for crab in &navy.subs {
			if crab.pos < pos {
				cur_fuel_consumption += pos - crab.pos;
			} else if crab.pos > pos {
				cur_fuel_consumption += crab.pos - pos;
			}
			// don't care about equals since it would be zero anyway
		}
		if cur_fuel_consumption < lowest_fuel_consumption {
			lowest_fuel_consumption = cur_fuel_consumption;
		}
	}

	println!("[*] Day 7 Part 1 Result: {}", lowest_fuel_consumption);
}

/// run Day 7 Part 2
pub fn run_part_2() {
	let mut navy = get_input();

	// calculate the fuel consumption for each possible location
	let mut lowest_fuel_consumption: u32 = u32::MAX;
	for pos in navy.lo_pos..navy.hi_pos {
		let mut cur_fuel_consumption: u32 = 0;
		for crab in &navy.subs {
			let mut delta = 0;
			if crab.pos < pos {
				delta = pos - crab.pos;
			} else if crab.pos > pos {
				delta = crab.pos - pos;
			}
			// don't care about equals since it would be zero anyway

			for i in 1..=delta {
				cur_fuel_consumption += i;
			}
		}
		if cur_fuel_consumption < lowest_fuel_consumption {
			lowest_fuel_consumption = cur_fuel_consumption;
		}
	}
	println!("[*] Day 7 Part 2 Result: {}", lowest_fuel_consumption);
}


#[derive(Debug, Default, Clone, PartialEq)]
struct Crab {
	pos: u32,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Navy {
	subs: Vec<Crab>,
	hi_pos: u32,
	lo_pos: u32,
}

impl From<String> for Navy {
	fn from(subs_str: String) -> Self {
		let mut navy = Navy::default();
		navy.lo_pos = u32::MAX;
		for sub in subs_str.split(",") {
			let cur_pos = sub.parse::<u32>().unwrap();
			navy.subs.push(Crab{pos: cur_pos});
			if cur_pos > navy.hi_pos {
				navy.hi_pos = cur_pos;
			} else if cur_pos < navy.lo_pos {
				navy.lo_pos = cur_pos;
			}
		}
		navy
	}
}
