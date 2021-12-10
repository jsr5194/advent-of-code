use aoc2021 as aoc;

fn run_day(day_num: usize) {
	match day_num {
		1 => {
			aoc::day1::run_part_1();
			aoc::day1::run_part_2();
		},
		2 => {
			aoc::day2::run_part_1();
			aoc::day2::run_part_2();
		},
		3 => {
			aoc::day3::run_part_1();
			aoc::day3::run_part_2();
		},
		4 => {
			aoc::day4::run_part_1();
			aoc::day4::run_part_2();
		},
		5 => {
			aoc::day5::run_part_1();
			aoc::day5::run_part_2();
		},
		6 => {
			aoc::day6::run_part_1();
			aoc::day6::run_part_2();
		},
		7 => {
			aoc::day7::run_part_1();
			aoc::day7::run_part_2();
		},
		8 => {
			aoc::day8::run_part_1();
			aoc::day8::run_part_2();
		},
		9 => {
			aoc::day9::run_part_1();
			aoc::day9::run_part_2();
		},
		10 => {
			aoc::day10::run_part_1();
			aoc::day10::run_part_2();
		},
//		11 => {
//			aoc::day11::run_part_1();
//			aoc::day11::run_part_2();
//		},
//		12 => {
//			aoc::day12::run_part_1();
//			aoc::day12::run_part_2();
//		},
//		13 => {
//			aoc::day13::run_part_1();
//			aoc::day13::run_part_2();
//		},
//		14 => {
//			aoc::day14::run_part_1();
//			aoc::day14::run_part_2();
//		},
//		15 => {
//			aoc::day15::run_part_1();
//			aoc::day15::run_part_2();
//		},
//		16 => {
//			aoc::day16::run_part_1();
//			aoc::day16::run_part_2();
//		},
//		17 => {
//			aoc::day17::run_part_1();
//			aoc::day17::run_part_2();
//		},
//		18 => {
//			aoc::day18::run_part_1();
//			aoc::day18::run_part_2();
//		},
//		19 => {
//			aoc::day19::run_part_1();
//			aoc::day19::run_part_2();
//		},
//		20 => {
//			aoc::day20::run_part_1();
//			aoc::day20::run_part_2();
//		},
//		21 => {
//			aoc::day21::run_part_1();
//			aoc::day21::run_part_2();
//		},
//		22 => {
//			aoc::day22::run_part_1();
//			aoc::day22::run_part_2();
//		},
//		23 => {
//			aoc::day23::run_part_1();
//			aoc::day23::run_part_2();
//		},
//		24 => {
//			aoc::day24::run_part_1();
//			aoc::day24::run_part_2();
//		},
//		25 => {
//			aoc::day25::run_part_1();
//			aoc::day25::run_part_2();
//		},
		_ => panic!("[!] ERROR: invalid day picked")
	}
}

fn main() {
	//run_day(8);
	//run_day(9);
	run_day(10);
}