use std::fs;

fn get_input() -> Transmission {
	let filename = "./src/day19/test_input.txt";
	let contents = fs::read_to_string(filename).unwrap();
	let mut transmission = Transmission::default();
	let mut reading_rules = true;
	for line in contents.lines() {
		if line == String::default() {
			reading_rules = false;
		} else if reading_rules {
			transmission.rules.push(Rule::from(line));
		} else {
			transmission.messages.push(Message::from(line));
		}
	}

	transmission
}

pub fn run_part1() {
	let mut transmission = get_input();

	let target_rule = 0;

	let expanded_rules = transmission.expand_rule(target_rule);

	println!("{:?}", expanded_rules);

	let mut count = 0;
	for msg in &transmission.messages {
		print!("Msg: {:?}", msg);
		if expanded_rules.contains(&msg.contents) {
			println!("!!!!!!!");
			count += 1;
		} else {
			println!("");
		}
	}


	println!("Day 19 Part 1: {}", count);
}

pub fn run_part2() {
	println!("Day 19 Part 2: TODO");
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Transmission {
	rules: Vec<Rule>,
	messages: Vec<Message>,
}

impl Transmission {
	fn get_rule(&self, id: usize) -> &Rule {
		for rule in &self.rules {
			if rule.id == id {
				return &rule
			}
		}
		panic!("could not find rule");
	}

	fn update_rule(&mut self, id: usize, new_contents: &Vec<String>) {
		let mut id_found = false;
		for cur_id in 0..self.rules.len() {
			if self.rules[cur_id].id == id {
				self.rules[cur_id].contents = new_contents.clone();
				id_found = true;
			}
		}
		if !id_found {
			panic!("could not find rule");
		}
	}

	fn expand_rule(&mut self, id: usize) -> Vec<String> {
		let rule = self.get_rule(id);

		let mut expanded_rules: Vec<String> = vec![String::default()];

		for content in &rule.contents {
			let mut new_expanded_rules: Vec<String> = vec![];
			for sub_id in content.chars() {
				let sub_rule = self.get_rule(sub_id.to_string().parse::<usize>().unwrap());
				if sub_rule.rule_type == RuleType::RawRule {
					for idx in 0..expanded_rules.len() {
						new_expanded_rules.push(expanded_rules[idx].clone() + sub_id.clone().to_string().as_str());
					}
				} else {
					for idx in 0..expanded_rules.len() {
						for sub_contents in &sub_rule.contents {
							let mut new_rule = expanded_rules[idx].clone();
							for sub_id in sub_contents.chars() {
								new_rule += sub_id.to_string().as_str();
							}
							new_expanded_rules.push(new_rule.clone());
						}
					}
				}
				println!("");
			}
			expanded_rules = new_expanded_rules.clone();
		}

		self.update_rule(id, &expanded_rules);

		let mut is_fully_expanded = true;
		for content in &self.get_rule(id).contents {
			for id in content.chars() {
				if self.get_rule(id.to_string().parse::<usize>().unwrap()).rule_type != RuleType::RawRule {
					is_fully_expanded = false;
				}
			}
		}

		if !is_fully_expanded {
			self.expand_rule(id);
		}

		let mut resolved_rules: Vec<String> = vec![];
		for rule in &self.get_rule(id).contents {
			let mut cur_rule = String::default();
			for c in rule.chars() {
				cur_rule += self.get_rule(c.to_string().parse::<usize>().unwrap()).contents.first().unwrap();
			}
			resolved_rules.push(cur_rule);
		}

		resolved_rules
	}
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Rule {
	id: usize,
	rule_type: RuleType,
	contents: Vec<String>,
}

impl From<&str> for Rule {
	fn from(raw_rule: &str) -> Self {
		let mut rule = Rule::default();
		let kvp: Vec<String> = raw_rule.split(": ").map(|x| x.to_string()).collect();
		rule.id = kvp[0].parse::<usize>().unwrap();

		if kvp[1].contains("\"") {
			rule.rule_type = RuleType::RawRule;
			rule.contents.push(kvp[1].split("\"").map(|x| x.to_string()).collect::<Vec<String>>()[1].clone());
		} else {
			rule.rule_type = RuleType::EncapRule;
			for group in kvp[1].split(" | ") {
				let mut content = String::default();
				for id in group.split(" ") {
					content += id.clone();
				}
				rule.contents.push(content);
			}
		}

		rule
	}
}

#[derive(Debug, Clone, PartialEq)]
enum RuleType {
	EncapRule,
	RawRule,
	NoType,
}

impl Default for RuleType {
	fn default() -> Self {
		RuleType::NoType
	}
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Message {
	contents: String,
}

impl From<&str> for Message {
	fn from(raw_message: &str) -> Self {
		Message{contents: raw_message.clone().to_string()}
	}
}
