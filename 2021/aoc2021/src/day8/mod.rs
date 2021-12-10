use std::collections::HashMap;
use std::fmt;
use std::fs;

fn get_input() -> Notes {
	let filename = "./src/day8/input_test_1.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Notes = Notes::from(contents_str);
	contents
}

/// run Day 8 Part 1
pub fn run_part_1() {
	// get the formatted input
	let mut notes: Notes = get_input();
	
	// decode all entries
	let mut result = 0;
	for entry in &mut notes.entries {
		entry.naive_decode();
		for display in &entry.output {
			if display.decoded {
				match display.get_value() {
					1 => result += 1,
					4 => result += 1,
					7 => result += 1,
					8 => result += 1,
					_ => {}
				}
			}
		}
	}

	println!("[*] Day 8 Part 1 Result: {}", result);
}

/// run Day 8 Part 2
pub fn run_part_2() {
	let mut notes: Notes = get_input();
	let mut result = 0;
	for entry in &mut notes.entries {
		entry.decode();
	}
	println!("[*] Day 8 Part 2 Result: TODO");
}


#[derive(Debug, Default, Clone, PartialEq)]
struct SevenSegmentDisplay {
	a: bool,
	b: bool,
	c: bool,
	d: bool,
	e: bool,
	f: bool,
	g: bool,
	decoded: bool,
}

impl SevenSegmentDisplay {
	fn get_lit_segments(&self) -> Vec<char> {
		let mut lit_segments: Vec<char> = vec![];
		if self.a {
			lit_segments.push('a');
		}
		if self.b {
			lit_segments.push('b');
		}
		if self.c {
			lit_segments.push('c');
		}
		if self.d {
			lit_segments.push('d');
		}
		if self.e {
			lit_segments.push('e');
		}
		if self.f {
			lit_segments.push('f');
		}
		if self.g {
			lit_segments.push('g');
		}

		lit_segments
	}

	fn get_lit_segment_count(&self) -> usize {
		self.get_lit_segments().len()
	}

	fn set_to(&mut self, value: u32) {
		// reset all segments
		self.a = false;
		self.b = false;
		self.c = false;
		self.d = false;
		self.e = false;
		self.f = false;
		self.g = false;

		match value {
			0 => {
				self.a = true;
				self.b = true;
				self.c = true;
				self.e = true;
				self.f = true;
				self.g = true;
			},
			1 => {
				self.c = true;
				self.f = true;
			},
			2 => {
				self.a = true;
				self.c = true;
				self.d = true;
				self.e = true;
				self.g = true;
			},
			3 => {
				self.a = true;
				self.c = true;
				self.d = true;
				self.f = true;
				self.g = true;
			},
			4 => {
				self.b = true;
				self.c = true;
				self.d = true;
				self.f = true;
			},
			5 => {
				self.a = true;
				self.b = true;
				self.d = true;
				self.f = true;
				self.g = true;
			},
			6 => {
				self.a = true;
				self.b = true;
				self.d = true;
				self.e = true;
				self.f = true;
				self.g = true;
			},
			7 => {
				self.a = true;
				self.c = true;
				self.f = true;
			},
			8 => {
				self.a = true;
				self.b = true;
				self.c = true;
				self.d = true;
				self.e = true;
				self.f = true;
				self.g = true;
			},
			9 => {
				self.a = true;
				self.b = true;
				self.c = true;
				self.d = true;
				self.f = true;
				self.g = true;
			},
			_ => panic!("cannot set segment to: {:?}", value),
		}
	}

	fn get_value(&self) -> u32 {
		let mut value;
		if self.a && self.b && self.c && !self.d && self.e && self.f && self.g {
			value = 0;
		}
		else if !self.a && !self.b && self.c && !self.d && !self.e && self.f && !self.g {
			value = 1;
		}
		else if self.a && !self.b && self.c && self.d && self.e && !self.f && self.g {
			value = 2;
		}
		else if self.a && !self.b && self.c && self.d && !self.e && self.f && self.g {
			value = 3;
		}
		else if !self.a && self.b && self.c && self.d && !self.e && self.f && !self.g {
			value = 4;
		}
		else if self.a && self.b && !self.c && self.d && !self.e && self.f && self.g {
			value = 5;
		}
		else if self.a && self.b && !self.c && self.d && self.e && self.f && self.g {
			value = 6;
		}
		else if self.a && !self.b && self.c && !self.d && !self.e && self.f && !self.g {
			value = 7;
		}
		else if self.a && self.b && self.c && self.d && self.e && self.f && self.g {
			value = 8;
		}
		else if self.a && self.b && self.c && self.d && !self.e && self.f && self.g {
			value = 9;
		}
		else {
			panic!("segment is messed up: {:?}", self);
		}

		value
	}
}

