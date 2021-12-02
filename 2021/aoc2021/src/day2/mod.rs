use std::fs;

fn get_input() -> Vec<Command> {
	let filename = "./src/day2/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let mut contents: Vec<Command> = vec![];
	for line in contents_str.lines() {
		contents.push(Command::from(line.to_string()));
	}
	contents
}

/// run Day s Part 1
pub fn run_part_1() {
	let mut submarine = Submarine::default();
	let commands = get_input();
	for cmd in commands {
		match cmd {
			Command::FORWARD{value} => {
				submarine.horiz += value
			},
			Command::UP{value} => {
				submarine.depth -= value
			},
			Command::DOWN{value} => {
				submarine.depth += value
			}
		}
	}

	println!("[*] Day 2 Part 1 Result: {}", submarine.depth*submarine.horiz)
}

/// run Day s Part 2
pub fn run_part_2() {
	let mut submarine = Submarine::default();
	let commands = get_input();
	for cmd in commands {
		match cmd {
			Command::FORWARD{value} => {
				submarine.horiz += value;
				submarine.depth += value * submarine.aim;
			},
			Command::UP{value} => {
				submarine.aim -= value;
			},
			Command::DOWN{value} => {
				submarine.aim += value;
			},
		}
	}

	println!("[*] Day 2 Part 2 Result: {}", submarine.horiz*submarine.depth)
}

#[derive(Default, Debug, Clone)]
struct Submarine {
	depth: u32,
	horiz: u32,
	aim: u32,
}

#[derive(Debug, Clone)]
enum Command {
	FORWARD {value:u32},
	UP {value:u32},
	DOWN {value:u32},
}

impl From<String> for Command {
	fn from(command: String) -> Self {
		let split_command: Vec<&str> = command.split(" ").map(|x| x).collect();
		let value = split_command[1].parse::<u32>().expect("[!] Error converting value");
		match split_command[0] {
			"forward" => Command::FORWARD{value: value},
			"up" => Command::UP{value: value},
			"down" => Command::DOWN{value: value},
			_ => panic!("[!] Error converting command direction")
		}
	}
}

