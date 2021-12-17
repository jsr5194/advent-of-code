use std::fs;
use hex::decode;

fn get_input() -> Packet {
	let filename = "./src/day16/input_test_3.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Packet = Packet::from(contents_str);
	contents
}

/// run Day 16 Part 1
pub fn run_part_1() {
	let mut pkt = get_input();
	pkt.parse();
	println!("{:?}", pkt);
	println!("[*] Day 16 Part 1 Result: TODO");
}

/// run Day 16 Part 2
pub fn run_part_2() {
	println!("[*] Day 16 Part 2 Result: TODO");
}

#[derive(Debug, Clone)] 
enum TypeId {
	NOTSET,
	LITERALVALUE,
	OPERATOR,
}

impl From<u8> for TypeId {
	fn from(type_id: u8) -> Self {
		match type_id {
			0x04 => Self::LITERALVALUE,
			_ => Self::OPERATOR,
		}
	}
}

impl Default for TypeId {
	fn default() -> Self {
		TypeId::NOTSET
	}
}


#[derive(Debug, Clone)]
enum LengthTypeId {
	BITS,
	PACKETS,
	NOTSET,
}

impl From<u8> for LengthTypeId {
	fn from(length_type_id: u8) -> Self {
		match length_type_id {
			0x00 => Self::BITS,
			0x01 => Self::PACKETS,
			_ => panic!("Invalid Length type id"),
		}
	}
}

impl Default for LengthTypeId {
	fn default() -> Self {
		TypeId::NOTSET
	}
}



#[derive(Debug, Default, Clone)]
struct Packet {
	version: u8,
	type_id: TypeId,
	raw_data: Vec<u8>,
	sub_packets: Vec<Packet>,
}

impl Packet {
	fn parse(&mut self) {
		match self.type_id {
			TypeId::LITERALVALUE => self.parse_as_literal(),
			TypeId::OPERATOR => self.parse_as_operator(),
			_ => panic!("unimplemented type id encountered: {:?}", self.type_id),
		}
	}

	fn parse_as_literal(&mut self) {
		let mut bit_count = 0x00;
		let mut finished = false;
		let mut literal: u32 = 0x00;
		for byte in &self.raw_data {
			let mut mask = 0b10000000;
			for bit_idx in (0..8).rev() {
				// sequence is 1 bit for state and 4 bits for data
				if bit_count % 0x05 == 0 {
					// 1 means more data; 0 means last data
					if byte & mask == 0 {
						finished = true;
					}
				} else {
					literal = literal << 1;
					literal = literal | (((byte & mask) >> bit_idx) as u32);
				}

				mask = mask >> 1;
				bit_count += 1;
			}

			if finished {
				break;
			}
		}
		println!("literal: {:?}", literal);
	}

	fn parse_as_operator(&mut self) {
		let mut bit_count = 0x00;
		let mut build_byte: u16 = 0x00;
		let mut length_type_id = LengthTypeId::default();
		let mut subpacket_bit_count_found = false;
		let mut subpacket_bit_count = 0x00;
		for byte in &self.raw_data {
			let mut mask = 0b10000000;
			for bit_idx in (0..8).rev() {
				if bit_count == 0 {
					length_type_id = LengthTypeId::from((byte & mask) >> bit_idx);
				} else {
					match length_type_id {
						LengthTypeId::BITS => {
							build_byte = build_byte << 1;
							build_byte = build_byte | (((byte & mask) >> bit_idx) as u16);
							if bit_count == 15 {
								subpacket_bit_count = build_byte;
								subpacket_bit_count_found = true;
							} else if subpacket_bit_count_found {
								// 
							}
						},
						LengthTypeId::PACKETS => {
							build_byte = build_byte << 1;
							build_byte = build_byte | (((byte & mask) >> bit_idx) as u16);
							if bit_count == 11 {
								self.subpacket_count = build_byte;
								finished = true;
								break;
							}
						},
						_ => panic!("invalid length type id: {:?}", length_type_id),
					}
				}

				mask = mask >> 1;
				bit_count += 1;
			}

			if finished {
				break;
			}
		}
	}
}

impl From<String> for Packet {
	fn from(pkt_str: String) -> Self {
		let mut pkt = Packet::default();
		let hdr_vec = hex::decode(&pkt_str[..2]).unwrap();
		pkt.version = (hdr_vec[0] & 0b11100000) >> 0x05;
		pkt.type_id = TypeId::from((hdr_vec[0] & 0b00011100) >> 0x02);
		let mut first_run = true;
		let mut bit_count = 0x00;
		let mut build_byte = 0x00;
		for cur_byte in hex::decode(&pkt_str).unwrap() {
			if !first_run {
				let mut mask = 0b10000000;
				for shift in (0..8).rev() {
					build_byte = build_byte << 1;
					build_byte = build_byte | ((cur_byte & mask) >> shift);
					bit_count += 1;
					if bit_count == 0x08 {
						pkt.raw_data.push(build_byte);
						bit_count = 0x00;
						build_byte = 0x00;
					}
					mask = mask >> 1;
				}
			} else {
				first_run = false;
				build_byte = cur_byte & 0b00000011;
				bit_count += 2;
			}
		}
		pkt
	}
}