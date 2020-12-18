use std::fs;

fn get_input() -> Notes {
	let filename = "./src/day16/test_input.txt";
	let contents = fs::read_to_string(filename).unwrap();

	let mut notes = Notes::default();

	let mut parser_flag = ParserFlag::Fields;
	for line in contents.lines() {
		if line == String::default() {
			// pass
		} else if line == "your ticket:" {
			parser_flag = ParserFlag::MyTicket;
		} else if line == "nearby tickets:" {
			parser_flag = ParserFlag::NearbyTickets;
		} else {
			match parser_flag {
				ParserFlag::Fields => {
					notes.fields.push(Field::from(line));
				},
				ParserFlag::MyTicket => {
					notes.my_ticket = Ticket::from(line);
				},
				ParserFlag::NearbyTickets => {
					notes.nearby_tickets.push(Ticket::from(line));
				},
			}
		}
	}

	notes
}

pub fn run_part1() {
	let notes = get_input();

	println!("Day 16 Part 1 Result: {}", notes.get_error_rate());
}

pub fn run_part2() {
	let mut notes = get_input();

	notes.remove_bad_tickets();

	println!("Day 16 Part 2 Result: TODO");
}


#[derive(Debug, Default, Clone, PartialEq)]
struct Notes {
	fields: Vec<Field>,
	my_ticket: Ticket,
	nearby_tickets: Vec<Ticket>,
}

impl Notes {
	fn get_error_rate(&self) -> usize {
		let mut error_rate = 0;
		for ticket in &self.nearby_tickets {
			// iterate over each ticket logging the bad ones
			for value in &ticket.ticket {
				let mut value_seen_count = 0;
				for field in &self.fields {
					if field.contains(*value) {
						value_seen_count += 1;
					}
				}
				if value_seen_count == 0 {
					error_rate += value;
				}
			}
		}

		error_rate
	}
	fn remove_bad_tickets(&mut self) {
		let mut bad_tickets: Vec<Ticket> = vec![];
		for ticket in &self.nearby_tickets {
			// iterate over each ticket logging the bad ones
			for value in &ticket.ticket {
				let mut value_seen_count = 0;
				for field in &self.fields {
					if field.contains(*value) {
						value_seen_count += 1;
					}
				}
				if value_seen_count == 0 {
					bad_tickets.push(ticket.clone());
					break;
				}
			}
		}

		// iterate backwards over the bad ticket indicies removing where possible
		for bad_ticket in bad_tickets {
			self.nearby_tickets.retain(|x| x != &bad_ticket);
		}
	}

}

#[derive(Debug, Default, Clone, PartialEq)]
struct Field {
	name: String,
	lo_range: Vec<usize>,
	hi_range: Vec<usize>,
}

impl From<&str> for Field {
	fn from(field_line: &str) -> Self {
		let kvp: Vec<String> = field_line.split(": ").map(|x| x.to_string()).collect();
		let ranges: Vec<String> = kvp[1].split(" or ").map(|x| x.to_string()).collect();
		let lo_range: Vec<usize> = ranges[0].split("-").map(|x| x.parse::<usize>().unwrap()).collect();
		let hi_range: Vec<usize> = ranges[1].split("-").map(|x| x.parse::<usize>().unwrap()).collect();

		Field {
			name: kvp.first().unwrap().clone(),
			lo_range: lo_range.clone(),
			hi_range: hi_range.clone(),
		}
	}
}

impl Field {
	fn contains(&self, value: usize) -> bool {
		if value >= self.lo_range[0] && value <= self.lo_range[1] {
			true
		} else if value >= self.hi_range[0] && value <= self.hi_range[1] {
			true
		} else {
			false
		}
	}
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Ticket {
	ticket: Vec<usize>,
}

impl From<&str> for Ticket {
	fn from(ticket_line: &str) -> Self {
		let ticket_values: Vec<usize> = ticket_line.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
		Ticket {
			ticket: ticket_values,
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
enum ParserFlag {
	Fields,
	MyTicket,
	NearbyTickets,
}