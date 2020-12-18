use std::collections::HashMap;
use std::fs;

fn get_input() -> Game {
	let filename = "./src/day15/input.txt";
	let contents = fs::read_to_string(filename).unwrap();

	let mut game = Game::default();

	let mut age = 1;
	for num in contents.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>() {
		game.numbers.insert(num, Number{ages: vec![age]});
		age += 1;
		game.last_number = num;
	}

	game

}

pub fn run_part1() {
	let mut game = get_input();
	game.rounds = 2020;

	game.run();

	println!("Day 15 Part 1: {}", game.last_number);
}

pub fn run_part2() {
	let mut game = get_input();
	game.rounds = 30000000;

	game.run();

	println!("Day 15 Part 2: {}", game.last_number);
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Game {
	numbers: HashMap<usize, Number>,
	last_number: usize,
	rounds: usize,
}

impl Game {
	fn run(&mut self) {
		assert!(self.rounds > self.numbers.len()+1);

		for turn in self.numbers.len()+1..self.rounds+1 {
			let mut spoken_number: usize = 0;

			// determine if last number has been said
			let last_number_already_said: bool;
			if self.numbers.contains_key(&self.last_number) {
				last_number_already_said = true;
			} else {
				last_number_already_said = false;
			}

			// determine if the last number has only been said once
			let last_number_said_once: bool;
			if self.numbers.get(&self.last_number).unwrap().ages.len() == 1 {
				last_number_said_once = true;
			} else {
				last_number_said_once = false;
			}


			let ages = &self.numbers.get(&self.last_number).unwrap().ages;
			if last_number_already_said {
				// spoken number defaults to zero
				if !last_number_said_once {
					spoken_number = ages[ages.len()-1] - ages[ages.len()-2];
				}

				if self.numbers.contains_key(&spoken_number) {
					self.numbers.get_mut(&spoken_number).unwrap().ages.push(turn);
				} else {
					self.numbers.insert(spoken_number, Number{ages: vec![turn]});
				}
			} else {
				// spoken number defaults to zero
				self.numbers.insert(spoken_number, Number{ages: vec![turn]});
			}

			self.last_number = spoken_number;
		}
	}
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Number {
	ages: Vec<usize>,
}
