use std::fs;

fn get_input() -> Vec<Credential> {
	let filename = "./src/day4/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: could not read file");
	
	let mut credentials: Vec<Credential> = vec![Credential::default()];
	for line in contents_str.lines() {
		// add a new credential if an empty line is encountered
		if line == "" {
			credentials.push(Credential::default());
		} else {
			for raw_kvp in line.split_whitespace() {
				let kvp: Vec<String> = raw_kvp.split(":").map(|x| String::from(x)).collect();
				match CredentialField::from(kvp[0].as_str()) {
					CredentialField::Byr => credentials.last_mut().expect("[!] ERROR: could not get last credential").byr = kvp[1].clone().parse::<u32>().expect("[!] Could not convert to int"),
					CredentialField::Iyr => credentials.last_mut().expect("[!] ERROR: could not get last credential").iyr = kvp[1].clone().parse::<u32>().expect("[!] Could not convert to int"),
					CredentialField::Eyr => credentials.last_mut().expect("[!] ERROR: could not get last credential").eyr = kvp[1].clone().parse::<u32>().expect("[!] Could not convert to int"),
					CredentialField::Hgt => credentials.last_mut().expect("[!] ERROR: could not get last credential").hgt = kvp[1].clone(),
					CredentialField::Hcl => credentials.last_mut().expect("[!] ERROR: could not get last credential").hcl = kvp[1].clone(),
					CredentialField::Ecl => credentials.last_mut().expect("[!] ERROR: could not get last credential").ecl = kvp[1].clone(),
					CredentialField::Pid => credentials.last_mut().expect("[!] ERROR: could not get last credential").pid = kvp[1].clone(),
					CredentialField::Cid => credentials.last_mut().expect("[!] ERROR: could not get last credential").cid = kvp[1].clone(),
				}
			}
		}
	}
	credentials
}

pub fn run_part1() {
	// parse the credentials
	let credentials = get_input();

	// iterate over each credential and determine if its valid
	let mut num_valid_credentials = 0;
	for credential in credentials {
		if credential.is_complete_passport() {
			num_valid_credentials += 1;
		}
	}

	println!("Day 4 Part 1 Result: {}", num_valid_credentials);
}

pub fn run_part2() {
	// parse the credentials
	let credentials = get_input();

	// iterate over each credential and determine if its valid
	let mut num_valid_credentials = 0;
	for credential in credentials {
		if credential.is_valid_passport() {
			num_valid_credentials += 1;
		}
	}

	println!("Day 4 Part 2 Result: {}", num_valid_credentials);
}


#[derive(Debug, Default)]
struct Credential {
	byr: u32,    // (Birth Year)
	iyr: u32,    // (Issue Year)
	eyr: u32,    // (Expiration Year)
	hgt: String, // (Height)
	hcl: String, // (Hair Color)
	ecl: String, // (Eye Color)
	pid: String, // (Passport ID)
	cid: String, // (Country ID)
}

impl Credential {
	fn is_complete_passport(&self) -> bool {
		// bail if any of the required fields are empty
		if self.byr == u32::default() {
			false
		} else if self.iyr == u32::default() {
			false
		} else if self.eyr == u32::default() {
			false
		} else if self.hgt == String::default() {
			false
		} else if self.hcl == String::default() {
			false
		} else if self.ecl == String::default() {
			false
		} else if self.pid == String::default() {
			false
		} else {
			true
		}
	}

	fn is_valid_passport(&self) -> bool {
		// ensure the passport is complete
		if !self.is_complete_passport() {
			return false;
		}

		// do field-specific parsing

		// byr (Birth Year) - four digits; at least 1920 and at most 2002.
		if self.byr < 1920 || self.byr > 2002 {
			return false;
		}

		// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
		if self.iyr < 2010 || self.iyr > 2020 {
			return false;
		}

		// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
		if self.eyr < 2020 || self.eyr > 2030 {
			return false;
		}

		// hgt (Height) - a number followed by either cm or in:
		let hgt_unit_idx = self.hgt.len()-2;
		let hgt_unit = &self.hgt[hgt_unit_idx..];
		let hgt_value = self.hgt[..hgt_unit_idx].parse::<u32>().expect("[!] ERROR: could not convert to u32");
		// If cm, the number must be at least 150 and at most 193.
		if hgt_unit == "cm" {
			if hgt_value < 150 || hgt_value > 193 {
				return false;
			}
		// If in, the number must be at least 59 and at most 76.
		} else if hgt_unit == "in" {
			if hgt_value < 59 || hgt_value > 76 {
				return false;
			}
		} else { 
			return false;
		}
		
		// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
		if self.hcl.len() != 7 || &self.hcl[0..1] != "#" {
			return false;
		} else {
			for &byte in self.hcl[1..].as_bytes() {
				if byte < 48 || (byte > 57 && byte < 97) || byte > 122 {
					return false;
				}
			}
		}
		
		// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
		let ecl_list = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
		if !ecl_list.contains(&self.ecl.as_str()){
			return false;
		}
		
		// pid (Passport ID) - a nine-digit number, including leading zeroes.
		if self.pid.len() != 9 || !self.pid.parse::<u64>().is_ok() {
			return false;
		}
		
		// cid (Country ID) - ignored, missing or not.

		true
	}
}


#[derive(Debug)]
enum CredentialField {
	Byr, // (Birth Year)
	Iyr, // (Issue Year)
	Eyr, // (Expiration Year)
	Hgt, // (Height)
	Hcl, // (Hair Color)
	Ecl, // (Eye Color)
	Pid, // (Passport ID)
	Cid, // (Country ID)
}

impl From<&str> for CredentialField {
	fn from(field: &str) -> Self {
		match field {
			"byr" => CredentialField::Byr,
			"iyr" => CredentialField::Iyr,
			"eyr" => CredentialField::Eyr,
			"hgt" => CredentialField::Hgt,
			"hcl" => CredentialField::Hcl,
			"ecl" => CredentialField::Ecl,
			"pid" => CredentialField::Pid,
			"cid" => CredentialField::Cid,
			_ => panic!("[!] ERROR: invalid credential field detected"),
		}
	}
}

