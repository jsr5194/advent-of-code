
#[derive(Debug, Default, Clone)]
pub struct Handheld {
	pub ip: usize,
	pub r0: u32,
	pub mem: Vec<Instruction>,
	pub halt: bool,
	pub interrupt: bool,
}

impl Handheld {
	pub fn assemble(&mut self, program: Vec<String>) {
		for instruction in program {
			self.mem.push(Instruction::from(instruction));
		}
	}

	pub fn run(&mut self) {
		while !self.interrupt {
			if self.mem[self.ip].get_executed() {
				self.exit()
			} else {
				match self.mem[self.ip] {
					Instruction::ACC{executed: _, value} => {
						self.r0 = self.r0.wrapping_add(value as u32);
						self.mem[self.ip].set_executed();
						self.inc_ip();
					},
					Instruction::JMP{executed: _, value} => {
						self.mem[self.ip].set_executed();
						self.set_ip(self.ip.wrapping_add(value as usize));
					},
					Instruction::NOP{executed: _} => {
						self.mem[self.ip].set_executed();
						self.inc_ip();
					}
				}
			}
		}
	}

	fn inc_ip(&mut self) {
		self.set_ip(self.ip + 1);
	} 

	fn set_ip(&mut self, new_ip:usize) {
		if new_ip > self.mem.len() {
			self.exit();
		} else {
			self.ip = new_ip;
		}
	}

	fn exit(&mut self) {
		self.interrupt = true;
		self.halt = true;
	}
}

#[derive(Debug, Clone)]
pub enum Instruction {
	ACC {executed:bool, value:i32},
	JMP {executed:bool, value:i32},
	NOP {executed:bool},
}

impl From<String> for Instruction {
	fn from(instruction: String) -> Self {
		let split_instruction: Vec<&str> = instruction.split(" ").map(|x| x).collect();
		let value = split_instruction[1].parse::<i32>().expect("[!] Could not convert to i32"); 
		match split_instruction[0] {
			"acc" => Instruction::ACC{executed:false, value:value},
			"jmp" => Instruction::JMP{executed:false, value:value},
			"nop" => Instruction::NOP{executed:false},
			_ => panic!("[!] ERROR: invalid instruction detected"),
		}
	}
}

impl Instruction {
	fn get_executed(&self) -> bool {
		match self {
			Instruction::ACC {executed, value: _} => return *executed,
			Instruction::JMP {executed, value: _} => return *executed,
			Instruction::NOP {executed} => return *executed,
		}
	}

	fn set_executed(&mut self) {
		match self {
			Instruction::ACC {executed, value: _} => *executed = true,
			Instruction::JMP {executed, value: _} => *executed = true,
			Instruction::NOP {executed} => *executed = true,
		}
	}
}




