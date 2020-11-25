use std::fmt;

// CPU struct
#[derive(Debug)]
pub struct IntcodeCpu {
	pub ip: usize,
	interrupt: bool,
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
		// TODO: figure out why this works
		let prog: Vec<&str> = raw_prog.split(",").collect();

		// loop
		while !self.interrupt {
			let opcode = self.fetch(prog);

		}

		return 0;
	}

	fn fetch(&self, prog: Vec<&str>) -> u32 {
		let opcode = prog[self.ip];

		match opcode {
			opcode if opcode == Opcode::Add as u32 => println!("Opcode: Add"),
			opcode if opcode == Opcode::Mul as u32 => println!("Opcode: Multiply"),
			opcode if opcode == Opcode::End as u32 => println!("Opcode: Exit"),
			_ => println!("Opcode: Unknown")
		}




		println!("{:?}", prog[self.ip]);
		return 0;
	}


	fn exit(&self) {
		println!("Exiting...\n TODO: still need to do cleanup code");
	}
}




// opcode enum
enum Opcode {
	Add = 1,
	Mul = 2,
	End = 99,
}

