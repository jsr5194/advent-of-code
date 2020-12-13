use std::collections::HashMap;
use std::fmt;
use std::fs;

fn get_input() -> Map {
	let filename = "./src/day11/input.txt";
	let contents = fs::read_to_string(filename).expect("could not read file");
	let contents_rows: Vec<&str> = contents.lines().map(|x| x).collect();
	
	let mut map = Map::default();
	let mut row_idx = 0;
	for row in contents_rows {
		let mut col_idx = 0;
		for cell in row.chars() {
			let key = format!("({},{})", col_idx, row_idx);
			match cell {
				'.' => {
					map.grid.insert(key, Cell{x: col_idx, y: row_idx, cell_type: CellType::Floor});
				},
				'L' => {
					map.grid.insert(key, Cell{x: col_idx, y: row_idx, cell_type: CellType::Empty});
				},
				'#' => {
					map.grid.insert(key, Cell{x: col_idx, y: row_idx, cell_type: CellType::Occupied});
				},
				_ => panic!("invalid cell type detected"),
			}
			col_idx += 1;
		}
		map.width = col_idx;
		row_idx += 1;
	}
	map.height = row_idx;

	map
}

pub fn run_part1() {
	let mut map = get_input();
	map.occupied_threshold = 4;
	map.only_adjacent = true;

	loop {
		let last_map = map.clone();
		map.run_round();
		if map.compare_map(&last_map) {
			break;
		}
	}

	let mut occupied_count = 0;
	for x in 0..map.width {
		for y in 0..map.height {
			if map.get_cell(x, y).cell_type == CellType::Occupied {
				occupied_count += 1;
			}
		}
	}

	println!("Day 11 Part 1: {}", occupied_count);

}

pub fn run_part2() {
	let mut map = get_input();
	map.occupied_threshold = 5;
	map.only_adjacent = false;

	loop {
		let last_map = map.clone();
		map.run_round();
		if map.compare_map(&last_map) {
			break;
		}
	}

	let mut occupied_count = 0;
	for x in 0..map.width {
		for y in 0..map.height {
			if map.get_cell(x, y).cell_type == CellType::Occupied {
				occupied_count += 1;
			}
		}
	}

	println!("Day 11 Part 2: {}", occupied_count);

}

#[derive(Debug, Default, Clone)]
struct Map {
	width: usize,
	height: usize,
	occupied_threshold: usize,
	only_adjacent: bool,
	grid: HashMap<String, Cell>,
}

impl Map {
	fn get_cell(&self, x: usize, y: usize) -> Cell {
		let key = format!("({},{})", x, y);
		let cell = self.grid.get(&key).expect("could not get cell");
		cell.clone()
	}

	fn set_cell(&mut self, x: usize, y: usize, value: CellType) {
		let key = format!("({},{})", x, y);
		self.grid.get_mut(&key).unwrap().cell_type = value;
	}

	fn peek_in_direction(&mut self, map_snapshot: &Map, x: usize, y:usize, location: Location) -> usize {
		let mut peek_x: usize = 0;
		let mut peek_y: usize = 0;
		let mut border_hit: bool = false;
		match location {
			Location::TopLeft => {
				// Top Left
				if !x.overflowing_sub(1).1 && !y.overflowing_sub(1).1 {
					peek_x = x-1;
					peek_y = y-1;
				} else {
					border_hit = true;
				}
			},
			Location::TopCenter => {
				// Top Center
				if !y.overflowing_sub(1).1 {
					peek_x = x;
					peek_y = y-1;
				} else {
					border_hit = true;
				}
			},
			Location::TopRight => {
				// Top Right
				if x+1 < map_snapshot.width && !y.overflowing_sub(1).1 {
					peek_x = x+1;
					peek_y = y-1;
				} else {
					border_hit = true;
				}
			},
			Location::CenterLeft => {
				// Center Left
				if !x.overflowing_sub(1).1 {
					peek_x = x-1;
					peek_y = y;
				} else {
					border_hit = true;
				}
			},
			Location::CenterRight => {
				// Center Right
				if x+1 < map_snapshot.width {
					peek_x = x+1;
					peek_y = y;
				} else {
					border_hit = true;
				}
			},
			Location::BottomLeft => {
				// Bottom Left
				if !x.overflowing_sub(1).1 && y+1 < map_snapshot.height {
					peek_x = x-1;
					peek_y = y+1;
				} else {
					border_hit = true;
				}
			},
			Location::BottomCenter => {
				// Bottom Center
				if y+1 < map_snapshot.height {
					peek_x = x;
					peek_y = y+1;
				} else {
					border_hit = true;
				}
			},
			Location::BottomRight => {
				// Bottom Right
				if x+1 < map_snapshot.width && y+1 < map_snapshot.height {
					peek_x = x+1;
					peek_y = y+1;
				} else {
					border_hit = true;
				}
			},
		}

		let mut occupied_count = 0;
		if !border_hit {
			match map_snapshot.get_cell(peek_x, peek_y).cell_type {
				CellType::Occupied => {
					occupied_count = 1;
				},
				CellType::Floor    => {
					if !self.only_adjacent {
						occupied_count = self.peek_in_direction(map_snapshot, peek_x, peek_y, location);
					}
				},
				CellType::Empty => (),
			}
		}

		occupied_count
	}

