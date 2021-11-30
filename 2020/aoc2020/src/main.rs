mod lib;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;

fn run_day(day_num: u32) {
	match day_num {
		1 => {
			day1::run_part1();
    		day1::run_part2();
		},
		2 => {
			day2::run_part1();
			day2::run_part2();
		},
		3 => {
			day3::run_part1();
			day3::run_part2();
		},
		4 => {
			day4::run_part1();
			day4::run_part2();
		},
		5 => {
			day5::run_part1();
			day5::run_part2();
		},
		6 => {
			day6::run_part1();
			day6::run_part2();
		},
		7 => {
			day7::run_part1();
			day7::run_part2();
		},
		8 => {
			day8::run_part1();
			day8::run_part2();
		},
		9 => {
			day9::run_part1();
			day9::run_part2();
		},
		10 => {
			day10::run_part1();
			day10::run_part2();
		},
		11 => {
			day11::run_part1();
			day11::run_part2();
		},
		12 => {
			day12::run_part1();
			day12::run_part2();
		},
		13 => {
			day13::run_part1();
			day13::run_part2();
		},
		14 => {
			day14::run_part1();
			day14::run_part2();
		},
		15 => {
			day15::run_part1();
			day15::run_part2();
		},
		16 => {
			day16::run_part1();
			day16::run_part2();
		},
		17 => {
			day17::run_part1();
			day17::run_part2();
		},
		18 => {
			day18::run_part1();
			day18::run_part2();
		},
		19 => {
			day19::run_part1();
			day19::run_part2();
		},
		20 => {
			day20::run_part1();
			day20::run_part2();
		},
		_ => panic!("[!] ERROR: invalid day picked")
	}
}

fn main() {
    run_day(13);
    run_day(14);
    run_day(16);
    run_day(18);
    run_day(19);
    run_day(20);
}
