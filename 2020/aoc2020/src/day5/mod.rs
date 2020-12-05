use regex::Regex;
use std::fs;

fn get_input() -> Vec<BoardingPass> {
	let filename = "./src/day5/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: could not read file");

	let boarding_pass_pattern: Regex = Regex::new(r"^[FB]{7}[LR]{3}$").unwrap();

	let mut boarding_passes: Vec<BoardingPass> = vec![];
	for line in contents_str.lines() {
		assert!(boarding_pass_pattern.is_match(line));
		boarding_passes.push(BoardingPass{seat_specifier: line.to_string()});
	}

	boarding_passes
}

fn find_seat_number(seat_specifier: String) -> u32 {
	let mut row_start: u32 = 0;
	let mut row_end: u32 = 127;
	let mut col_start: u32 = 0;
	let mut col_end: u32 = 7;
	for instr in seat_specifier.chars() {
		match instr {
			'F' => row_end   -= (row_end - row_start) / 2 + 1,
			'B' => row_start += (row_end - row_start) / 2 + 1,
			'L' => col_end   -= (col_end - col_start) / 2 + 1,
			'R' => col_start += (col_end - col_start) / 2 + 1,
			_ => panic!("[!] Invalid seat specifier instruction encountered: {}", instr),
		}
	}

	// if the start and end values don't match each other something is wrong
	if row_start != row_end || col_start != col_end {
		panic!("[!] Invalid row and column results:\n\tTicket: {}\n\tRow: {}-{}\n\tColumn: {}-{}", seat_specifier, row_start, row_end, col_start, col_end);
	}

	// Every seat also has a unique seat ID: 
	// multiply the row by 8, then add the column
	let seat_number = (row_start * 8) + col_start;
	seat_number as u32
}

pub fn run_part1() {
	let mut largest_seat_number = 0;
	let boarding_passes = get_input();
	for bp in boarding_passes {
		let cur_seat_number = find_seat_number(bp.seat_specifier);
		if cur_seat_number > largest_seat_number {
			largest_seat_number = cur_seat_number;
		}
	}

	println!("Day 5 Part 1 Result: {}", largest_seat_number);
}

pub fn run_part2() {
	let mut known_seats: Vec<u32> = vec![];
	let boarding_passes = get_input();
	
	// calculate every seat number
	for bp in boarding_passes {
		known_seats.push(find_seat_number(bp.seat_specifier));
	}

	// iterate through the possible seat numbers looking for one that doesnt exist
	let mut missing_seat = 0;
	let low_seat = 0*8+0+1;
	let hi_seat = 127*8+7;
	for idx in low_seat..hi_seat {
		if !known_seats.contains(&idx) {
			if known_seats.contains(&(idx-1)) && known_seats.contains(&(idx+1)){
				missing_seat = idx;
			}
		}
	}

	println!("Day 5 Part 2 Result: {}", missing_seat);
}



#[derive(Debug, Default)]
struct BoardingPass {
	seat_specifier: String,
}