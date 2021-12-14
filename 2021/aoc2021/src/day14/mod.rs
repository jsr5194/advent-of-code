use std::collections::HashMap;
use std::fmt;
use std::fs;

fn get_input() -> Manual {
	let filename = "./src/day14/input_test.txt";
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
	for (k, v) in manual.get_polymer_counts() {
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
	for _ in 0..40 {
		manual.naive_apply_rules();
	}

	let mut high_count = u32::MIN;
	let mut low_count = u32::MAX;
	for (k, v) in manual.get_polymer_counts() {
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
}

impl Manual {
	fn apply_rules(&mut self) {
		println!("TODO");
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

	fn get_polymer_counts(&self) -> HashMap<char, u32> {
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
		manual
	}
}



