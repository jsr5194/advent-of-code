use std::fmt;

// CPU struct
#[derive(Debug)]
pub struct IntcodeCpu {
	pub ip: usize,
	pub interrupt: bool,
	pub mem: Vec<u32>,
}

impl fmt::Display for IntcodeCpu {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Instruction Pointer: {}", self.ip)
	}
}

impl IntcodeCpu {
	pub fn run(&mut self, program: String) -> u32 {
		// display the raw program
		println!("Program: {}", program);

		// initialze the instruction pointer to zero
		self.ip = 0;

		// split out the program into parsable elements and convert to a vector
		self.mem = program.split(",").map(|x| x.parse::<u32>().expect("[!] ERROR: could not convert to u32")).collect();

		// loop
		while !self.interrupt {
			// fetch
			let cur_opcode = self.mem[self.ip];

			match Opcode::from(cur_opcode) {
				Opcode::Add => self.exec_add(),
				Opcode::Multiply => self.exec_multiply(),
				Opcode::Exit => self.exec_exit(),
				_ => self.exec_unknown(),
			}
		}

		return 0;
	}

	// Opcode Add
	fn exec_add(&mut self) {
		println!("Opcode: Add, Param: {}, Param: {}, Param: {}", self.mem[self.ip+1], self.mem[self.ip+2], self.mem[self.ip+3]);
		self.ip += 4;
	}

	// Opcode Multiply
	fn exec_multiply(&mut self) {
		println!("Opcode: Multiply, Param: {}, Param: {}, Param: {}", self.mem[self.ip+1], self.mem[self.ip+2], self.mem[self.ip+3]);
		self.ip += 4;
	}

	// Opcode Exit
	fn exec_exit(&mut self) {
		println!("Opcode: Exit");
		self.shutdown(0);
	}


	// Opcode Unknown
	fn exec_unknown(&mut self) {
		println!("Opcode: Unknown");
		self.shutdown(1);
	}

	// system shutdown
	fn shutdown(&mut self, err: u32) {
		println!("[!] Exiting with code {}", err);
		self.interrupt = true;
	}



}




// opcode enum
enum Opcode {
	Bad,
	Add,
	Multiply,
	Exit,
}

impl From<u32> for Opcode {
	fn from(opcode: u32) -> Self {
		match opcode {
			1 => Opcode::Add,
			2 => Opcode::Multiply,
			99 => Opcode::Exit,
			_ => Opcode::Bad
		}
	}
}
