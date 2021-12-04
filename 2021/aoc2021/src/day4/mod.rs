use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;

fn get_input() -> BingoGame {
	let filename = "./src/day4/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: an error occurred while reading the file");
	let contents: BingoGame = BingoGame::from(contents_str);
	contents
}

/// run Day 4 Part 1
pub fn run_part_1() {
	let mut game: BingoGame = get_input();
	let mut winning_card = BingoCard::default();
	let mut last_called_num = game.called_numbers[0];
	let mut winning_card_found = false;

	// call a new number
	for cur_num in game.called_numbers {
		last_called_num = cur_num;
		// check and mark each card when appropriate
		for card in &mut game.cards {
			// check row by row
			for row in &mut card.card {
				for square in row {
					if square.value == cur_num {
						square.set_stamped();
					}
				}
			}
			// check if the card won
			if card.is_winner() {
				winning_card = card.clone();
				winning_card_found = true;
				break;
			}
		}
		if winning_card_found {
			break;
		}
	}

	// sanity check
	if winning_card == BingoCard::default() {
		panic!("ERROR: No winning card was found");
	}

	let mut score = 0;
	for row in &winning_card.card {
		for square in row {
			if !square.stamped {
				score += square.value;
			}
		}
	}

	println!("[*] Day 4 Part 1 Result: {}", score * last_called_num);
}

/// run Day 4 Part 2
pub fn run_part_2() {
	let mut game: BingoGame = get_input();
	let mut last_winning_card = BingoCard::default();
	let mut last_called_num = game.called_numbers[0];
	let mut last_winning_card_found = false;
	let total_cards = game.cards.len();
	let mut winning_cards_found = 0;

	// call a new number
	for cur_num in game.called_numbers {
		last_called_num = cur_num;
		// check and mark each card when appropriate
		for card in &mut game.cards {
			if !card.won {
				// check row by row
				for row in &mut card.card {
					for square in row {
						if square.value == cur_num {
							square.set_stamped();
						}
					}
				}
				// check if the card won
				if card.is_winner() {
					winning_cards_found += 1;
					if winning_cards_found == total_cards {
						last_winning_card = card.clone();
						last_winning_card_found = true;
						break;
					}
				}
			}
		}
		if last_winning_card_found {
			break;
		}
	}

	// sanity check
	if last_winning_card == BingoCard::default() {
		panic!("ERROR: No winning card was found");
	}

	let mut score = 0;
	for row in &last_winning_card.card {
		for square in row {
			if !square.stamped {
				score += square.value;
			}
		}
	}

	println!("[*] Day 4 Part 2 Result: {}", score * last_called_num);
}


#[derive(Default, Debug, Clone, PartialEq)]
struct BingoSquare {
	value: u32,
	stamped: bool,
}

impl FromStr for BingoSquare {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(BingoSquare{ value: s.parse::<u32>()?, stamped: false })
	}
}

impl BingoSquare {
	fn set_stamped(&mut self) {
		self.stamped = true
	}
}

#[derive(Default, Debug, Clone, PartialEq)]
struct BingoCard {
	card: Vec<Vec<BingoSquare>>,
	won: bool,
}

impl BingoCard {
	fn is_winner(&mut self) -> bool {
		// set bingo tracker
		let mut bingo = true;

		// check by row
		for row in &self.card {
			bingo = true;
			for col in row {
				if !col.stamped {
					bingo = false;
					break;
				}
			}
			if bingo {
				break;
			}
		}

		// check by column if row wasn't a winner
		if !bingo {
			for col in 0..self.card[0].len() {
				bingo = true;
				for row in 0..self.card.len() {
					if !self.card[row][col].stamped {
						bingo = false;
						break;
					}
				}
				if bingo {
					break;
				}
			}
		}

		if bingo {
			self.won = true;
		}

		bingo
	}
}

#[derive(Default, Debug, Clone, PartialEq)]
struct BingoGame {
	called_numbers: Vec<u32>,
	cards: Vec<BingoCard>,
}

impl From<String> for BingoGame {
	fn from(game_str: String) -> Self {
		let mut game = BingoGame::default();

		let mut first_run: bool = true;
		for line in game_str.lines() {
			if !first_run {
				if line == "" {
					game.cards.push(BingoCard::default());
				} else {
					let cur_card: Vec<BingoSquare> = line.split_whitespace().map(|x| x.parse::<BingoSquare>().unwrap()).collect();
					let idx = game.cards.len() - 1;
					game.cards[idx].card.push(cur_card);
				}
			} else {
				game.called_numbers = line.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
				first_run = false;
			}
		}

		game
	}
}
