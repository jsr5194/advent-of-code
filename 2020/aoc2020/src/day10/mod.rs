use std::collections::HashMap;
use itertools::Itertools;
use std::fs;

fn read_input() -> AdapterChain {
	let filename = "./src/day10/input.txt";
	let contents_str = fs::read_to_string(filename).expect("Could not read file");
	let mut contents: Vec<usize> = contents_str.lines().map(|x| x.parse::<usize>().expect("could not convert to int")).collect();
	
	// add the charging outlet
	contents.push(0);

	// sort the results
	contents.sort();

	// create the bag
	let mut adapter_chain = AdapterChain::default();
	adapter_chain.adapters = contents;

	// add in the device adapter
	adapter_chain.adapters.push(adapter_chain.get_device_rating());

	// return
	adapter_chain
}

pub fn run_part1() {
	let adapter_chain = read_input();

	let diffs = adapter_chain.get_voltage_differences();

	let result = diffs.get(&1).expect("could not get entry") * diffs.get(&3).expect("could not get entry");

	println!("Day 10 Part 1: {}", result);
}

pub fn run_part2() {
	let adapter_chain = read_input();

	println!("Day 10 Part 2: {}", adapter_chain.get_num_distinct_arrangements());

}


#[derive(Debug, Default, Clone)]
struct AdapterChain {
	adapters: Vec<usize>,
}

impl AdapterChain {
	fn get_device_rating(&self) -> usize {
		3 + self.adapters.iter().max().expect("could not get max value")
	}

	fn get_voltage_differences(&self) -> HashMap<usize, usize> {
		let mut voltage_differences : HashMap<usize, usize> = HashMap::new();

		for idx in 1..self.adapters.len() {
			let diff = self.adapters[idx] - self.adapters[idx-1];
			if diff > 3 {
				panic!("A difference of more than 3 was detected: {:?} - {:?}", self.adapters[idx], self.adapters[idx-1]);
			}
			if voltage_differences.contains_key(&diff) {
				voltage_differences.insert(diff, voltage_differences.get(&diff).expect("could not get diff") + 1);
			} else {
				voltage_differences.insert(diff, 1);
			}
		}

		voltage_differences
	}

	fn is_valid_arrangement(&self) -> bool {
		for idx in 1..self.adapters.len() {
			if self.adapters.get(idx).expect("could not get index") - self.adapters.get(idx-1).expect("could not get index") > 3 {
				return false;
			}
		}
		true
	}

	fn get_removable_adapters(&self) -> Vec<usize> {
		let mut removable_adapters: Vec<usize> = vec![];
		for idx in 1..self.adapters.len()-1 {
			let mut tmp_adapter_chain = self.clone();
			tmp_adapter_chain.adapters.remove(idx);
			if tmp_adapter_chain.is_valid_arrangement() {
				removable_adapters.push(self.adapters[idx]);
			}
		}

		removable_adapters
	}

	fn get_permanent_adapters(&self) -> Vec<usize> {
		let mut permanent_adapters: Vec<usize> = vec![];
		permanent_adapters.push(0);
		for idx in 1..self.adapters.len()-1 {
			let mut tmp_adapter_chain = self.clone();
			tmp_adapter_chain.adapters.remove(idx);
			if !tmp_adapter_chain.is_valid_arrangement() {
				permanent_adapters.push(self.adapters[idx]);
			}
		}

		// add in device
		permanent_adapters.push(self.adapters
			.last()
			.expect("could not get last permanent adapter")
			.clone());

		permanent_adapters
	}

	fn get_num_distinct_arrangements(&self) -> usize {
		let removable_adapters = self.get_removable_adapters();
		let permanent_adapters = self.get_permanent_adapters();

		let mut result_pattern: Vec<Vec<usize>> = vec![];
		result_pattern.push([].to_vec());

		let mut dynamic_subsets: Vec<Vec<usize>> = vec![];
		dynamic_subsets.push([].to_vec());

		// categorize each of the adapters
		for adapter in &self.adapters {
			if permanent_adapters.contains(adapter) {
				result_pattern
				.last_mut()
				.expect("could not get last pattern vec")
				.push(*adapter);
				if dynamic_subsets.last().expect("no subset").len() != 0 { 
					dynamic_subsets.push([].to_vec());
				}
			} else if removable_adapters.contains(adapter) {
				dynamic_subsets
				.last_mut()
				.expect("could not get last pattern vec")
				.push(*adapter);
				if result_pattern.last().expect("no result pattern").len() != 0 { 
					result_pattern.push([].to_vec());
				}
			} else {
				panic!("value not in either set detected")
			}
		}

		let mut running_total = 1;
		for i in 0..dynamic_subsets.len()-1 {
			let mut range = AdapterChain::default();

			// add in the last element of the pattern before the subset
			range.adapters.push(*result_pattern[i].last_mut().unwrap());

			// add in each of the values from this subset
			for value in &dynamic_subsets[i] {
				range.adapters.push(value.clone());
			}

			// add in the first element of the pattern following the subset
			range.adapters.push(*result_pattern[i+1].first_mut().unwrap());

			running_total *= range.solve_range();

		}

		running_total
	}

	fn solve_range(&self) -> usize {
		// determine all of the adapters that could possibly be removable
		let removable_adapters = self.get_removable_adapters();
 
 		let mut good_count = 1; // we know the starting point is valid
		
		// iterate over the total number of sizes possible not including the start or end
		for count in 1..removable_adapters.len()+1 {
			// get every combination possible of the known removable adapters
			let combos = removable_adapters.iter().combinations(count).unique();
			
			// iterate over each of these possible combinations checking to see if the removal results in a valid arrangement
			for combo in combos {
				// create a temporarialy adapter chain to perform the modifications on
				let mut tmp_adapter_chain = self.clone();

				// get rid of each adapter in the combination for this attempt
				for adapter in combo {
					tmp_adapter_chain.adapters.remove(tmp_adapter_chain.adapters.iter().position(|x| x==adapter).expect("could not find entry"));
				}

				// when the resulting chain is invalid, increase the number of invalid results
				if tmp_adapter_chain.is_valid_arrangement() {
					good_count += 1; 
				}
			}
		}

		good_count
	}
}
