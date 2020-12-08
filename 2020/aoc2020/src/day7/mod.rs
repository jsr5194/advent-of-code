use regex::Regex;
use std::fmt;
use std::fs;

fn read_input() -> Vec<Bag> {
	let filename = "./src/day7/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: Could not read file");
	let mut contents: Vec<String> = contents_str.lines().map(|x| x.parse::<String>().expect("[!] ERROR: could not convert input to String")).collect();
	contents.reverse();

	// do an initial pass of the raw bag data
	let mut bag_list: Vec<Bag> = vec![];
	for line in contents {
		bag_list.push(Bag::parse(line.as_str()));
	}

	// update contents
	for bag_list_idx in 0..bag_list.len() {
		for contents_idx in 0..bag_list[bag_list_idx].contents.len() {
			let cur_key = &bag_list[bag_list_idx].contents[contents_idx].key;
			bag_list[bag_list_idx].contents[contents_idx] = get_bag_by_key(&bag_list, cur_key.as_str());
		}
	}

	bag_list
}

pub fn run_part1() {
	let bag_list = read_input();

	let mut result = 0;
	let key = "shiny gold";
	for bag in bag_list {
		if bag.key != key && bag.contains_key(key) {
			//println!("{}", bag);
			result += 1;
		}
	}

	println!("Day 7 Part 1 Result: {}", result);
}

pub fn run_part2() {

}

#[derive(Debug, Default, Clone)]
struct Bag {
	key: String,
	color: String,
	attribute: String,
	contents: Vec<Bag>,
}

impl fmt::Display for Bag {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut writeline = String::default();
		writeline += &self.attribute;
		writeline += " ";
		writeline += &self.color;
		writeline += " bag contains:";
		for idx in 0..self.contents.len() {
			writeline += "\n\t";
			writeline += &self.contents[idx].attribute;
			writeline += " ";
			writeline += &self.contents[idx].color;
			writeline += " bag";
		}

		write!(f, "{}", writeline)
	}
}

impl Bag {
	fn parse(bag_line: &str) -> Bag {
		// verify format
		let bag_pattern: Regex = Regex::new(r"^\w+? \w+? bags contain( (\d \w+? \w*?|no other) bags?[,\.])+?$").unwrap();
		assert!(bag_pattern.is_match(bag_line));

		// extract necessary pieces
		let bag_line_split: Vec<String> = bag_line.split(" contain ").map(|x| String::from(x)).collect();
		let new_bag_details: Vec<String> = bag_line_split[0].split(" ").map(|x| String::from(x)).collect();
		let raw_contents: Vec<String> =  bag_line_split[1].split(", ").map(|x| String::from(x)).collect();

		// create the new bag
		let mut new_bag = Bag::default();
		new_bag.attribute = new_bag_details[0].clone();
		new_bag.color = new_bag_details[1].clone();
		new_bag.key = format!("{} {}", new_bag.attribute, new_bag.color);
		for line in raw_contents {
			if !line.contains("no other bags") {
				let contents_split: Vec<String> = line.split(" ").map(|x| String::from(x)).collect();
				let contents_bag_count = contents_split[0].parse::<u32>().expect("[!] Could not convert bag count to string");
				for _ in 0..contents_bag_count {
					let mut contents_bag = Bag::default();
					contents_bag.attribute = contents_split[1].clone();
					contents_bag.color = contents_split[2].clone();
					contents_bag.key = format!("{} {}", contents_bag.attribute, contents_bag.color);
					new_bag.contents.push(contents_bag.clone());
				}
			}
		}

		new_bag
	}

	fn contains_key(&self, key: &str) -> bool {
		println!("wtf");
		if self.key == key {
			return true;
		} else {
			for bag in &self.contents {
				if bag.contains_key(key) {
					return true;
				}
			}
		}
		false
	}
}

fn get_bag_by_key(bag_list: &Vec<Bag>, key: &str) -> Bag {
	for bag in bag_list {
		if bag.key == key {
			return bag.clone();
		}
	}
	panic!("[!] Could not find the bag");
}