impl From<String> for SevenSegmentDisplay {
	fn from(display_str: String) -> Self {
		let mut disp = SevenSegmentDisplay::default();
		for c in display_str.chars() {
			match c {
				'a' => disp.a = true,
				'b' => disp.b = true,
				'c' => disp.c = true,
				'd' => disp.d = true,
				'e' => disp.e = true,
				'f' => disp.f = true,
				'g' => disp.g = true,
				_ => panic!("invalid display segment provided"),
			}
		}
		disp
	}
}

impl fmt::Display for SevenSegmentDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	let mut display_str = String::default();
    	if self.a {
    		display_str += "a";
    	}
    	if self.b {
    		display_str += "b";
    	}
    	if self.c {
    		display_str += "c";
    	}
    	if self.d {
    		display_str += "d";
    	}
    	if self.e {
    		display_str += "e";
    	}
    	if self.f {
    		display_str += "f";
    	}
    	if self.g {
    		display_str += "g";
    	}
        write!(f, "{}", display_str)
    }
}


#[derive(Debug, Default, Clone, PartialEq)]
struct Signals {
	patterns: Vec<SevenSegmentDisplay>,
	output: Vec<SevenSegmentDisplay>,
	key: HashMap<char, char>,
}

impl Signals {
	fn get_pattern(&self, pattern: u32) -> SevenSegmentDisplay {
		for segment in &self.patterns {
			if segment.decoded {
				if segment.get_value() == pattern {
					return segment.clone()
				}
			} else {
				match pattern {
					1 => {
						if segment.get_lit_segment_count() == 2 {
							return segment.clone()
						}
					},
					4 => {
						if segment.get_lit_segment_count() == 4 {
							return segment.clone()
						}
					},
					7 => {
						if segment.get_lit_segment_count() == 3 {
							return segment.clone()
						}
					},
					8 => {
						if segment.get_lit_segment_count() == 7 {
							return segment.clone()
						}
					},
					_ => {},
				}
			}
		}
		panic!("segment not found");
	}

