use std::fs;

fn read_file(filename: &str) -> Vec<i32> {
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Vec<i32> = contents_str.lines().map(|x| x.parse::<i32>().expect("[!] ERROR: could not convert to i32")).collect();
	return contents;
}

fn main() {
	// get the file contents
	let contents:Vec<i32> = read_file("./src/input.txt");

	// 
	let mut result = 0;
	for i in 0..contents.len() {
		for j in 1..contents.len() {
			if contents[i] + contents[j] == 2020 {
				result = contents[i] * contents[j];
				break;
			}
		}
	}

	println!("[*] Result: {}", result);
}


