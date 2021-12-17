use std::fs;
use hex::decode;

fn get_input() -> Packet {
	let filename = "./src/day16/input_test_1.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Packet = Packet::from(contents_str);
	contents
}

/// run Day 16 Part 1
pub fn run_part_1() {
	let mut pkt = get_input();
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

#[derive(Debug, Default, Clone)]
struct Packet {
	version: u8,
	type_id: TypeId,
	data: Vec<u8>,
}

impl From<String> for Packet {
	fn from(pkt_str: String) -> Self {
		let mut pkt = Packet::default();
		let hdr_vec = hex::decode(&pkt_str[..2]).unwrap();
		pkt.version = (hdr_vec[0] & 0b11100000) >> 0x05;
		pkt.type_id = TypeId::from((hdr_vec[0] & 0b00011100) >> 0x02);
		let mut raw_data = hex::decode(&pkt_str).unwrap();
		let mut first_run = true;
		let mut bit_count = 0x00;
		let mut build_byte = 0x00;
		for cur_byte in raw_data {
			if !first_run {
				let mut mask = 0b10000000;
				for shift in (0..8).rev() {
					build_byte = build_byte << 1;
					build_byte = build_byte | ((cur_byte & mask) >> shift);
					bit_count += 1;
					if bit_count == 0x08 {
						pkt.data.push(build_byte);
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