	fn decode (&mut self) {
		let mut verticals: Vec<char> = vec![];
		let mut horizontals: Vec<char> = vec![];

		let one = self.get_pattern(1);
		let seven = self.get_pattern(7);

		// a: the odd segment out between 1 and 7 will give you a
		for segment in seven.get_lit_segments() {
			if one.get_lit_segments().contains(&segment) {
				if !verticals.contains(&segment) {
					verticals.push(segment);
				}
			} else {
				if !horizontals.contains(&segment) {
					horizontals.push(segment);
				}
				self.key.insert('a', segment);
			}
		}

		println!("\ngot a");
		println!("key: {:?}", self.key);
		println!("horizontals: {:?}", horizontals);
		println!("verticals: {:?}\n", verticals);

		// find 2, 3, and 5 - the three segments in all of them will give you the three horizontal segments but not in order
		let mut group235: Vec<SevenSegmentDisplay> = vec![];
		for segment in &self.patterns {
			if segment.get_lit_segment_count() == 5 {
				group235.push(segment.clone());
			}
		}
		for s0 in group235[0].get_lit_segments() {
			for s1 in group235[1].get_lit_segments() {
				if s0 == s1 {
					if group235[2].get_lit_segments().contains(&s0) {
						if !horizontals.contains(&s0) {
							horizontals.push(s0);
						}
					}
				}
			}
		}

		println!("\ngot all horizontals");
		println!("key: {:?}", self.key);
		println!("horizontals: {:?}", horizontals);
		println!("verticals: {:?}\n", verticals);


// d	// find 4 - once you have all of the horizontal segments you can tell one is segment d since a is already taken and g doesn't exist here
		let four = self.get_pattern(4);
		for segment in four.get_lit_segments() {
			if horizontals.contains(&segment) {
				let mut found = false;
				for (_, v) in self.key.iter() {
					if *v == segment {
						found = true;
					}
				}
				if !found {
					self.key.insert('d', segment);
				}
			} else {
				if !verticals.contains(&segment) {
					verticals.push(segment);
				}
			}
		}

		println!("\ngot d and another vertical");
		println!("key: {:?}", self.key);
		println!("horizontals: {:?}", horizontals);
		println!("verticals: {:?}\n", verticals);

// g	// g is the last remaining horizontal
		for h in &horizontals {
			let mut found = false;
			for (_, v) in self.key.iter() {
				if v == h {
					found = true;
				}
			}
			if !found {
				self.key.insert('g', *h);
				break;
			}
		}
		println!("\ngot g");
		println!("key: {:?}", self.key);
		println!("horizontals: {:?}", horizontals);
		println!("verticals: {:?}\n", verticals);

// b	// find 1 and 4 - once you have d, figuring out b is process of elimination
		for h in &verticals {
			let mut found = false;
			for (_, v) in self.key.iter() {
				if v == h {
					found = true;
				}
			}
			if !found {
				self.key.insert('b', *h);
				break;
			}
		}
		println!("\ngot b");
		println!("key: {:?}", self.key);
		println!("horizontals: {:?}", horizontals);
		println!("verticals: {:?}\n", verticals);


// e	// find 0 and 1 -once you have all horizontals, segment b, and the flippable position of c and f you can get e
		let mut group0: Vec<SevenSegmentDisplay> = vec![];
		for display in &self.patterns {
			if display.get_lit_segment_count() == 6 {
				let mut horiz_count = 0;
				for segment in display.get_lit_segments() {
					if horizontals.contains(&segment) {
						horiz_count += 1;
					}
				}
				if horiz_count == 2 {
					group0.push(display.clone());
				}
			}
		}

		let zero = &group0[0];
		for segment in &zero.get_lit_segments() {
			// can't be a horizontal 
			if !horizontals.contains(&segment) {
				// can't be in one
				if one.get_lit_segments().contains(&segment) {
					// can't be a known key
					let mut found = false;
					for (_, v) in self.key.iter() {
						if v == segment {
							found = true;
						}
					}
					if !found {
						self.key.insert('e', *segment);
						if !verticals.contains(&segment) {
							verticals.push(*segment);
						}
						break;
					}
				}
			}
		}

		println!("\ngot e");
		println!("key: {:?}", self.key);
		println!("horizontals: {:?}", horizontals);
		println!("verticals: {:?}\n", verticals);

		// find 6 adn by process of elimination you can get f
		let mut group6: Vec<SevenSegmentDisplay> = vec![];
		for display in &self.patterns {
			if display.get_lit_segment_count() == 6 {
				let mut horiz_count = 0;
				for segment in display.get_lit_segments() {
					if horizontals.contains(&segment) {
						horiz_count += 1;
					}
				}
				if horiz_count == 3 {
					// make sure it does not contain the same as one
					let mut one_count = 0;
					for segment in display.get_lit_segments() {
						if one.get_lit_segments().contains(&segment) {
							one_count += 1;
						}
					}

					if one_count == 1 {
						group6.push(display.clone());
					}
				}
			}
		}

		let six = &group6[0];
		
		// can't be a known key
		for segment in six.get_lit_segments() {
			let mut found = false;
			for (_, v) in self.key.iter() {
				if *v == segment {
					found = true;
				}
			}
			if !found {
				self.key.insert('f', segment);
				if !verticals.contains(&segment) {
					verticals.push(segment);
				}
				break;
			}
		}

		println!("\ngot f");
		println!("key: {:?}", self.key);
		println!("horizontals: {:?}", horizontals);
		println!("verticals: {:?}\n", verticals);


		// last remaining entry can be pulled from eight
		let eight = self.get_pattern(8);
		for segment in eight.get_lit_segments() {
			let mut found = false;
			for (_, v) in self.key.iter() {
				if *v == segment {
					found = true;
				}
			}
			if !found {
				self.key.insert('c', segment);
				if !verticals.contains(&segment) {
					verticals.push(segment);
				}
				break;
			}
		}

		println!("\ngot c");
		println!("key: {:?}", self.key);
		println!("horizontals: {:?}", horizontals);
		println!("verticals: {:?}\n", verticals);


		// flip b and e to check
		let old_b = self.key.get(&'b').unwrap().clone();
		let old_e = self.key.get(&'e').unwrap().clone();
		self.key.insert('e', old_b);
		self.key.insert('b', old_e);


		// full map is built, so now just iterate through the outputs and rebuild
		for display in &mut self.output {
			let mut new_display_str: String = String::default();
			for segment in display.get_lit_segments() {
				for (k, v) in self.key.iter() {
					if *v == segment {
						new_display_str += &k.to_string();
					}
				}
			}

			println!("display {}", display);
			println!("str {:?}", new_display_str);
			display.set_to(SevenSegmentDisplay::from(new_display_str).get_value());
			display.decoded = true;
		}
	}

	fn naive_decode(&mut self) {
		for display in &mut self.output {
			// 1
			if display.get_lit_segment_count() == 2 {
				display.set_to(1);
				display.decoded = true;
			}

			// 4
			else if display.get_lit_segment_count() == 4 {
				display.set_to(4);
				display.decoded = true;
			}

			// 7
			else if display.get_lit_segment_count() == 3 {
				display.set_to(7);
				display.decoded = true;
			}

			// 8
			else if display.get_lit_segment_count() == 7 {
				display.set_to(8);
				display.decoded = true;
			}
		}
	}
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Notes {
	entries: Vec<Signals>,
}

impl From<String> for Notes {
	fn from(entry_str: String) -> Self {
		let mut notes = Notes::default();
		for line in entry_str.lines() {
			let mut cur_signals = Signals::default();
			let mut in_output = false;
			for disp_str in line.split_whitespace() {
				if disp_str == "|" {
					in_output = true;
				} else {
					if in_output {
						cur_signals.output.push(SevenSegmentDisplay::from(disp_str.to_string()));
					} else {
						cur_signals.patterns.push(SevenSegmentDisplay::from(disp_str.to_string()));
					}
				}
			}
			notes.entries.push(cur_signals);
		}
		notes
	}
}



