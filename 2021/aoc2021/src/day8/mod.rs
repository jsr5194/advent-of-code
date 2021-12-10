use std::collections::HashMap;
use std::fmt;
use std::fs;

fn get_input() -> Notes {
	let filename = "./src/day8/input.txt";
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
	for entry in &mut notes.entries {
		entry.decode();
	}

	let mut output_value_sum = 0;
	for entry in notes.entries {
		let mut output_value = String::default();
		for output in entry.output {
			output_value += &output.get_value().to_string();
		}
		output_value_sum += output_value.parse::<u32>().unwrap();
	}
	println!("[*] Day 8 Part 2 Result: {}", output_value_sum);
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
	map: HashMap<char, char>,
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

	fn decode(&mut self) {
		let mut horizontals: Vec<char> = vec![];
		let mut verticals: Vec<char> = vec![];

		// get known displays
		let one = self.get_pattern(1);
		let four = self.get_pattern(4);
		let seven = self.get_pattern(7);
		let eight = self.get_pattern(8);

		// get all displays with 5 segments
		// the 3 common entries are the horizontals
		let mut five_seg_group: Vec<SevenSegmentDisplay> = vec![];
		for display in &self.patterns {
			if display.get_lit_segment_count() == 5 {
				five_seg_group.push(display.clone());
			}
		}
		for seg_i in five_seg_group[0].get_lit_segments() {
			for seg_j in five_seg_group[1].get_lit_segments() {
				for seg_k in five_seg_group[2].get_lit_segments() {
					if seg_i == seg_j && seg_j == seg_k {
						horizontals.push(seg_k);
					}
				}
			}
		}

		//// get display 8
		// the segments not in horizontals are the verticals
		for segment in eight.get_lit_segments() {
			if !horizontals.contains(&segment) {
				verticals.push(segment);
			}
		}

		//// a: only horizontal in display7
		for segment in seven.get_lit_segments() {
			if horizontals.contains(&segment) {
				self.map.insert(segment, 'a');
				break;
			}
		}

		//// b: only vertical in display4 that isnt in display1
		for s4 in four.get_lit_segments() {
			if !one.get_lit_segments().contains(&s4) {
				self.map.insert(s4, 'b');
			}
		}

		//// c: only vertical in display8 that isnt in display6
		// get all displays with six lit segments
		let mut six_lit_displays: Vec<SevenSegmentDisplay> = vec![];
		for display in &self.patterns {
			if display.get_lit_segment_count() == 6 {
				six_lit_displays.push(display.clone());
			}
		}
		// display6 is the one that does not contain every vertical in display1
		let mut display_6_group: Vec<SevenSegmentDisplay> = vec![];
		for display in &six_lit_displays {
			for s1 in one.get_lit_segments() {
				if verticals.contains(&s1) {
					if !display.get_lit_segments().contains(&s1) {
						display_6_group.push(display.clone());
						break;
					}
				}
			}
		}
		let six: SevenSegmentDisplay = display_6_group[0].clone();
		// diff display6 and dipslay8
		for segment in eight.get_lit_segments() {
			if !six.get_lit_segments().contains(&segment) {
				self.map.insert(segment, 'c');
			}
		}

		//// d: only horizontal in display4
		for segment in four.get_lit_segments() {
			if horizontals.contains(&segment) {
				self.map.insert(segment, 'd');
				break;
			}
		}

		//// e: only vertical in display8 that isnt in display9
		// display9 is the remaining one in `six_lit_displays` that has all 3 horizontals
		let mut display_9_group: Vec<SevenSegmentDisplay> = vec![];
		for display in &six_lit_displays {
			let mut horiz_count = 0;
			for segment in display.get_lit_segments() {
				if horizontals.contains(&segment) {
					horiz_count += 1;
				}
			}
			if horiz_count == 3 && *display != six {
				display_9_group.push(display.clone());
			}
		}
		let nine: SevenSegmentDisplay = display_9_group[0].clone();
		for s8 in eight.get_lit_segments() {
			if !nine.get_lit_segments().contains(&s8) {
				self.map.insert(s8, 'e');
				break;
			}
		}

		//// g: only horizontal not yet in the map
		for h in &horizontals {
			let mut contains_h = false;
			for key in self.map.keys() {
				if h == key {
					contains_h = true;
				}
			}
			if !contains_h {
				self.map.insert(*h, 'g');
			}
		}

		//// f: only value not yet in the map
		let options = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
		let mut f_group: Vec<char> = vec![];
		for option in options.iter() {
			let mut found = false;
			for key in self.map.keys() {
				if key == option {
					found = true;
				}
			}
			if !found{
				f_group.push(*option);
				break;
			}
		}
		self.map.insert(f_group[0], 'f');

		// rebuild the display
		for display in &mut self.output {
			let mut new_display = SevenSegmentDisplay::default();
			for segment in display.get_lit_segments() {
				let new_segment = self.map.get(&segment).unwrap();
				match new_segment {
					'a' => new_display.a = true,
					'b' => new_display.b = true,
					'c' => new_display.c = true,
					'd' => new_display.d = true,
					'e' => new_display.e = true,
					'f' => new_display.f = true,
					'g' => new_display.g = true,
					_ => panic!("invalid segment"),
				}
			}
			display.set_to(new_display.get_value());
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



