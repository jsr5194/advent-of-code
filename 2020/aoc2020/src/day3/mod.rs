use std::fmt;
use std::fs;

fn get_input() -> Map {
	let filename = "./src/day3/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: could not read file to string");

	let mut map = Map {
		height: 0,
		width: 0,
		grid: vec![],
	};

	for line in contents_str.lines() {
		// allows us to perform some work on only the first run since width will be set after
		if map.width == 0 {
			map.width = line.len();
		}

		for point in line.chars() {
			match point {
				'.' => map.grid.push(true),
				'#' => map.grid.push(false),
				_ => panic!("[!] ERROR: invalid map element detected"),
			}
		}

		// account for the new row
		map.height += 1;
	}
	map
}

pub fn run_part1() {
	let map = get_input();

	let mut x = 0;
	let mut y = 0;
	let mut num_trees = 0;
	let slope = Slope{x:3, y:1};
	loop {
		if y >= map.height {
			break;
		}

		if !map.is_safe_space(x, y) {
			num_trees += 1;
		}

		x += slope.x;
		y += slope.y;
	}

	println!("Day 3 Part 1 Result: {}", num_trees);
}

pub fn run_part2() {
	let map = get_input();

	let mut result = 1;

	let mut slopes: Vec<Slope> = vec![];
	slopes.push(Slope{x:1, y:1});
	slopes.push(Slope{x:3, y:1});
	slopes.push(Slope{x:5, y:1});
	slopes.push(Slope{x:7, y:1});
	slopes.push(Slope{x:1, y:2});

	for slope in slopes {
		let mut x = 0;
		let mut y = 0;
		let mut num_trees = 0;

		loop {
			if y >= map.height {
				break
			}

			if !map.is_safe_space(x, y) {
				num_trees += 1;
			}

			x += slope.x;
			y += slope.y;
		}
		result *= num_trees;
	}
	println!("Day 3 Part 2 Result: {}", result);
}

#[derive(Debug, Default)]
struct Map {
	grid: Vec<bool>,
	height: usize,
	width: usize,
}

// implement a pretty print view for the map
impl fmt::Display for Map {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut idx = 0;
		let mut print_grid: String = "".to_string();
		for point in &self.grid {
			if *point {
				print_grid += ".";
			} else {
				print_grid += "#";
			}

			idx += 1;
			if idx % self.width == 0 {
				print_grid += "\n";
			}
		}
		write!(f, "{}", print_grid)
	}
}

// create accessor methods to stramline indexing
impl Map {
	fn is_safe_space(&self, raw_x: usize, y: usize) -> bool {
		// since the pattern can repeat on the x axis, just mod the value to get the position within our window
		let mut x = raw_x;
		if x >= self.width {
			x = x % self.width;
		}

		// since everything is in a 1d array we need to shift accordingly
		let index: usize = (y * self.width) + x;
		self.grid[index]
	}
}

// wrap the slope into a struct to allow easier batch testing
struct Slope {
	x: usize,
	y: usize,
}




