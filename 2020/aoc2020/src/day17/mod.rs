use std::collections::HashMap;
use std::fmt;
use std::fs;

fn get_input() -> Map {
	let filename = "./src/day17/input.txt";
	let contents = fs::read_to_string(filename).unwrap();

	let mut map = Map::default();
	let w = 0;
	let z = 0;
	let mut y = 0;
	for line in contents.lines() {
		let mut x = 0;
		for cell in line.chars() {
			let key = map.build_key(x, y, z, w);
			match cell {
				'#' => {
					map.grid.insert(key, Coordinate{x: x, y: y, z: z, w: w, state: State::Active});
				},
				'.' => {
					map.grid.insert(key, Coordinate{x: x, y: y, z: z, w: w, state: State::Inactive});
				},
				_ =>{
					panic!("Invalid cell state");
				},
			}
			x += 1;
		}
		y += 1;
	}

	map
}

pub fn run_part1() {
	let mut map = get_input();

	let cycles = 6;
	for cycle in 0..cycles {
		map.run_3d_cycle();
	}

	println!("Day 17 Part 1: {}", map.get_active_cell_count());
}

pub fn run_part2() {
	let mut map = get_input();
	
	let cycles = 6;
	for cycle in 0..cycles {
		map.run_4d_cycle();
	}

	println!("Day 17 Part 2: {}", map.get_active_cell_count());
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Map {
	grid: HashMap<String, Coordinate>,
	unknown_state: State,
}

impl Map {
	fn run_3d_cycle(&mut self) {
		let old_map = self.clone();
		let w = 0;
		for z in self.get_viewport_depth_start()-1..self.get_viewport_depth_end()+1 {
			for y in self.get_viewport_height_start()-1..self.get_viewport_height_end()+1 {
				for x in self.get_viewport_width_start()-1..self.get_viewport_width_end()+1 {
					self.change_cell(&old_map, x, y, z, w);
				}
			}
		}
	}

	fn run_4d_cycle(&mut self) {
		let old_map = self.clone();
		for w in self.get_viewport_time_start()-1 ..self.get_viewport_time_end()+1 {
			for z in self.get_viewport_depth_start()-1..self.get_viewport_depth_end()+1 {
				for y in self.get_viewport_height_start()-1..self.get_viewport_height_end()+1 {
					for x in self.get_viewport_width_start()-1..self.get_viewport_width_end()+1 {
						self.change_cell(&old_map, x, y, z, w);
					}
				}
			}
		}
	}

	fn build_key(&self, x: isize, y: isize, z: isize, w: isize) -> String {
		format!("({},{},{},{})", x, y, z, w)
	}

	fn get_viewport_time_start(&self) -> isize {
		let mut known_time = isize::MAX;
		for key in self.grid.keys() {
			if self.grid[key].w < known_time {
				if self.grid[key].state == State::Active {
					known_time = self.grid[key].w;
				}
			}
		}
		known_time
	}

	fn get_viewport_time_end(&self) -> isize {
		let mut known_time = isize::MIN;
		for key in self.grid.keys() {
			if self.grid[key].w > known_time {
				if self.grid[key].state == State::Active {
					known_time = self.grid[key].w;
				}
			}
		}
		known_time+1
	}

	fn get_viewport_height_start(&self) -> isize {
		let mut known_height = isize::MAX;
		for key in self.grid.keys() {
			if self.grid[key].y < known_height {
				if self.grid[key].state == State::Active {
					known_height = self.grid[key].y;
				}
			}
		}
		known_height
	}

	fn get_viewport_height_end(&self) -> isize {
		let mut known_height = isize::MIN;
		for key in self.grid.keys() {
			if self.grid[key].y > known_height {
				if self.grid[key].state == State::Active {
					known_height = self.grid[key].y;
				}
			}
		}
		known_height+1
	}

	fn get_viewport_width_start(&self) -> isize {
		let mut known_width = isize::MAX;
		for key in self.grid.keys() {
			if self.grid[key].x < known_width {
				if self.grid[key].state == State::Active {
					known_width = self.grid[key].x;
				}
			}
		}
		known_width
	}

	fn get_viewport_width_end(&self) -> isize {
		let mut known_width = isize::MIN;
		for key in self.grid.keys() {
			if self.grid[key].x > known_width {
				if self.grid[key].state == State::Active {
					known_width = self.grid[key].x;
				}
			}
		}
		known_width+1
	}

	fn get_viewport_depth_start(&self) -> isize {
		let mut known_depth = isize::MAX;
		for key in self.grid.keys() {
			if self.grid[key].z < known_depth {
				if self.grid[key].state == State::Active {
					known_depth = self.grid[key].z;
				}
			}
		}
		known_depth
	}

	fn get_viewport_depth_end(&self) -> isize {
		let mut known_depth = isize::MIN;
		for key in self.grid.keys() {
			if self.grid[key].z > known_depth {
				if self.grid[key].state == State::Active {
					known_depth = self.grid[key].z;
				}
			}
		}
		known_depth+1
	}

	fn get_cell(&self, x: isize, y: isize, z: isize, w: isize) -> Coordinate {
		let nearby_cell_key = self.build_key(x, y, z, w); 

		if !self.grid.contains_key(&nearby_cell_key) {
			Coordinate{x: x, y: y, z: z, w: w, state: State::default()}
		} else {
			self.grid[&nearby_cell_key].clone()
		}
	}

	fn modify_cell_contents(&mut self, x: isize, y: isize, z: isize, w: isize, new_state: State) {
		let key = self.build_key(x, y, z, w);
		if self.grid.contains_key(&key) {
			self.grid.get_mut(&key).unwrap().state = new_state;
		} else {
			if new_state != State::Inactive {
				self.grid.insert(key, Coordinate{x: x, y: y, z: z, w: w, state: new_state});
			}
		}
	}

	fn change_cell(&mut self, old_map: &Map, x: isize, y: isize, z: isize, w: isize) {
		let cell = &old_map.get_cell(x, y, z, w);
		let key = self.build_key(x, y, z, w);

		let mut active_count = 0;

		for h in 0..3 {
			let w = cell.w + h - 1;
			for i in 0..3 {
				let z = cell.z + i - 1;
				for j in 0..3 {
					let y = cell.y + j - 1;
					for k in 0..3 {
						let x = cell.x + k - 1;
						if self.build_key(x, y, z, w) != *key {
							if old_map.get_cell(x, y, z, w).state == State::Active {
								active_count += 1;
							}
						}
					}
				}
			}
		}

		//print!("Key: {} Active Count: {}", key, active_count);

		// make the update as needed
		match cell.state {
			State::Active => {
				if active_count == 2 || active_count == 3 {
					//self.grid.get_mut(&key).unwrap().state = State::Active;
					//println!(" Remains Active");
				} else {
					self.modify_cell_contents(cell.x, cell.y, cell.z, cell.w, State::Inactive);
					//self.grid.get_mut(&key).unwrap().state = State::Inactive;
					//println!(" Set Inactive");
				}
			},
			State::Inactive => {
				if active_count == 3 {
					self.modify_cell_contents(cell.x, cell.y, cell.z, cell.w, State::Active);
					//self.grid.get_mut(&key).unwrap().state = State::Active;
					//println!(" Set Active");
				} else {
					//self.grid.get_mut(&key).unwrap().state = State::Inactive;
					//println!(" Remains Inactive");
				}
			},
		}
	}

	fn get_active_cell_count(&self) -> usize {
		let mut active_count = 0;
		for w in self.get_viewport_time_start()-1..self.get_viewport_time_end()+1{
			for z in self.get_viewport_depth_start()-1..self.get_viewport_depth_end()+1 {
				for y in self.get_viewport_height_start()-1..self.get_viewport_height_end()+1 {
					for x in self.get_viewport_width_start()-1..self.get_viewport_width_end()+1 {
						if self.get_cell(x, y, z, w).state == State::Active {
							active_count += 1;
						}
					}
				}
			}
		}

		active_count
	}
}

// implement a pretty print view for the map
impl fmt::Display for Map {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut print_string = String::default();
		for w in self.get_viewport_time_start()..self.get_viewport_time_end() {
			print_string += format!("w == {}\n", w).as_str();
			for z in self.get_viewport_depth_start()..self.get_viewport_depth_end() {
				print_string += format!("z == {}\n", z).as_str();
				for y in self.get_viewport_height_start()..self.get_viewport_height_end() {
					for x in self.get_viewport_width_start()..self.get_viewport_width_end() {
						print_string += format!("{}", self.get_cell(x, y, z, w).clone()).as_str();
					}
					print_string += "\n";
				}
				print_string += "\n";
			}
		}
		write!(f, "{}\n", print_string)
	}
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Coordinate {
	x: isize,
	y: isize,
	z: isize,
	w: isize,
	state: State,
}

impl Coordinate {
	fn get_key(&self) -> String {
		format!("({},{},{},{})", self.x, self.y, self.z, self.w)
	}
}

impl fmt::Display for Coordinate {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.state {
			State::Active => write!(f, "#"),
			State::Inactive => write!(f, "."),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
enum State {
	Active,
	Inactive,
}

impl Default for State {
	fn default() -> Self {
		State::Inactive
	}
}