use std::fs;

fn get_input() -> Schedule {
	let filename = "./src/day13/input.txt";
	let contents = fs::read_to_string(filename).unwrap();
	let contents_lines: Vec<String> = contents.lines().map(|x| x.to_string()).collect();
	let mut sched = Schedule::default();
	sched.earliest_departure = contents_lines.first().unwrap().parse::<usize>().unwrap();
	for bus_id in contents_lines[1].split(",") {
		let mut bus = Bus::default();
		if bus_id == "x" {
			bus.id = 0;
		} else {
			bus.id = bus_id.parse::<usize>().unwrap();
		}
		sched.busses.push(bus.clone());
	}
	sched
}

pub fn run_part1() {
	let sched = get_input();

	let mut earliest_bus = 0;
	let mut earliest_bus_time = usize::MAX;
	for bus in sched.busses {
		if bus.id > 0 {
			let cur_bus_time = bus.get_next_departure(sched.earliest_departure);
			if cur_bus_time < earliest_bus_time {
				earliest_bus = bus.id;
				earliest_bus_time = cur_bus_time;
			}
		}
	}

	println!("Day 13 Part 1: {:?}", earliest_bus*(earliest_bus_time - sched.earliest_departure));
}

pub fn run_part2() {

}

#[derive(Debug, Default, Clone, PartialEq)]
struct Schedule {
	earliest_departure: usize,
	busses: Vec<Bus>,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Bus {
	id: usize,
}

impl Bus {
	fn get_next_departure(&self, earliest_departure: usize) -> usize {
		earliest_departure + self.id - (earliest_departure % self.id)
	}
}