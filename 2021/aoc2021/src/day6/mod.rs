use std::fmt;
use std::fs;

fn get_input() -> School {
	let filename = "./src/day6/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: School = School::from(contents_str);
	contents
}

/// run Day 6 Part 1
pub fn run_part_1() {
	let mut school: School = get_input();
	let testing_period = 80;

	for _ in 0..testing_period {
		school.age_fish();
	}

	println!("[*] Day 6 Part 1 Result: {}", school.size());
}

/// run Day 6 Part 2
pub fn run_part_2() {
	let mut school: School = get_input();
	let testing_period = 256;

	for _ in 0..testing_period {
		school.age_fish();
	}

	println!("[*] Day 6 Part 2 Result: {}", school.size());
}

#[derive(Debug, Default, PartialEq)]
struct School {
	age0: u64,
	age1: u64,
	age2: u64,
	age3: u64,
	age4: u64,
	age5: u64,
	age6: u64,
	age7: u64,
	age8: u64,
}

impl From<String> for School {
	fn from(school_str: String) -> Self {
		let mut school: School = School::default();
		for fish_str in school_str.split(",") {
			match fish_str.parse::<u8>().unwrap() {
				0 => {
					school.age0 += 1;
				},
				1 => {
					school.age1 += 1;
				},
				2 => {
					school.age2 += 1;
				},
				3 => {
					school.age3 += 1;
				},
				4 => {
					school.age4 += 1;
				},
				5 => {
					school.age5 += 1;
				},
				6 => {
					school.age6 += 1;
				},
				7 => {
					school.age7 += 1;
				},
				8 => {
					school.age8 += 1;
				},
				_ => panic!("invalid initial age")
			}
		}
		school
	}
}

impl School {
	fn age_fish(&mut self) {
		let old_age0 = self.age0;
		self.age0 = self.age1;
		self.age1 = self.age2;
		self.age2 = self.age3;
		self.age3 = self.age4;
		self.age4 = self.age5;
		self.age5 = self.age6;
		self.age6 = self.age7 + old_age0;
		self.age7 = self.age8;
		self.age8 = old_age0;
	}

	fn size(&self) -> u64 {
		self.age0 
		  + self.age1 
		  + self.age2 
		  + self.age3 
		  + self.age4 
		  + self.age5 
		  + self.age6 
		  + self.age7
		  + self.age8
	}
}