	fn set_new_seat_state(&mut self, map_snapshot: &Map, cell: &Cell) {
		let mut occupied_count = 0;

		// check 
		occupied_count += self.peek_in_direction(map_snapshot, cell.x, cell.y, Location::TopLeft);
		occupied_count += self.peek_in_direction(map_snapshot, cell.x, cell.y, Location::TopCenter);
		occupied_count += self.peek_in_direction(map_snapshot, cell.x, cell.y, Location::TopRight);
		occupied_count += self.peek_in_direction(map_snapshot, cell.x, cell.y, Location::CenterLeft);
		occupied_count += self.peek_in_direction(map_snapshot, cell.x, cell.y, Location::CenterRight);
		occupied_count += self.peek_in_direction(map_snapshot, cell.x, cell.y, Location::BottomLeft);
		occupied_count += self.peek_in_direction(map_snapshot, cell.x, cell.y, Location::BottomCenter);
		occupied_count += self.peek_in_direction(map_snapshot, cell.x, cell.y, Location::BottomRight);


		// If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
		// If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
		match map_snapshot.get_cell(cell.x, cell.y).cell_type {
			CellType::Occupied => {
				if occupied_count >= self.occupied_threshold {
					self.set_cell(cell.x, cell.y, CellType::Empty);
				}
			},
			CellType::Empty    => {
				if occupied_count == 0 {
					self.set_cell(cell.x, cell.y, CellType::Occupied);
				}
			},
			CellType::Floor    => (),
		}
	}

	fn compare_map(&self, comparison_map: &Map) -> bool {
		let mut maps_match = true;
		for x in 0..self.width {
			for y in 0..self.height {
				if self.get_cell(x, y).cell_type != comparison_map.get_cell(x, y).cell_type {
					maps_match = false;
					break; 
				}
			}
			if !maps_match {
				break;
			}
		}

		maps_match
	}

	fn run_round(&mut self) {
		let map_snapshot = self.clone();
		for x in 0..self.width {
			for y in 0..self.height {
				self.set_new_seat_state(&map_snapshot, &self.get_cell(x, y));
			}
		}
	}
}

// implement a pretty print view for the map
impl fmt::Display for Map {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut print_grid = String::default();

		for y in 0..self.height {
			for x in 0..self.width {
				match self.get_cell(x, y).cell_type {
					CellType::Occupied => print_grid += "#",
					CellType::Empty    => print_grid += "L",
					CellType::Floor    => print_grid += ".",
				}
			}
			print_grid += "\n";
		}
		write!(f, "{}", print_grid)
	}
}

#[derive(Debug, Clone)]
struct Cell {
	x: usize,
	y: usize,
	cell_type: CellType,
}

#[derive(Debug, Clone, PartialEq)]
enum CellType {
	Occupied,
	Empty,
	Floor,
}

#[derive(Debug, Clone, PartialEq)]
enum Location {
	TopLeft,
	TopCenter,
	TopRight,
	CenterLeft,
	CenterRight,
	BottomLeft,
	BottomCenter,
	BottomRight,
}