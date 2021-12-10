use std::fmt;
use std::fs;

fn get_input() -> Program {
	let filename = "./src/day10/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Program = Program::from(contents_str);
	contents
}

/// run Day 10 Part 1
pub fn run_part_1() {
	let mut prog: Program = get_input();
	println!("[*] Day 10 Part 1 Result: {}", prog.check_syntax());
}

/// run Day 10 Part 2
pub fn run_part_2() {
	let mut prog: Program = get_input();
	prog.check_syntax();
	prog.autocomplete();

	let mut scores: Vec<u64> = vec![];
	for line in prog.lines {
		if !line.is_corrupt {
			scores.push(line.autocomplete_score);
		}
	}
	scores.sort();
	println!("[*] Day 10 Part 2 Result: {}", scores[scores.len()/2]);
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Instruction {
	symbol: char,
	is_corrupt: bool,
}

impl Instruction {
	fn is_open(&self) -> bool {
		match self.symbol {
			'(' => true,
			'[' => true,
			'{' => true,
			'<' => true,
			_ => false,
		}
	}

	fn is_close(&self) -> bool {
		match self.symbol {
			')' => true,
			']' => true,
			'}' => true,
			'>' => true,
			_ => false,
		}
	}

	fn closes(&self, opener: &Instruction) -> bool {
		let mut ret: bool = false;
		match opener.symbol {
			'(' => {
				if self.symbol == ')' {
					ret = true;
				}
			},
			'[' => {
				if self.symbol == ']' {
					ret = true;
				}
			},
			'{' => {
				if self.symbol == '}' {
					ret = true;
				}
			},
			'<' => {
				if self.symbol == '>' {
					ret = true;
				}
			},
			_ => ret = false,
		}
		ret
	}
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Line {
	contents: Vec<Instruction>,
	is_corrupt: bool,
	autocomplete_score: u64,
}

impl Line {
	fn check_syntax(&mut self) -> u64 {
		let mut score: u64 = 0;
		let mut instr_stack: Vec<Instruction> = vec![];
		for instr in &mut self.contents {
			if instr.is_open() {
				instr_stack.push(instr.clone());
			} else if instr.is_close() {
				if !instr.closes(&instr_stack[instr_stack.len()-1]) {
					instr.is_corrupt = true;
					match instr.symbol {
					    ')' => score += 3,
					    ']' => score += 57,
					    '}' => score += 1197,
					    '>' => score += 25137,
					    _ => panic!("invalid instruction"),
					}
					self.is_corrupt = true;
					break;
				} else {
					let i = instr_stack.pop();
				}
			} else {
				panic!("instruction is somehow not open or close: {:?}", instr);
			}
		}
		score
	}
	fn autocomplete(&mut self) -> u64 {
		let mut score: u64 = 0;
		if !self.is_corrupt {
			let contents_len = self.contents.len();
			for idx in (0..contents_len).rev() {
				if self.contents[idx].is_open() {
					match self.contents[idx].symbol {
						'(' => {
							let mut is_closed = false;
							let mut open_count = 0;
							for j in idx+1..contents_len {
								if self.contents[j].symbol == '(' {
									open_count += 1;
								} else if self.contents[j].symbol == ')' {
									open_count -= 1;
									if open_count < 0 {
										is_closed = true;
										break;
									}
								}
							}
							if !is_closed {
								self.contents.push(Instruction{symbol: ')', is_corrupt: false});
								score *= 5;
								score += 1;
							}
						},
						'[' => {
							let mut is_closed = false;
							let mut open_count = 0;
							for j in idx+1..contents_len {
								if self.contents[j].symbol == '[' {
									open_count += 1;
								} else if self.contents[j].symbol == ']' {
									open_count -= 1;
									if open_count < 0 {
										is_closed = true;
										break;
									}
								}
							}
							if !is_closed {
								self.contents.push(Instruction{symbol: ']', is_corrupt: false});
								score *= 5;
								score += 2;
							}
						},
						'{' => {
							let mut is_closed = false;
							let mut open_count = 0;
							for j in idx+1..contents_len {
								if self.contents[j].symbol == '{' {
									open_count += 1;
								} else if self.contents[j].symbol == '}' {
									open_count -= 1;
									if open_count < 0 {
										is_closed = true;
										break;
									}
								}
							}
							if !is_closed {
								self.contents.push(Instruction{symbol: '}', is_corrupt: false});
								score *= 5;
								score += 3;
							}
						},
						'<' => {
							let mut is_closed = false;
							let mut open_count = 0;
							for j in idx+1..contents_len {
								if self.contents[j].symbol == '<' {
									open_count += 1;
								} else if self.contents[j].symbol == '>' {
									open_count -= 1;
									if open_count < 0 {
										is_closed = true;
										break;
									}
								}
							}
							if !is_closed {
								self.contents.push(Instruction{symbol: '>', is_corrupt: false});
								score *= 5;
								score += 4;
							}
						},
						_ => panic!("invalid symbol"),
					}
				}
			}
		}
		score
	}
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Program {
	lines: Vec<Line>,
}

impl Program {
	fn check_syntax(&mut self) -> u64 {
		let mut score: u64 = 0;
		for line in &mut self.lines {
			score += line.check_syntax();
		}
		score
	}

	fn autocomplete(&mut self) {
		for line in &mut self.lines {
			line.autocomplete_score = line.autocomplete();
		}
	}
}

impl From<String> for Program {
	fn from(prog_str: String) -> Self {
		let mut prog = Program::default();
		for cur_line_str in prog_str.lines() {
			let mut cur_line = Line::default();
			for instr_str in cur_line_str.chars() {
				cur_line.contents.push(Instruction{symbol: instr_str, is_corrupt: false});
			}
			prog.lines.push(cur_line);
		}
		prog
	}
}

impl fmt::Display for Program {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut prog_str = String::default();
		for line in &self.lines {
			if !line.is_corrupt {
				for instr in &line.contents {
					prog_str += &instr.symbol.to_string();
				}
				prog_str += "\n";
			}
		}

		write!(f, "{}", prog_str)
	}
}