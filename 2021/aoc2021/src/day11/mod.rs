use std::fmt;
use std::fs;

fn get_input() -> Consortium {
	let filename = "./src/day11/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Consortium = Consortium::from(contents_str);
	contents
}

/// run Day 11 Part 1
pub fn run_part_1() {
	let mut consortium = get_input();
	for step in 0..100 {
		consortium.step();	
	}

	println!("{}", consortium);
	
	println!("[*] Day 11 Part 1 Result: {}", consortium.flashes);
}

/// run Day 11 Part 2
pub fn run_part_2() {
	let mut consortium = get_input();
	let mut step = 1;
	let mut simultaneous_flash = true;
	loop {
		simultaneous_flash = true;
		consortium.step();
		for y in 0..consortium.members.len() {
			for x in 0..consortium.members[0].len() {
				if consortium.members[y][x].energy != 0 {
					simultaneous_flash = false;
					break;
				}
			}
			if simultaneous_flash {
				break;
			}
		}
		if simultaneous_flash {
			break;
		}
		step += 1;
	}
	
	println!("[*] Day 11 Part 2 Result: {}", step);
}


#[derive(Debug, Default, Clone, PartialEq)]
struct Octopus {
	energy: usize,
	flashed_this_step: bool,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Consortium {
	members: Vec<Vec<Octopus>>,
	flashes: usize,
}

impl Consortium {
	fn step(&mut self) {
		// reset the flash state
		for y in 0..self.members.len() {
			for x in 0..self.members[0].len() {
				self.members[y][x].flashed_this_step = false;
			}
		}

		for y in 0..self.members.len() {
			for x in 0..self.members[0].len() {
				if self.members[y][x].energy == 9 {
					self.flash(x, y);
				} else {
					if !self.members[y][x].flashed_this_step {
						self.members[y][x].energy += 1;
					}
				}
			}
		}
	}

	fn flash(&mut self, x: usize, y:usize) {
		self.members[y][x].energy = 0;
		self.members[y][x].flashed_this_step = true;
		self.flashes += 1;
		
		// top left
		if x == 0 && y == 0 {
			if !self.members[y][x+1].flashed_this_step {
				self.members[y][x+1].energy += 1;
			}
			if !self.members[y+1][x].flashed_this_step {
				self.members[y+1][x].energy += 1;
			}
			if !self.members[y+1][x+1].flashed_this_step {
				self.members[y+1][x+1].energy += 1;
			}
		}
		// top right
		else if x == self.members[0].len()-1 && y == 0 {
			if !self.members[y][x-1].flashed_this_step {
				self.members[y][x-1].energy += 1;
			}
			if !self.members[y+1][x-1].flashed_this_step {
				self.members[y+1][x-1].energy += 1;
			}
			if !self.members[y+1][x].flashed_this_step {
				self.members[y+1][x].energy += 1;
			}
		}
		// bottom left
		else if x == 0 && y == self.members.len()-1 {
			if !self.members[y-1][x].flashed_this_step {
				self.members[y-1][x].energy += 1;
			}
			if !self.members[y-1][x+1].flashed_this_step {
				self.members[y-1][x+1].energy += 1;
			}
			if !self.members[y][x+1].flashed_this_step {
				self.members[y][x+1].energy += 1;
			}
		}
		// bottom right
		else if x == self.members[0].len()-1 && y == self.members.len()-1 {
			if !self.members[y-1][x].flashed_this_step {
				self.members[y-1][x].energy += 1;
			}
			if !self.members[y-1][x-1].flashed_this_step {
				self.members[y-1][x-1].energy += 1;
			}
			if !self.members[y][x-1].flashed_this_step {
				self.members[y][x-1].energy += 1;
			}
		}
		// top row
		else if y == 0 {
			if !self.members[y][x-1].flashed_this_step {
				self.members[y][x-1].energy += 1;
			}
			if !self.members[y][x+1].flashed_this_step {
				self.members[y][x+1].energy += 1;
			}
			if !self.members[y+1][x-1].flashed_this_step {
				self.members[y+1][x-1].energy += 1;
			}
			if !self.members[y+1][x].flashed_this_step {
				self.members[y+1][x].energy += 1;
			}
			if !self.members[y+1][x+1].flashed_this_step {
				self.members[y+1][x+1].energy += 1;
			}
		}
		// left row
		else if x == 0 {
			if !self.members[y-1][x].flashed_this_step {
				self.members[y-1][x].energy += 1;
			}
			if !self.members[y-1][x+1].flashed_this_step {
				self.members[y-1][x+1].energy += 1;
			}
			if !self.members[y][x+1].flashed_this_step {
				self.members[y][x+1].energy += 1;
			}
			if !self.members[y+1][x].flashed_this_step {
				self.members[y+1][x].energy += 1;
			}
			if !self.members[y+1][x+1].flashed_this_step {
				self.members[y+1][x+1].energy += 1;
			}
		}
		// right row
		else if x == self.members[0].len()-1 {
			if !self.members[y-1][x-1].flashed_this_step {
				self.members[y-1][x-1].energy += 1;
			}
			if !self.members[y-1][x].flashed_this_step {
				self.members[y-1][x].energy += 1;
			}
			if !self.members[y][x-1].flashed_this_step {
				self.members[y][x-1].energy += 1;
			}
			if !self.members[y+1][x].flashed_this_step {
				self.members[y+1][x].energy += 1;
			}
			if !self.members[y+1][x-1].flashed_this_step {
				self.members[y+1][x-1].energy += 1;
			}
		}
		// bottom row
		else if y == self.members.len()-1 {
			if !self.members[y-1][x-1].flashed_this_step {
				self.members[y-1][x-1].energy += 1;
			}
			if !self.members[y-1][x].flashed_this_step {
				self.members[y-1][x].energy += 1;
			}
			if !self.members[y-1][x+1].flashed_this_step {
				self.members[y-1][x+1].energy += 1;
			}
			if !self.members[y][x-1].flashed_this_step {
				self.members[y][x-1].energy += 1;
			}
			if !self.members[y][x+1].flashed_this_step {
				self.members[y][x+1].energy += 1;
			}
		}
		// middles
		else {
			if !self.members[y-1][x-1].flashed_this_step {
				self.members[y-1][x-1].energy += 1;
			}
			if !self.members[y-1][x].flashed_this_step {
				self.members[y-1][x].energy += 1;
			}
			if !self.members[y-1][x+1].flashed_this_step {
				self.members[y-1][x+1].energy += 1;
			}
			if !self.members[y][x-1].flashed_this_step {
				self.members[y][x-1].energy += 1;
			}
			if !self.members[y][x+1].flashed_this_step {
				self.members[y][x+1].energy += 1;
			}
			if !self.members[y+1][x-1].flashed_this_step {
				self.members[y+1][x-1].energy += 1;
			}
			if !self.members[y+1][x].flashed_this_step {
				self.members[y+1][x].energy += 1;
			}
			if !self.members[y+1][x+1].flashed_this_step {
				self.members[y+1][x+1].energy += 1;
			}
		}

		// loop over everything and make sure nothing is nine
		for y in 0..self.members.len() {
			for x in 0..self.members[0].len() {
				if self.members[y][x].energy > 9 {
					self.flash(x, y);
				}
			}
		}

	}
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
				cur_octopi.push(Octopus{energy: c.to_string().parse::<usize>().unwrap(), flashed_this_step: false});
			}
			consortium.members.push(cur_octopi);
		}
		consortium
	}
}