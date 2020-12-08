
#[derive(Debug, Default, Clone)]
pub struct Handheld {
	pub ip: usize,
	pub r0: u32,
	pub mem: Vec<Instruction>,
	pub halt: bool,
	pub int: bool,
	pub error: bool,
}

impl Handheld {
	pub fn reset(&mut self) {
		*self = Handheld::default();
	}

	pub fn run(&mut self, program: Vec<Instruction>) {
		for instruction in program {
			self.mem.push(instruction);
		}

		while !self.int {
			if self.mem[self.ip].get_executed() {
				self.interrupt(ReturnCode::Error);
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
					Instruction::NOP{executed: _, value: _} => {
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
		if new_ip >= self.mem.len() {
			self.interrupt(ReturnCode::Success);
		} else {
			self.ip = new_ip;
		}
	}

	fn set_error(&mut self, retcode: ReturnCode) {
		match retcode {
			ReturnCode::Success => self.error = false,
			                  _ => self.error = true,
		}
	}

	fn interrupt(&mut self, retcode: ReturnCode) {
		self.set_error(retcode);
		self.int = true;
	}

//	fn exit(&mut self, retcode: ReturnCode) {
//		self.set_error(retcode);
//		self.halt = true;
//	}
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
	ACC {executed:bool, value:i32},
	JMP {executed:bool, value:i32},
	NOP {executed:bool, value:i32},
}

impl From<String> for Instruction {
	fn from(instruction: String) -> Self {
		let split_instruction: Vec<&str> = instruction.split(" ").map(|x| x).collect();
		let value = split_instruction[1].parse::<i32>().expect("[!] Could not convert to i32"); 
		match split_instruction[0] {
			"acc" => Instruction::ACC{executed:false, value:value},
			"jmp" => Instruction::JMP{executed:false, value:value},
			"nop" => Instruction::NOP{executed:false, value:value},
			_ => panic!("[!] ERROR: invalid instruction detected"),
		}
	}
}

impl Instruction {
	fn get_executed(&self) -> bool {
		match self {
			Instruction::ACC {executed, value: _} => return *executed,
			Instruction::JMP {executed, value: _} => return *executed,
			Instruction::NOP {executed, value: _} => return *executed,
		}
	}

	fn set_executed(&mut self) {
		match self {
			Instruction::ACC {executed, value: _} => *executed = true,
			Instruction::JMP {executed, value: _} => *executed = true,
			Instruction::NOP {executed, value: _} => *executed = true,
		}
	}

	pub fn get_value(&self) -> i32 {
		match self {
			Instruction::ACC {executed: _, value} => return *value,
			Instruction::JMP {executed: _, value} => return *value,
			Instruction::NOP {executed: _, value} => return *value,
		}
	}
}

#[derive(Debug, Clone)]
enum ReturnCode {
	Error,
	Success,
}




