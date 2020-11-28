use std::fmt;

// CPU struct
#[derive(Debug, Default)]
pub struct IntcodeCpu {
	pub ip: usize,
	pub interrupt: bool,
	pub halt: bool,
	pub mem: Vec<i32>,
	pub r0: i32,
	pub r1: i32,
	pub r2: i32,
	pub input: i32,
	pub output: i32,
	pub input_given: bool,
}

impl fmt::Display for IntcodeCpu {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "CPU:\n\tInstruction Pointer: {}\n\tr0: {}\n\tr1: {}\n\tr2: {}\n\tinterrupt: {}\n\thalt: {}\n\tMemory: {:?}", self.ip, self.interrupt, self.halt, self.r0, self.r1, self.r2, self.mem)
	}
}

impl IntcodeCpu {
	pub fn load(&mut self, program: String) {
		self.ip = 0;
 		self.interrupt = false;
 		self.halt = false;
 		self.mem = Vec::new();
 		self.r0 = 0;
 		self.r1 = 0;
 		self.r2 = 0;
 		self.input = 0;
 		self.output = 0;

		// split out the program into parsable elements and convert to a vector
		self.mem = program.split(",").map(|x| x.parse::<i32>().expect("[!] ERROR: could not convert to i32")).collect();
	}

	pub fn input(&mut self, value: i32) {
		self.input_given = true;
		self.input = value;
	}

	pub fn run(&mut self) {
		self.interrupt = false;

		// loop
		while !self.interrupt {
			// fetch
			let cur_instruction = self.mem[self.ip];

			// decode
			// ABCDD
			// A  == mode of third parameter (read into r2)
			// B  == mode of second parameter (read into r1)
			// C  == mode of first parameter (read into r0)
			// DD == opcode (read into local variable)

			self.r0 = (cur_instruction % 1000) / 100;
			self.r1 = (cur_instruction % 10000) / 1000;
			self.r2 = cur_instruction / 10000;
			let cur_opcode = cur_instruction % 100;

			// execute
			match Opcode::from(cur_opcode) {
				Opcode::Add => self.exec_add(),
				Opcode::Multiply => self.exec_multiply(),
				Opcode::Input => self.exec_input(),
				Opcode::Output => self.exec_output(),
				Opcode::JumpIfTrue => self.exec_jump_if_true(),
				Opcode::JumpIfFalse => self.exec_jump_if_false(),
				Opcode::LessThan => self.exec_less_than(),
				Opcode::Equals => self.exec_equals(),
				Opcode::Exit => self.exec_exit(),
				_ => self.exec_unknown(),
			}
		}
	}

	// resolve argument
	// at this point registers hold the following values:
	//  * r0 == mode of argument 0
	//  * r1 == mode of argument 1
	//  * r2 == mode of argument 2
	fn resolve_arg(&mut self, arg_num: i32) -> i32 {
		return self.priv_resolve_arg(arg_num, false);
	}
	fn resolve_write_arg(&mut self, arg_num: i32) -> i32 {
		// Parameters that an instruction writes to will never be in immediate mode
		match arg_num {
			0 => {
				if self.r0 != 0 {
					panic!("[!] ERROR: instruction write parameter set to position mode");
				}
			}
			1 => {
				if self.r1 != 0 {
					panic!("[!] ERROR: instruction write parameter set to position mode");
				}
			}
			2 => {
				if self.r2 != 0 {
					panic!("[!] ERROR: instruction write parameter set to position mode");
				}
			}
			_ => {
				panic!("[!] Invalid argument selection");
			}
		}
		return self.priv_resolve_arg(arg_num, true);
	}
	// lower level function that actually does the argument resolving
	// this function is not meant to be called directly as wrapper functions can be used
	fn priv_resolve_arg(&mut self, arg_num: i32, is_write_arg: bool) -> i32 {
		let arg: i32;

		match arg_num {
			0 => {
				if self.r0 == 0 && !is_write_arg {
					let arg_idx = self.mem[self.ip+1] as usize;
					arg = self.mem[arg_idx];
				} else {
					arg = self.mem[self.ip+1];
				}
			}
			1 => {
				if self.r1 == 0 && !is_write_arg {
					let arg_idx = self.mem[self.ip+2] as usize;
					arg = self.mem[arg_idx];
				} else {
					arg = self.mem[self.ip+2];
				}
			}
			2 => {
				if self.r2 == 0 && !is_write_arg {
					let arg_idx = self.mem[self.ip+3] as usize;
					arg = self.mem[arg_idx];
				} else {
					arg = self.mem[self.ip+3];
				}
			}
			_ => {
				panic!("[!] Invalid argument selection");
			}
		}

		return arg;
	}

