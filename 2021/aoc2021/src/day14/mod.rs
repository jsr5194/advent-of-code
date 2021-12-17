use std::collections::HashMap;
use std::fmt;
use std::fs;

fn get_input() -> Manual {
	let filename = "./src/day14/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Manual = Manual::from(contents_str);
	contents
}

/// run Day 14 Part 1
pub fn run_part_1() {
	let mut manual: Manual = get_input();
	for _ in 0..10 {
		manual.naive_apply_rules();
	}

	let mut high_count = u32::MIN;
	let mut low_count = u32::MAX;
	for (k, v) in manual.get_polymer_template_counts() {
		if v > high_count {
			high_count = v;
		}
		if v < low_count {
			low_count = v;
		}
	}

	println!("[*] Day 14 Part 1 Result: {}", high_count-low_count);
}

/// run Day 14 Part 2
pub fn run_part_2() {
	let mut manual: Manual = get_input();
	println!("{:?}", manual);
	for _ in 0..10 {
		manual.apply_rules();
	}
	println!("polymer Count: {:?}", manual.polymer_count);
	let mut high_count = u32::MIN;
	let mut low_count = u32::MAX;
	for (k, v) in manual.polymer_count {
		if v > high_count {
			high_count = v;
		}
		if v < low_count {
			low_count = v;
		}
	}

	println!("[*] Day 14 Part 2 Result: {}", high_count-low_count);
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Manual {
	polymer_template: Vec<char>,
	pair_insertion_rules: HashMap<String, char>,
	pair_count: HashMap<String, u32>,
	polymer_count: HashMap<char, u32>,
}

impl Manual {
	fn apply_rules(&mut self) {
		let mut new_polymer_count: HashMap<char, u32> = HashMap::new();
		let mut new_pair_count: HashMap<String, u32> = HashMap::new(); 
		let mut last_run = false;
		let mut idx = 0;
		let pair_count_size = self.pair_count.len();
		for (pair, count) in &self.pair_count {
			let mut pair_chars = pair.chars();
			let c1 = &pair_chars.next().unwrap();
			let c2 = &pair_chars.next().unwrap();

			let new_polymer = self.pair_insertion_rules.get(pair).unwrap();
			
			let first = format!("{}{}", &c1, &new_polymer);
			if new_pair_count.contains_key(&first) {
				*new_pair_count.get_mut(&first).unwrap() += count;
			} else {
				new_pair_count.insert(first.clone(), *count);
			}
			
			let second = format!("{}{}", &new_polymer, &c2);
			if new_pair_count.contains_key(&second) {
				*new_pair_count.get_mut(&second).unwrap() += count;
			} else {
				new_pair_count.insert(second.clone(), *count);
			}

			// update polymer counts
			if new_polymer_count.contains_key(&c1) {
				*new_polymer_count.get_mut(&c1).unwrap() += count;
			} else {
				new_polymer_count.insert(*c1, *count);
			}
			if idx == pair_count_size-1 {
				if new_polymer_count.contains_key(&c2) {
					*new_polymer_count.get_mut(&c2).unwrap() += 1;
				} else {
					new_polymer_count.insert(*c2, *count);
				}
			}
			if new_polymer_count.contains_key(&new_polymer) {
				*new_polymer_count.get_mut(&new_polymer).unwrap() += count;
			} else {
				new_polymer_count.insert(*new_polymer, *count);
			}
			idx += 1;
		}

		let cloned_polymer_count = self.polymer_count.clone();
		for key in cloned_polymer_count.keys() {
			self.polymer_count.remove(key);
		}

		for (key, count) in new_polymer_count {
			self.polymer_count.insert(key.clone(), count);
		}

		let cloned_pair_count = self.pair_count.clone();
		for key in cloned_pair_count.keys() {
			self.pair_count.remove(key);
		}

		for (pair, count) in new_pair_count {
			self.pair_count.insert(pair.clone(), count);
		}
	}

	fn naive_apply_rules(&mut self) {
		let mut idx = 0;
		loop {
			let cur_pair = format!("{}{}", self.polymer_template[idx], self.polymer_template[idx+1]);
			if self.pair_insertion_rules.contains_key(&cur_pair) {
				self.polymer_template.insert(idx+1, *self.pair_insertion_rules.get(&cur_pair).unwrap());
				idx += 1;
			}
			idx += 1;
			if idx >= self.polymer_template.len()-1 { // -1 to allow for the integrated +1
				break;
			}
		}
	}

	fn get_polymer_template_counts(&self) -> HashMap<char, u32> {
		let mut polymer_count: HashMap<char, u32> = HashMap::new();
		for c in &self.polymer_template {
			if polymer_count.contains_key(&c) {
				*polymer_count.get_mut(&c).unwrap() += 1;
			} else {
				polymer_count.insert(*c, 1);
			}
		}
		polymer_count
	}
}

impl fmt::Display for Manual {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut manual_str = String::default();
		for c in &self.polymer_template {
			manual_str += &c.to_string();
		}
		write!(f, "{}", manual_str)
	}
}

impl From<String> for Manual {
	fn from(manual_str: String) -> Self {
		let mut manual = Manual::default();
		let mut template_obtained = false;
		for line in manual_str.lines() {
			if line == "" {
				template_obtained = true;
			} else {
				if !template_obtained {
					for c in line.chars() {
						manual.polymer_template.push(c);
					}
				} else {
					let mut idx = 0;
					let mut key = String::default();
					let mut value = char::default();
					for res in line.split(" -> ") {
						if idx == 0 {
							key = res.to_string();
						} else if idx == 1 {
							value = res.chars().next().unwrap();
						} else {
							panic!("too many results in insertion rule: {:?}", line);
						}
						idx += 1;
					}
					manual.pair_insertion_rules.insert(key, value);
				}
			}
		}

		// add initial counts
		for idx in 0..manual.polymer_template.len()-1 { //-1 to allow for the +1 access
			manual.pair_count.insert(format!("{}{}", manual.polymer_template[idx], manual.polymer_template[idx+1]), 1);
		}

		manual
	}
}



