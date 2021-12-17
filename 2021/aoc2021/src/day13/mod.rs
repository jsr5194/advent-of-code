use std::collections::HashMap;
use std::fmt;
use std::fs;
use regex::Regex;

fn get_input() -> Grid {
	let filename = "./src/day13/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Grid = Grid::from(contents_str);
	contents
}

/// run Day 13 Part 1
pub fn run_part_1() {
	let mut grid = get_input();
	grid.fold();
	println!("[*] Day 13 Part 1 Result: {}", grid.coords.len());
}

/// run Day 13 Part 2
pub fn run_part_2() {
	let mut grid = get_input();
	for _ in 0..grid.fold_instructions.len() {
		grid.fold();
	}
	println!("Grid:\n{}", grid);
	println!("[*] Day 13 Part 2 Result: TODO");
}

#[derive(Debug)]
enum FoldInstruction {
	HORIZONTAL{row: u32},
	VERTICAL{col: u32},
}

#[derive(Debug, Default)]
struct Grid {
	coords: HashMap<String,bool>,
	fold_instructions: Vec<FoldInstruction>,
	max_x: u32,
	max_y: u32,
}

impl Grid {
	fn fold(&mut self) {
		let cur_instr: Vec<FoldInstruction> = self.fold_instructions.drain(0..1).collect();
		match cur_instr[0] {
			FoldInstruction::HORIZONTAL{row} => {
				for cur_row in row..=self.max_y {
					let y_delta = cur_row - row;
					for cur_col in 0..=self.max_x {
						let cur_key = format_key(cur_col, cur_row);
						if self.coords.contains_key(&cur_key) {
							let new_key = format_key(cur_col, row-y_delta);
							self.coords.insert(new_key, true);
							self.coords.remove(&cur_key);
						}
					}
					self.max_y = row;
				}
			},
			FoldInstruction::VERTICAL{col} => {
				for cur_row in 0..=self.max_y {
					for cur_col in col..=self.max_x {
						let x_delta = cur_col - col;
						let cur_key = format_key(cur_col, cur_row);
						if self.coords.contains_key(&cur_key) {
							let new_key = format_key(col-x_delta, cur_row);
							self.coords.insert(new_key, true);
							self.coords.remove(&cur_key);
						}
					}
					
				}
				self.max_x = col;
			},
		}
	}
}

impl fmt::Display for Grid {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut print_str = String::default();
		for y in 0..=self.max_y {
			let mut y_fold_line = false;
			for instr in &self.fold_instructions {
				match instr {
					FoldInstruction::HORIZONTAL{row} => {
						if y == *row {
							y_fold_line = true;
						}
					},
					_ => (),
				}
			}
			for x in 0..=self.max_x {
				if y_fold_line {
					print_str += "_";
				} else {
					// check to see if there is a vertical fold
					let mut x_fold_line = false;
					for instr in &self.fold_instructions {
						match instr {
							FoldInstruction::VERTICAL{col} => {
								if x == *col {
									x_fold_line = true;
								}
							},
							_ => (),
						}
					}

					if x_fold_line {
						print_str += "|";
					} else {
						let key = format_key(x, y);
						if self.coords.contains_key(&key) {
							print_str += "#";
						} else {
							print_str += ".";
						}
					}
				}
			}
			print_str += "\n";
		}
		write!(f, "{}", print_str)
	}
}

impl From<String> for Grid {
	fn from(grid_str: String) -> Self {
		let mut grid = Grid::default();
		let mut coords_finished = false;
		for line in grid_str.lines() {
			if line == "" {
				coords_finished = true;
			} else {
				if !coords_finished {
					let raw_key: Vec<u32> = line.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
					let mut key = format_key(raw_key[0], raw_key[1]);
					grid.coords.insert(key, true);
					if raw_key[0] > grid.max_x {
						grid.max_x = raw_key[0];
					}
					if raw_key[1] > grid.max_y {
						grid.max_y = raw_key[1];
					}
				} else {

					let re_y = Regex::new(r"^fold along y=\d+?$").unwrap();
					let re_x = Regex::new(r"^fold along x=\d+?$").unwrap();
					let value_vec: Vec<&str> = line.split("=").collect();

					if re_y.is_match(line) {
						grid.fold_instructions.push(FoldInstruction::HORIZONTAL{row: value_vec[1].parse::<u32>().unwrap()});
					} else if re_x.is_match(line) {
						grid.fold_instructions.push(FoldInstruction::VERTICAL{col: value_vec[1].parse::<u32>().unwrap()});
					} else {
						panic!("invalid fold instructions");
					}
				}
			}
		}

		grid
	}
}

fn format_key(x: u32, y: u32) -> String {
	format!("({},{})", x, y)
}