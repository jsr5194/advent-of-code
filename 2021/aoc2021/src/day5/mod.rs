use std::fs;

fn get_part1_input() -> Part1Map {
	let filename = "./src/day5/input_test.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Part1Map = Part1Map::from(contents_str);
	contents
}

fn get_part2_input() -> Part2Map {
	let filename = "./src/day5/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Part2Map = Part2Map::from(contents_str);
	contents
}

/// run Day 5 Part 1
pub fn run_part_1() {
	let _map = get_part1_input();
//	let mut overlaps: u32 = 0;
//	for coord in map.coords { 
//		if coord.vent_count > 1 {
//			overlaps += 1;
//		}
//	}
//	println!("[*] Day 5 Part 1 Result: {}", overlaps);
	println!("[*] Day 5 Part 1 Result: skipped");
}

/// run Day 5 Part 2
pub fn run_part_2() {
	let map = get_part2_input();
	let mut overlaps: u32 = 0;
	for coord in map.coords { 
		if coord.vent_count > 1 {
			overlaps += 1;
		}
	}
	println!("[*] Day 5 Part 2 Result: {}", overlaps);
}


#[derive(Default, Debug, Clone, PartialEq)]
struct Coordinate {
	x: u32,
	y: u32,
	vent_count: u32,
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Part1Map {
	coords: Vec<Coordinate>,
}

impl From<String> for Part1Map {
	fn from(map_str: String) -> Self {
		let mut map = Part1Map::default();

		// iterate over each of the line segment definitions
		for line in map_str.lines() {
			let coord_group: Vec<&str> = line.split(" -> ").collect();
			let c1: Vec<u32> = coord_group[0].split(",").map(|x| x.parse::<u32>().unwrap()).collect();
			let c2: Vec<u32> = coord_group[1].split(",").map(|x| x.parse::<u32>().unwrap()).collect();

			// For now, only consider horizontal and vertical lines
			if c1[0] == c2[0] || c1[1] == c2[1] {
				let start_x;
				let end_x;

				if c1[0] > c2[0] {
					start_x = c2[0];
					end_x = c1[0];
				} else {
					start_x = c1[0];
					end_x = c2[0];
				}

				let start_y;
				let end_y;
				if c1[1] > c2[1] {
					start_y = c2[1];
					end_y = c1[1];
				} else {
					start_y = c1[1];
					end_y = c2[1];
				}

				for x in start_x..=end_x {
					for y in start_y..=end_y {
						let cur_coord = Coordinate{x: x, y: y, vent_count: 1};
						if map.contains(&cur_coord) {
							map.increase_vent_count(&cur_coord);
						} else {
							map.coords.push(cur_coord.clone());
						}
					}
				}
			}
		}

		map
	}
}

impl Part1Map {
	fn contains(&self, check_coord: &Coordinate) -> bool {
		for cur_coord in &self.coords {
			if check_coord.x == cur_coord.x && check_coord.y == cur_coord.y {
				return true
			}
		}
		false
	}

	fn increase_vent_count(&mut self, check_coord: &Coordinate) {
		for cur_coord in &mut self.coords {
			if check_coord.x == cur_coord.x && check_coord.y == cur_coord.y {
				cur_coord.vent_count += 1;
				break;
			}
		}
	}
}




#[derive(Default, Debug, Clone, PartialEq)]
struct Part2Map {
	coords: Vec<Coordinate>,
}

impl From<String> for Part2Map {
	fn from(map_str: String) -> Self {
		let mut map = Part2Map::default();

		// iterate over each of the line segment definitions
		for line in map_str.lines() {
			let coord_group: Vec<&str> = line.split(" -> ").collect();
			let c1: Vec<u32> = coord_group[0].split(",").map(|x| x.parse::<u32>().unwrap()).collect();
			let c2: Vec<u32> = coord_group[1].split(",").map(|x| x.parse::<u32>().unwrap()).collect();

			let mut cur_x = c1[0];
			let mut cur_y = c1[1];

			// lower x to higher x, lower y to higher or equals y
			// ex: 1,1 -> 3,3
			if c1[0] <= c2[0] && c1[1] <= c2[1] {
				loop {
					let cur_coord = Coordinate{x: cur_x, y: cur_y, vent_count: 1};
					//println!("adding {},{}", cur_coord.x, cur_coord.y);
					if map.contains(&cur_coord) {
						map.increase_vent_count(&cur_coord);
					} else {
						map.coords.push(cur_coord.clone());
					}

					if cur_x == c2[0] && cur_y == c2[1] {
						break;
					}

					if cur_x != c2[0] {
						cur_x += 1;
					}

					if cur_y != c2[1] {
						cur_y += 1;
					}
				}
			}
			// lower x to higher x, higher y to lower y
			// ex: 5,5 -> 8,2
			else if c1[0] <= c2[0] && c1[1] >= c2[1] {
				loop {
					let cur_coord = Coordinate{x: cur_x, y: cur_y, vent_count: 1};
					//println!("adding {},{}", cur_coord.x, cur_coord.y);
					if map.contains(&cur_coord) {
						map.increase_vent_count(&cur_coord);
					} else {
						map.coords.push(cur_coord.clone());
					}

					if cur_x == c2[0] && cur_y == c2[1] {
						break;
					}

					if cur_x != c2[0] {
						cur_x += 1;
					}

					if cur_y != c2[1] {
						cur_y -= 1;
					}
				}
			}
			// higher x to lower x, lower y to higher y
			// ex: 9,7 -> 7,9
			else if c1[0] >= c2[0] && c1[1] <= c2[1] {
				loop {
					let cur_coord = Coordinate{x: cur_x, y: cur_y, vent_count: 1};
					//println!("adding {},{}", cur_coord.x, cur_coord.y);
					if map.contains(&cur_coord) {
						map.increase_vent_count(&cur_coord);
					} else {
						map.coords.push(cur_coord.clone());
					}

					if cur_x == c2[0] && cur_y == c2[1] {
						break;
					}
					if cur_x != c2[0] {
						cur_x -= 1;
					}

					if cur_y != c2[1] {
						cur_y += 1;
					}
				}
			}
			// higher x to lower x, higher y to lower y
			// ex: 6,4 -> 2,0
			else if c1[0] >= c2[0] && c1[1] >= c2[1] {
				loop {
					let cur_coord = Coordinate{x: cur_x, y: cur_y, vent_count: 1};
					//println!("adding {},{}", cur_coord.x, cur_coord.y);
					if map.contains(&cur_coord) {
						map.increase_vent_count(&cur_coord);
					} else {
						map.coords.push(cur_coord.clone());
					}

					if cur_x == c2[0] && cur_y == c2[1] {
						break;
					}

					if cur_x != c2[0] {
						cur_x -= 1;
					}

					if cur_y != c2[1] {
						cur_y -= 1;
					}
				}
			}
		}

		map
	}
}

impl Part2Map {
	fn contains(&self, check_coord: &Coordinate) -> bool {
		for cur_coord in &self.coords {
			if check_coord.x == cur_coord.x && check_coord.y == cur_coord.y {
				return true
			}
		}
		false
	}

	fn increase_vent_count(&mut self, check_coord: &Coordinate) {
		for cur_coord in &mut self.coords {
			if check_coord.x == cur_coord.x && check_coord.y == cur_coord.y {
				cur_coord.vent_count += 1;
				break;
			}
		}
	}
}