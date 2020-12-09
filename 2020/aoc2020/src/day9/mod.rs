use std::fs;

const PREAMBLE_SIZE_TEST: usize = 5;
const PREAMBLE_SIZE_PRODUCTION: usize = 25;

fn get_input() -> XMAS {
	let filename = "./src/day9/input.txt";
	let contents_str = fs::read_to_string(filename).expect("could not read file");

	let mut contents = XMAS::default();
	contents.preamble_size = PREAMBLE_SIZE_PRODUCTION;
	for line in contents_str.lines() {
		contents.data.push(line.parse::<usize>().expect("could not convert line to usize"));
	}
	contents
}

pub fn run_part1() {
	let mut data = get_input();

	println!("Day 9 Part 1 Result: {:?}", data.find_invalid_pair());
}

pub fn run_part2() {
	let mut data = get_input();

	let mut result = data.find_encryption_weakness();
	result.sort();
	let low = result.first().expect("could not get first element");
	let high = result.last().expect("could not get last element");

	println!("Day 9 Part 2 Result: {:?}", low+high);

}



#[derive(Debug, Default, Clone)]
struct XMAS {
	preamble_size: usize,
	data: Vec<usize>,
}

impl XMAS {

	fn find_invalid_pair(&self) -> usize {
		let mut cur_idx: usize = self.preamble_size;
		let mut cur_idx_valid = false;

		// loop over each value of the XMAS data excluding the preamble
		for _ in self.preamble_size..self.data.len() {
			// reset our start index each loop using the updated current index
			let start_idx: usize = cur_idx - self.preamble_size;

			// reset our flag indicating whether or not the current index is okay
			cur_idx_valid = false;

			// dual loop over the values allowed for testing searching for a match
			for i in start_idx..start_idx+self.preamble_size {
				for j in start_idx+1..start_idx+self.preamble_size {
					// a match is found when two numbers add up to our current number
					if self.data[i] + self.data[j] == self.data[cur_idx] {
						cur_idx_valid = true;
						break;
					}
				}
				if cur_idx_valid {
					break;
				}
			}

			if !cur_idx_valid {
				return self.data[cur_idx];
			}

			cur_idx += 1;
		}
		panic!("The invalid value could not be found");
	}

	fn find_encryption_weakness(&self) -> Vec<usize> {
		let target_value = self.find_invalid_pair();

		let mut result: Vec<usize> = vec![];

		for i in 0..self.data.len() {
			let mut is_too_big: bool = false;
			for j in 1..self.data.len() {
				let mut cur_result = 0;
				for idx in i..j {
					cur_result += self.data[idx];

					// bail if we know its too large
					if cur_result > target_value {
						is_too_big = true;
						break;
					}
				}

				// return the result if its successful
				if cur_result == target_value {
					for idx in i..j {
						result.push(self.data[idx]);
					}
					return result;
				}

				// bail if we know its too large
				if is_too_big {
					break;
				}
			}
		}
		panic!("The weakness could not be found");
	}
}
