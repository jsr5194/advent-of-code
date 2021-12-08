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
	fn get_lit_segment_count(&self) -> u32 {
		let mut lit_segment_count: u32 = 0;
		if self.a {
			lit_segment_count += 1;
		}
		if self.b {
			lit_segment_count += 1;
		}
		if self.c {
			lit_segment_count += 1;
		}
		if self.d {
			lit_segment_count += 1;
		}
		if self.e {
			lit_segment_count += 1;
		}
		if self.f {
			lit_segment_count += 1;
		}
		if self.g {
			lit_segment_count += 1;
		}

		lit_segment_count
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
	fn decode (&mut self) {

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



