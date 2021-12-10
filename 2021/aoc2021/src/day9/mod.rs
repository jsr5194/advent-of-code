use std::fmt;
use std::fs;

fn get_input() -> Map {
	let filename = "./src/day9/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Map = Map::from(contents_str);
	contents
}

/// run Day 9 Part 1
pub fn run_part_1() {
	let mut map: Map = get_input();

	for y in 0..=map.max_y {
		for x in 0..=map.max_x {
			let mut cur_point = map.get_point(x, y);
			if x == 0 && y == 0 {
				// top left
				println!("top left: {}", cur_point);
				if map.get_point(x+1, y) <= cur_point {
					continue;
				}
				if map.get_point(x, y+1) <= cur_point {
					continue;
				}
				map.low_points.push(cur_point);
			} else if x == map.max_x && y == 0 {
				// top right
				println!("top right: {}", cur_point);
				if map.get_point(x-1, y) <= cur_point {
					continue;
				}
				if map.get_point(x, y+1) <= cur_point {
					continue;
				}
				map.low_points.push(cur_point);
			} else if x == 0 && y == map.max_y {
				// bottom left
				println!("bottom left: {}\nx=={}\ny=={}", cur_point, x, y);
				if map.get_point(x+1, y) <= cur_point {
					continue;
				}
				if map.get_point(x, y-1) <= cur_point {
					continue;
				}
				map.low_points.push(cur_point);
			} else if x == 0 {
				// left side
				println!("left side: {}", cur_point);
				if map.get_point(x+1, y) <= cur_point {
					continue;
				}
				if map.get_point(x, y-1) <= cur_point {
					continue;
				}
				if map.get_point(x, y+1) <= cur_point {
					continue;
				}
				map.low_points.push(cur_point);
			} else if y == 0 {
				// top row
				println!("top row: {}", cur_point);
				if map.get_point(x-1, y) <= cur_point {
					continue;
				}
				if map.get_point(x+1, y) <= cur_point {
					continue;
				}
				if map.get_point(x, y+1) <= cur_point {
					continue;
				}
				map.low_points.push(cur_point);
			} else if x == map.max_x {
				// right row
				println!("right row: {}", cur_point);
				if map.get_point(x-1, y) <= cur_point {
					continue;
				}
				if map.get_point(x, y-1) <= cur_point {
					continue;
				}
				if map.get_point(x, y+1) <= cur_point {
					continue;
				}
				map.low_points.push(cur_point);
			} else if x == map.max_x && y == map.max_y {
				// bottom right
				println!("bottom right: {}", cur_point);
				if map.get_point(x-1, y) <= cur_point {
					continue;
				}
				if map.get_point(x, y-1) <= cur_point {
					continue;
				}
				map.low_points.push(cur_point);
			}  else if y == map.max_y {
				// bottom row
				println!("bottom row: {}", cur_point);
				if map.get_point(x-1, y) <= cur_point {
					continue;
				}
				if map.get_point(x+1, y) <= cur_point {
					continue;
				}
				if map.get_point(x, y-1) <= cur_point {
					continue;
				}
				map.low_points.push(cur_point);
			} else {
				println!("middle: {}", cur_point);
				if map.get_point(x-1, y) <= cur_point {
					continue;
				}
				if map.get_point(x+1, y) <= cur_point {
					continue;
				}
				if map.get_point(x, y-1) <= cur_point {
					continue;
				}
				if map.get_point(x, y+1) <= cur_point {
					continue;
				}
				map.low_points.push(cur_point);
			}
		}
	}

	let mut risk_level_sum = 0;
	for point in map.low_points {
		risk_level_sum += point;
		risk_level_sum += 1;
	}
	println!("[*] Day 9 Part 1 Result: {}", risk_level_sum);
}

/// run Day 9 Part 2
pub fn run_part_2() {
	println!("[*] Day 9 Part 2 Result: TODO");
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Map {
	points: Vec<Vec<usize>>,
	max_x: usize,
	max_y: usize,
	low_points: Vec<usize>,
}

impl Map {
	fn get_point(&self, x: usize, y: usize) -> usize {
		self.points[y][x]
	}
}

impl fmt::Display for Map {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut map_str = String::default();
		for row in &self.points {
			for point in row {
				map_str += format!("{}", point).as_str();
			}
			map_str += "\n";
		}
		write!(f, "{}", map_str)
	}
}

impl From<String> for Map {
	fn from(map_str: String) -> Self {
		let mut map = Map::default();
		let mut idx = 0;
		for line in map_str.lines() {
			map.points.push(vec![]);
			for point in line.chars() {
				map.points[idx].push(point.to_string().parse::<usize>().unwrap());
			}
			idx += 1;
		}
		map.max_x = map.points[0].len() - 1;
		map.max_y = map.points.len() - 1;
		map
	}
}