	// Opcode Add
	fn exec_add(&mut self) {
		// write out results
		let wr_index = self.resolve_write_arg(2) as usize;
		self.mem[wr_index] = self.resolve_arg(0) + self.resolve_arg(1);
		self.inc_ip(4);
	}

	// Opcode Multiply
	fn exec_multiply(&mut self) {
		// write out results
		let wr_index = self.resolve_write_arg(2) as usize;
		self.mem[wr_index] = self.resolve_arg(0) * self.resolve_arg(1);
		self.inc_ip(4);
	}

	// Opcode Input
	fn exec_input(&mut self) {
		if !self.input_given {
			self.interrupt = true;
		} else {
			let wr_index = self.resolve_write_arg(0) as usize;
			self.mem[wr_index] = self.input;
			self.inc_ip(2);
		}
	}

	// Opcode Output
	fn exec_output(&mut self) {
		self.output = self.resolve_arg(0);
		self.inc_ip(2);
		self.interrupt = true;
	}

	// Opcode Jump If True
	fn exec_jump_if_true(&mut self) {
		if self.resolve_arg(0) != 0 {
			let arg1 = self.resolve_arg(1) as usize;
			self.set_ip(arg1);
		} else {
			self.inc_ip(3);
		}
	}

	// Opcode Jump If False
	fn exec_jump_if_false(&mut self) {
		if self.resolve_arg(0) == 0 {
			let arg1 = self.resolve_arg(1) as usize;
			self.set_ip(arg1);
		} else {
			self.inc_ip(3);
		}
	}

	// opcode Less Than
	fn exec_less_than(&mut self) {
		let wr_index = self.resolve_write_arg(2) as usize;
		if self.resolve_arg(0) < self.resolve_arg(1) {
			self.mem[wr_index] = 1;
		} else {
			self.mem[wr_index] = 0;
		}
		self.inc_ip(4);
	}

	// opcode Equals
	fn exec_equals(&mut self) {
		let wr_index = self.resolve_write_arg(2) as usize;
		if self.resolve_arg(0) == self.resolve_arg(1) {
			self.mem[wr_index] = 1;
		} else {
			self.mem[wr_index] = 0;
		}
		self.inc_ip(4);
	}

	// Opcode Exit
	fn exec_exit(&mut self) {
		self.interrupt = true;
		self.halt = true;
	}

	// Opcode Unknown
	fn exec_unknown(&mut self) {
		let msg = format!("Opcode: Unknown\n{}", self);
		panic!(msg);
	}






	// memory bounds checking
	fn inc_ip(&mut self, offset: usize) {
		let new_ip = self.ip + offset;
		if new_ip < self.mem.len() {
			self.ip += offset;
		} else {
			panic!("[!] Invalid instruction pointer offset");
		}
	}
	fn set_ip(&mut self, value: usize) {
		if value < self.mem.len() {
			self.ip = value;
		} else {
			panic!("[!] Invalid instruction pointer value");
		}
	}
}




// opcode enum
enum Opcode {
	Bad,
	Add,
	Multiply,
	Input,
	Output,
	JumpIfTrue,
	JumpIfFalse,
	LessThan,
	Equals,
	Exit,
}

impl From<i32> for Opcode {
	fn from(opcode: i32) -> Self {
		match opcode {
			1 => Opcode::Add,
			2 => Opcode::Multiply,
			3 => Opcode::Input,
			4 => Opcode::Output,
			5 => Opcode::JumpIfTrue,
			6 => Opcode::JumpIfFalse,
			7 => Opcode::LessThan,
			8 => Opcode::Equals,
			99 => Opcode::Exit,
			_ => Opcode::Bad
		}
	}
}


// parameter enum
enum Parameter {
	Bad,
	Position,
	Immediate,
}

impl From<i32> for Parameter {
	fn from(param: i32) -> Self {
		match param {
			0 => Parameter::Position,
			1 => Parameter::Immediate,
			_ => Parameter::Bad,
		}
	}
}
