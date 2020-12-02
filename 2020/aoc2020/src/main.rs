mod day1;
mod day2;

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
		_ => panic!("[!] ERROR: invalid day picked")
	}
}

fn main() {
    run_day(2);
}
