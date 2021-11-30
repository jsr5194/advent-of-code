use std::fmt;
use std::fs;

fn get_input() -> Vec<Equation> {
	let filename = "./src/day18/test_input.txt";
	let contents = fs::read_to_string(filename).unwrap();

	let mut equations: Vec<Equation> = vec![];
	for raw_line in contents.lines() {
		let mut last_c = ' ';
		let mut line = String::default();
		for c in raw_line.chars() {
			if c != ' ' {
				if last_c != ' ' {
					line += " ";
				}
			}
			
			line += c.to_string().as_str();

			last_c = c;
		}
		equations.push(Equation{equation: line});
	}

	equations
}

pub fn run_part1() {
	let equations = get_input();

	println!("Day 18 Part 1: TODO");
}

pub fn run_part2() {
	println!("Day 18 Part 2: TODO");
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Equation {
	equation: String,
}

fn evaluate_equation(equation: &String) {
	println!("Equation: {:?}", *equation);
	let mut level = 0;
	for elem in equation.split(" ") {
		match elem {
			"(" => {
				level += 1;
			},
			")" => {
				level -= 1;
			},
			" " => {
				();
			},
			_ => {
				for _ in 0..level {
					print!(" ");
				}
				println!("{:?}", elem);
			}
		}
	}


	println!("\n\n");
}