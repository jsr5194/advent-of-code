use std::fs;

fn get_input() {
	let filename = "./src/day20/test_input.txt";
	let contents = fs::read_to_string(filename).unwrap();

}

pub fn run_part1() {
	get_input();
}

pub fn run_part2() {

}

#[derive(Debug, Default, Clone, PartialEq)] 
struct Image {

}
