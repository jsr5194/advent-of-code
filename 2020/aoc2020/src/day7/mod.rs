use std::collections::HashMap;
use regex::Regex;
use std::fs;

fn read_input() -> HashMap<String, Vec<Contents>> {
	let filename = "./src/day7/input.txt";
	let input_str = fs::read_to_string(filename).expect("[!] ERROR: Could not read file");

	let mut bag_collection: HashMap<String, Vec<Contents>> = HashMap::new();

	let bag_pattern: Regex = Regex::new(r"^\w+? \w+? bags contain( (\d \w+? \w*?|no other) bags?[,\.])+?$").unwrap();

	for bag_line in input_str.lines() {
		// verify format
		assert!(bag_pattern.is_match(bag_line));

		// extract necessary pieces
		let bag_line_split: Vec<String> = bag_line.split(" bags contain ").map(|x| String::from(x)).collect();

		let mut contents: Vec<Contents> = vec![];
		if !bag_line_split[1].contains("no other bags") {
			for cur_element in bag_line_split[1].split(", ") {
				let cur_element_split: Vec<String> = cur_element.split(" ").map(|x| String::from(x)).collect();
				let count = cur_element_split[0].parse::<u32>().expect("could not convert to int");
				let bag_type = format!("{} {}", cur_element_split[1].clone(), cur_element_split[2].clone());
				contents.push(Contents{count:count, bag_type:bag_type});
			}
		}
		bag_collection.insert(bag_line_split[0].clone(), contents);
	}
	bag_collection
}

pub fn run_part1() {
//	let bag_collection = read_input();
//	let target_bag = String::from("shiny gold");
//
//	let mut count = 0;
//	for key in bag_collection.keys(){
//		for idx in 0..bag_collection[key].len(){
//			if contains_target_bag(&target_bag, bag_collection[key][idx].clone(), bag_collection.clone()) {
//				count += 1;
//				break;
//			}
//		}
//	}
//
	println!("Day 7 Part 1 Result: Uncomment if you really want to run it");
}

pub fn run_part2() {
	let bag_collection = read_input();
	let target_bag = bag_collection.get("shiny gold").expect("Could not find bag in collection");
	
	let mut num_bags = 0;
	for content in target_bag {
		num_bags += get_total_bag_count(content.clone(), bag_collection.clone());
	}

	println!("Day 7 Part 2 Result: {}", num_bags);
}

fn contains_target_bag(target_bag: &String, content: Contents, bag_collection: HashMap<String, Vec<Contents>>) -> bool {
	if content.bag_type == *target_bag {
		return true;
	} 

	for sub_collection in bag_collection.get(&content.bag_type).expect("Could not find bag type") {
		if contains_target_bag(target_bag, sub_collection.clone(), bag_collection.clone()) {
			return true;
		}
	}
	false
}

fn get_total_bag_count(bag: Contents, bag_collection: HashMap<String, Vec<Contents>>) -> u32 {
	let mut num_bags = 0;
	let bag_contents = bag_collection.get(&bag.bag_type).expect("Could not find bag type");
	
	num_bags += bag.count;

	if bag_contents.len() > 0 {
		for sub_content in bag_contents  {
			num_bags = num_bags + (bag.count * get_total_bag_count(sub_content.clone(), bag_collection.clone()));
		}
	}

	num_bags
}

#[derive(Debug, Default, Clone)]
struct Contents {
	count: u32,
	bag_type: String,
}


