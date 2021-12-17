use hex::decode;


// analyzer struct
#[derive(Debug, Clone)]
pub struct IntShark {
	halt: bool,
	cursor: usize,
	raw: Vec<u8>,
	packets: Vec<Packet>,
	minimum_bits: usize,
}

impl IntShark {
	fn get_total_bits(&self) -> usize {
		self.raw.len() * 8
	}

	fn inc_cursor(&mut self) {
		self.set_cursor(self.cursor + 1)
	}

	fn set_cursor(&mut self, pos: usize) {
		if pos >= (self.raw.len() * 8) {
			self.cursor = (self.raw.len() * 8) - 1;
			self.halt = true;
		} else {
			self.cursor = pos
		}
	}

	fn read_n_bits(&mut self, n: usize) -> usize {
		let raw_byte_idx = self.cursor / 8;
		let mut mask = 0b10000000 >> (self.cursor % 8);
		let mut result: usize = 0x00;
		let mut bit_count = 0;
		for bit_idx in (0..(8-(self.cursor % 8))).rev() {
			result = result << 1;
			result = result | (((self.raw[raw_byte_idx] & mask) >> bit_idx) as usize);
			mask = mask >> 1;
			bit_count += 1;
			if bit_count == n {
				break;
			}
		}
		self.set_cursor(self.cursor+n);
		result
	}

	pub fn load(&mut self, capture: String) {
		self.cursor = 0;
		self.raw = hex::decode(&capture).unwrap();
	}

	pub fn dissect(&mut self) {
		let mut pkt = Packet::default();
		loop {
			if self.halt {
				break;
			}

			if pkt == Packet::default() {
				let mut hdr = PacketHeader::default();
				hdr.version = self.read_n_bits(3) as u8;
				let tmp_type_id = self.read_n_bits(3);
				hdr.type_id = TypeId::from(tmp_type_id as u8);
				match hdr.type_id {
					TypeId::LITERAL => {
						let mut finished = false;
						let mut data: Vec<u8> = vec![];
						let mut build_byte: u8 = 0x00;
						let mut build_bit = 0x00;
						loop {
							// 1 means more data; 0 means no more data
							let more_data = self.read_n_bits(1);
							if more_data == 0 {
								finished = true;
							}
							build_byte = build_byte << 1;
							build_byte = build_byte | (more_data as u8);
							if build_bit == 8 {
								data.push(build_byte);
								build_bit = 0x00;
								build_byte = 0x00;
							} else {
								build_bit += 1;
							}

							for _ in 0..4 {
								build_byte = build_byte << 1;
								build_byte = build_byte | (self.read_n_bits(1) as u8);
								if build_bit == 7 {
									data.push(build_byte);
									build_bit = 0x00;
									build_byte = 0x00;
								} else {
									build_bit += 1;
								}
							}

							if finished {
								if build_bit < 8 {
									for _ in build_bit..8 {
										build_byte = build_byte << 1;
									}
									data.push(build_byte);
								}
								break;
							}
						}
						pkt = Packet::LITERAL{hdr: hdr.clone(), data: data.clone()};
					},
					TypeId::OPERATOR => {
						//pkt = Packet::OPERATOR{hdr: hdr.clone()};
						panic!("operator not yet implemented");
					},
					_ => panic!("invalid type id"),
				}
			}

			self.packets.push(pkt.clone());

			// check if we've hit the end of the packet
			if self.get_total_bits() - self.cursor < self.minimum_bits {
				self.halt = true;
			}
		}
	}
}

impl Default for IntShark {
	fn default() -> Self {
		IntShark{
			halt: bool::default(),
			cursor: usize::default(),
			raw: vec![],
			packets: vec![],
			minimum_bits: 4,
		}
	}
}


// Packet types
#[derive(Debug, Clone, PartialEq)]
enum Packet {
	LITERAL {hdr: PacketHeader, data: Vec<u8>},
	OPERATOR {hdr: PacketHeader},

	DEFAULT,
}

impl Default for Packet {
	fn default() -> Self {
		Packet::DEFAULT
	}
}








// generic packet header
#[derive(Debug, Default, Clone, PartialEq)]
struct PacketHeader {
	version: u8,
	type_id: TypeId,
}







#[derive(Debug, Clone, PartialEq)] 
enum TypeId {
	LITERAL,
	OPERATOR,

	DEFAULT,
}
impl From<u8> for TypeId {
	fn from(type_id: u8) -> Self {
		match type_id {
			0x04 => Self::LITERAL,
			_ => Self::OPERATOR,
		}
	}
}
impl Default for TypeId {
	fn default() -> Self {
		TypeId::DEFAULT
	}
}

