mod lib;
mod day1;
//mod day2;
//mod day3;
//mod day4;
//mod day5;
//mod day6;
//mod day7;
//mod day8;
//mod day9;
//mod day10;
//mod day11;
//mod day12;
//mod day13;
//mod day14;
//mod day15;
//mod day16;
//mod day17;
//mod day18;
//mod day19;
//mod day20;
//mod day21;
//mod day22;
//mod day23;
//mod day24;
//mod day25;

fn run_day(day_num: usize) {
	match day_num {
		1 => {
			day1::run_part_1();
			day1::run_part_2();
		},
//		2 => {
//			day2::run_part_1();
//			day2::run_part_2();
//		},
//		3 => {
//			day3::run_part_1();
//			day3::run_part_2();
//		},
//		4 => {
//			day4::run_part_1();
//			day4::run_part_2();
//		},
//		5 => {
//			day5::run_part_1();
//			day5::run_part_2();
//		},
//		6 => {
//			day6::run_part_1();
//			day6::run_part_2();
//		},
//		7 => {
//			day7::run_part_1();
//			day7::run_part_2();
//		},
//		8 => {
//			day8::run_part_1();
//			day8::run_part_2();
//		},
//		9 => {
//			day9::run_part_1();
//			day9::run_part_2();
//		},
//		10 => {
//			day10::run_part_1();
//			day10::run_part_2();
//		},
//		11 => {
//			day11::run_part_1();
//			day11::run_part_2();
//		},
//		12 => {
//			day12::run_part_1();
//			day12::run_part_2();
//		},
//		13 => {
//			day13::run_part_1();
//			day13::run_part_2();
//		},
//		14 => {
//			day14::run_part_1();
//			day14::run_part_2();
//		},
//		15 => {
//			day15::run_part_1();
//			day15::run_part_2();
//		},
//		16 => {
//			day16::run_part_1();
//			day16::run_part_2();
//		},
//		17 => {
//			day17::run_part_1();
//			day17::run_part_2();
//		},
//		18 => {
//			day18::run_part_1();
//			day18::run_part_2();
//		},
//		19 => {
//			day19::run_part_1();
//			day19::run_part_2();
//		},
//		20 => {
//			day20::run_part_1();
//			day20::run_part_2();
//		},
//		21 => {
//			day21::run_part_1();
//			day21::run_part_2();
//		},
//		22 => {
//			day22::run_part_1();
//			day22::run_part_2();
//		},
//		23 => {
//			day23::run_part_1();
//			day23::run_part_2();
//		},
//		24 => {
//			day24::run_part_1();
//			day24::run_part_2();
//		},
//		25 => {
//			day25::run_part_1();
//			day25::run_part_2();
//		},
		_ => panic!("[!] ERROR: invalid day picked")
	}
}

fn main() {
	run_day(1);
}