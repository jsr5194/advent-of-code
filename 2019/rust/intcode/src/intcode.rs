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
		let op1_idx = self.mem[self.ip+1] as usize;
		let op2_idx = self.mem[self.ip+2] as usize;
		let res_idx = self.mem[self.ip+3] as usize;

		self.mem[res_idx] = self.mem[op1_idx] + self.mem[op2_idx];
		self.inc_ip(4);
	}

	// Opcode Multiply
	fn exec_multiply(&mut self) {
		let op1_idx = self.mem[self.ip+1] as usize;
		let op2_idx = self.mem[self.ip+2] as usize;
		let res_idx = self.mem[self.ip+3] as usize;

		self.mem[res_idx] = self.mem[op1_idx] * self.mem[op2_idx];
		self.inc_ip(4);
	}

	// Opcode Exit
	fn exec_exit(&mut self) {
		println!("Opcode: Exit");
		println!("Final Mem State: {:?}", self.mem);
		self.shutdown(0);
	}


	// Opcode Unknown
	fn exec_unknown(&mut self) {
		println!("Opcode: Unknown");
		self.shutdown(1);
	}


	// memory bounds checking
	fn inc_ip(&mut self, value: usize) {
		if self.ip + value < self.mem.len() {
			self.ip += value;
		} else {
			panic!("[!] Invalid instruction pointer value");
		}
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
