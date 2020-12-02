use std::fs;

fn get_input() -> Vec<String> {
	let filename = "./src/day2/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents = contents_str.lines().map(|x| x.parse::<String>().expect("[!] ERROR: could not convert input to String")).collect();
	contents
}

pub fn run_part1() {
	let contents = get_input();

	let mut valid_passwd_cnt = 0;

	for line in contents {
		let passwd_group: Vec<String> = line.split(": ").map(|x| x.parse::<String>().expect("[!] ERROR: could not convert to line to String")).collect();
		let full_policy: Vec<String> = passwd_group[0].split(" ").map(|x| x.parse::<String>().expect("[!] ERROR: could not convert to line to String")).collect();
		let policy_amount: Vec<u32> = full_policy[0].clone().split("-").map(|x| x.parse::<u32>().expect("[!] ERROR: could not convert to line to u32")).collect();
		let policy_amount_low:u32 = policy_amount[0].clone();
		let policy_amount_high:u32 = policy_amount[1].clone();
		let policy_char = full_policy[1].clone();
		let passwd = passwd_group[1].clone();

		let mut cnt = 0;
		for letter in passwd.chars() {
			if letter == policy_char.chars().next().expect("[!] ERROR: could not get next byte") {
				cnt += 1;
			}
		}

		if cnt >= policy_amount_low && cnt <= policy_amount_high {
			valid_passwd_cnt += 1;
		}
	}

	println!("[*] Day 2 Part 1 Result: {}", valid_passwd_cnt);
}

pub fn run_part2 () {
	let contents = get_input();

	let mut valid_passwd_cnt = 0;

	for line in contents {
		let passwd_group: Vec<String> = line.split(": ").map(|x| x.parse::<String>().expect("[!] ERROR: could not convert to line to String")).collect();
		let full_policy: Vec<String> = passwd_group[0].split(" ").map(|x| x.parse::<String>().expect("[!] ERROR: could not convert to line to String")).collect();
		let policy_locations: Vec<u32> = full_policy[0].clone().split("-").map(|x| x.parse::<u32>().expect("[!] ERROR: could not convert to line to u32")).collect();
		let policy_location_low:u32 = policy_locations[0].clone();
		let policy_location_high:u32 = policy_locations[1].clone();
		let policy_char = full_policy[1].clone();
		let passwd = passwd_group[1].chars().clone();

		let mut idx = 1;
		let mut first = false;
		let mut second = false;
		for letter in passwd {
			if idx == policy_location_low {
				if letter == policy_char.chars().next().expect("[!] ERROR: could not get next character") {
					first = true;
				}
			} else if idx == policy_location_high {
				if letter == policy_char.chars().next().expect("[!] ERROR: could not get next character") {
					second = true;
				}
			}
			idx += 1;
		}

		if first ^ second {
			valid_passwd_cnt += 1;
		}
	}

	println!("[*] Day 2 Part 2 Result: {}", valid_passwd_cnt);
}