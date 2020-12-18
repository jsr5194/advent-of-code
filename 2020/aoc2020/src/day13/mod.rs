use std::fs;

fn get_input() -> Schedule {
	let filename = "./src/day13/test_input.txt";
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
//	let sched = get_input();

//	let mut N = 1;
//	for idx in 0..sched.busses.len() {
//		if sched.busses[idx].id != 0 {
//			N *= sched.busses[idx].id;
//		}
//	}

	println!("Day 13 Part 2: TODO");

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

//#[derive(Debug, Default, Clone, PartialEq)]
//struct ExtEuclid {
//	a: usize,
//	b: usize,
//	gcd: usize,
//	s: usize,
//	t: usize,
//}
//
//impl ExtEuclid {
//	fn solve(&mut self) {
//		let mut r0: usize = self.a;
//		let mut r1: usize = self.b;
//		let mut s0: usize = 1;
//		let mut s1: usize = 0;
//		let mut t0: usize = 0;
//		let mut t1: usize = 1;
//
//		loop {
//			let q = r0 / r1;
//			let r2 = r0 - q * r1;
//
//			if r2 == 0 {
//				break;
//			}
//
//			let s2 = s0 - q * s1;
//			let t2 = t0 - q * t1;
//
//			// set up for next round
//			r0 = r1;
//			r1 = r2;
//			s0 = s1;
//			s1 = s2;
//			t0 = t1;
//			t1 = t2;
//
//			// save off results
//			self.gcd = r2;
//			self.s = s2;
//			self.t = t2;
//		}
//	}
//}
