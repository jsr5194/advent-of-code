mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn run_day(day_num: u32) {
	match day_num {
		1 => {
			day1::run_part1();
    		day1::run_part2();
		}
		2 => {
			day2::run_part1();
			day2::run_part2();
		}
		3 => {
			day3::run_part1();
			day3::run_part2();
		}
		4 => {
			day4::run_part1();
			day4::run_part2();
		}
		5 => {
			day5::run_part1();
			day5::run_part2();
		}
		_ => panic!("[!] ERROR: invalid day picked")
	}
}

fn main() {
    run_day(5);
}
