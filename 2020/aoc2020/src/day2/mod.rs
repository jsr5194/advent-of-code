use std::fs;

fn get_input() -> Vec<Password> {
	let filename = "./src/day2/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Vec<String> = contents_str.lines().map(|x| x.parse::<String>().expect("[!] ERROR: could not convert input to String")).collect();

	let mut password_db: Vec<Password> = Vec::new();

	for line in contents {
		let passwd_group: Vec<String> = line.split(": ").map(|x| x.parse::<String>().expect("[!] ERROR: could not convert to line to String")).collect();
		let full_policy: Vec<String> = passwd_group[0].split(" ").map(|x| x.parse::<String>().expect("[!] ERROR: could not convert to line to String")).collect();
		let policy_range: Vec<u32> = full_policy[0].clone().split("-").map(|x| x.parse::<u32>().expect("[!] ERROR: could not convert to line to u32")).collect();

		password_db.push(
			Password {
				passwd: passwd_group[1].clone(),
				policy_char: full_policy[1].chars().next().expect("[!] ERROR: could not convert to char").clone(),
				policy_range_lo: (policy_range[0].clone() - 1) as usize, // need to subtract one as its not zero indexed
				policy_range_hi: (policy_range[1].clone() - 1) as usize, // need to subtract one as its not zero indexed
			}
		);
	}

	password_db
}

pub fn run_part1() {
	let password_db = get_input();

	let mut valid_passwd_cnt = 0;

	for password_group in password_db {
		let mut cnt = 0;
		for letter in password_group.passwd.chars() {
			if letter == password_group.policy_char {
				cnt += 1;
			}
		}

		if cnt >= password_group.policy_range_lo && cnt <= password_group.policy_range_hi {
			valid_passwd_cnt += 1;
		}
	}

	println!("[*] Day 2 Part 1 Result: {}", valid_passwd_cnt);
}

pub fn run_part2 () {
	let password_db = get_input();

	let mut valid_passwd_cnt = 0;

	for password_group in password_db {
		let first_loc = password_group.passwd.as_bytes()[password_group.policy_range_lo] as char;
		let second_loc = password_group.passwd.as_bytes()[password_group.policy_range_hi] as char;

		if (first_loc == password_group.policy_char) ^ (second_loc == password_group.policy_char) {
			valid_passwd_cnt += 1;
		}
	}

	println!("[*] Day 2 Part 2 Result: {}", valid_passwd_cnt);
}

#[derive(Debug, Default)]
struct Password {
	passwd: String,
	policy_char: char,
	policy_range_lo: usize,
	policy_range_hi: usize,
}