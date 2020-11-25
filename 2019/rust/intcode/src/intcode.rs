use std::fmt;

// CPU struct
#[derive(Debug)]
pub struct IntcodeCpu {
	pub ip: usize,
	pub interrupt: bool,
}

impl fmt::Display for IntcodeCpu {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Instruction Pointer: {}", self.ip)
	}
}

impl IntcodeCpu {
	pub fn run(&mut self, raw_prog: String) -> u32 {
		// display the raw program
		println!("Program: {}", raw_prog);

		// initialze the instruction pointer to zero
		self.ip = 0;

		// split out the program into parsable elements and convert to a vector
		let prog: Vec<&str> = raw_prog.split(",").collect();

		// loop
		while !self.interrupt {
			// fetch
			let cur_opcode = prog[self.ip].parse::<u32>().expect("[!] ERROR: Could not parse next opcode");

			match Opcode::from(cur_opcode) {
				Opcode::Add => println!("Opcode: Add"),
				Opcode::Multiply => println!("Opcode: Multiply"),
				Opcode::Exit => println!("Opcode: Exit"),
				_ => println!("Opcode: Unknown")
			}
		}

		return 0;
	}


	fn exit(&self) {
		println!("Exiting...\n TODO: still need to do cleanup code");
	}
}




// opcode enum
enum Opcode {
	Bad = 0,
	Add = 1,
	Multiply = 2,
	Exit = 99,
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
