use crate::lib::navcpu::Instruction;
use crate::lib::navcpu::Direction;
use crate::lib::navcpu::Latitude;
use crate::lib::navcpu::Longitude;
use crate::lib::navcpu::Position;
use crate::lib::navcpu::NavCpu;
use std::fs;

fn get_input() -> Vec<Instruction> {
	let filename = "./src/day12/input.txt";
	let contents = fs::read_to_string(filename).expect("could not read file");
	
	let mut program: Vec<Instruction> = vec![];

	for line in contents.lines() {
		let value: usize = line.chars()
							.next()
							.map(|_x| &line[1..])
							.unwrap()
							.parse::<usize>()
							.unwrap();

		let instr: Instruction;
		match line.chars().nth(0).unwrap() {
			'N' => instr = Instruction{action: Direction::North, value: value},
			'S' => instr = Instruction{action: Direction::South, value: value},
			'E' => instr = Instruction{action: Direction::East, value: value},
			'W' => instr = Instruction{action: Direction::West, value: value},
			'L' => instr = Instruction{action: Direction::Left, value: value},
			'R' => instr = Instruction{action: Direction::Right, value: value},
			'F' => instr = Instruction{action: Direction::Forward, value: value},
			_ => panic!("Invalid direction detected"),
		}

		program.push(instr);
	}

	program
}

pub fn run_part1() {
	let program = get_input();
	let mut nav_cpu = NavCpu {
		ship: Position {
			latitude: Latitude::default(),
			longitude: Longitude {
				direction: Direction::East,
				value: 1,
			},
		},
		waypoint: Position {
			latitude: Latitude::default(),
			longitude: Longitude {
				direction: Direction::East,
				value: 1,
			},
		},
	};

	nav_cpu.run(&program);

	println!("Day 12 Part 1 Result: {}", nav_cpu.ship.latitude.value + nav_cpu.ship.longitude.value);
	println!("Day 12 Part 1 Result: TODO: fix");
}

pub fn run_part2() {

	let program = get_input();

	let mut nav_cpu = NavCpu {
		ship: Position {
			latitude: Latitude {
				direction: Direction::North,
				value: usize::default(),
			},
			longitude: Longitude {
				direction: Direction::East,
				value: usize::default(),
			},
		},
		waypoint: Position {
			latitude: Latitude {
				direction: Direction::North,
				value: 1,
			},
			longitude: Longitude {
				direction: Direction::East,
				value: 10,
			},
		},
	};

	nav_cpu.run(&program);

	println!("Day 12 Part 2 Result: {}", nav_cpu.ship.latitude.value + nav_cpu.ship.longitude.value);
}


