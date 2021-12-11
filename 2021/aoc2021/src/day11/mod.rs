use std::fmt;
use std::fs;

fn get_input() -> Consortium {
	let filename = "./src/day11/input_test_1.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Consortium = Consortium::from(contents_str);
	contents
}

/// run Day 11 Part 1
pub fn run_part_1() {
	let consortium = get_input();
	println!("{}", consortium);
	println!("[*] Day 11 Part 1 Result: TODO");
}

/// run Day 11 Part 2
pub fn run_part_2() {
	println!("[*] Day 11 Part 2 Result: TODO");
}


#[derive(Debug, Default, Clone, PartialEq)]
struct Octopus {
	energy: u32,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Consortium {
	members: Vec<Vec<Octopus>>,
}

impl fmt::Display for Consortium {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut print_str = String::default();
		for row in &self.members {
			for octopus in row {
				print_str += octopus.energy.to_string().as_str();
			}
			print_str += "\n";
		}
		write!(f, "{}", print_str)
	}
}

impl From<String> for Consortium {
	fn from(consortium_str: String) -> Self {
		let mut consortium = Consortium::default();
		for line in consortium_str.lines() {
			let mut cur_octopi: Vec<Octopus> = vec![];
			for c in line.chars() {
				cur_octopi.push(Octopus{energy: c.to_string().parse::<u32>().unwrap()});
			}
			consortium.members.push(cur_octopi);
		}
		consortium
	}
}