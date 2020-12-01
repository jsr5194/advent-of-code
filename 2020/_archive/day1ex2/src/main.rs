use std::fs;

/// takes a filename, splits on newline, and returns a Vec<u32> of the elements
fn read_file(filename: &str) -> Vec<u32> {
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: Vec<u32> = contents_str.lines().map(|x| x.parse::<u32>().expect("[!] ERROR: an error occurred while parsing the lines")).collect();
	return contents;
}

fn main() {
	// get the file contents
	let contents: Vec<u32> = read_file("./src/input.txt");

	// need three entries that sum to 2020
	let mut result = 0;
	for i in 0..contents.len() {
		for j in 1..contents.len() {
			for k in 2..contents.len() {
				if contents[i] + contents[j] + contents[k] == 2020 {
					result = contents[i] * contents[j] * contents[k];
					break;
				}
			}
		}
	}

	println!("[*] Result: {}", result);
}