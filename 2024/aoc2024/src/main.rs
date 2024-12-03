use clap::Parser;
use log::info;

use aoc2024 as aoc;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    day: usize,
}

fn main() {
    // initialze the logger
    simple_logger::init_with_level(log::Level::Info).unwrap();

    // parse arguments
    let args = Args::parse();
    match args.day {
        1 => {
            let input = aoc::common::read_file("./src/exercises/day1/input.txt");
            //let input = aoc::common::read_file("./src/exercises/day1/input_test.txt");
            //let input2 = aoc::common::read_file("./src/exercises/day1/input_test.txt");
            let input2 = aoc::common::read_file("./src/exercises/day1/input.txt");
            aoc::exercises::day1::run_part1(&input);
            aoc::exercises::day1::run_part2(&input2);
        }
        2 => {
            //let input = aoc::common::read_file("./src/exercises/day2/input_test.txt");
            let input = aoc::common::read_file("./src/exercises/day2/input.txt");
            aoc::exercises::day2::run_part1(&input);
            aoc::exercises::day2::run_part2(&input);
        }
        //        3 => {
        //            let input = aoc::common::read_file("./src/exercises/day3/input.txt");
        //            aoc::exercises::day3::run_part1(&input);
        //            aoc::exercises::day3::run_part2(&input);
        //        }
        //        4 => {
        //            let input = aoc::common::read_file("./src/exercises/day4/input.txt");
        //            aoc::exercises::day4::run_part1(&input);
        //            aoc::exercises::day4::run_part2(&input);
        //        }
        //        5 => {
        //            let input = aoc::common::read_file_notrim("./src/exercises/day5/input.txt");
        //            aoc::exercises::day5::run_part1(&input);
        //            aoc::exercises::day5::run_part2(&input);
        //        }
        //        6 => {
        //            let input = aoc::common::read_file("./src/exercises/day6/input.txt");
        //            aoc::exercises::day6::run_part1(&input);
        //            aoc::exercises::day6::run_part2(&input);
        //        }
        //        7 => {
        //            let input = aoc::common::read_file("./src/exercises/day7/input.txt");
        //            aoc::exercises::day7::run_part1(&input);
        //            aoc::exercises::day7::run_part2(&input);
        //        }
        //        8 => {
        //            let input = aoc::common::read_file("./src/exercises/day8/input.txt");
        //            aoc::exercises::day8::run_part1(&input);
        //            aoc::exercises::day8::run_part2(&input);
        //        }
        //        9 => {
        //            let input = aoc::common::read_file("./src/exercises/day9/input.txt");
        //            aoc::exercises::day9::run_part1(&input);
        //            aoc::exercises::day9::run_part2(&input);
        //        }
        //        10 => {
        //            let input = aoc::common::read_file("./src/exercises/day10/input.txt");
        //            aoc::exercises::day10::run_part1(&input);
        //            aoc::exercises::day10::run_part2(&input);
        //        }
        //        11 => {
        //            let input = aoc::common::read_file("./src/exercises/day11/input_test.txt");
        //            aoc::exercises::day11::run_part1(&input);
        //            aoc::exercises::day11::run_part2(&input);
        //        }
        //        12 => {
        //            let input = aoc::common::read_file("./src/exercises/day12/input.txt");
        //            aoc::exercises::day12::run_part1(&input);
        //            aoc::exercises::day12::run_part2(&input);
        //        }
        //        13 => {
        //            let input = aoc::common::read_file("./src/exercises/day13/input.txt");
        //            aoc::exercises::day13::run_part1(&input);
        //            aoc::exercises::day13::run_part2(&input);
        //        }
        //        14 => {
        //            let input = aoc::common::read_file("./src/exercises/day14/input.txt");
        //            aoc::exercises::day14::run_part1(&input);
        //            aoc::exercises::day14::run_part2(&input);
        //        }
        //        15 => {
        //            let input = aoc::common::read_file("./src/exercises/day15/input.txt");
        //            aoc::exercises::day15::run_part1(&input);
        //            aoc::exercises::day15::run_part2(&input);
        //        }
        //        16 => {
        //            let input = aoc::common::read_file("./src/exercises/day16/input.txt");
        //            aoc::exercises::day16::run_part1(&input);
        //            aoc::exercises::day16::run_part2(&input);
        //        }
        //        17 => {
        //            let input = aoc::common::read_file("./src/exercises/day17/input.txt");
        //            aoc::exercises::day17::run_part1(&input);
        //            aoc::exercises::day17::run_part2(&input);
        //        }
        //        18 => {
        //            let input = aoc::common::read_file("./src/exercises/day18/input.txt");
        //            aoc::exercises::day18::run_part1(&input);
        //            aoc::exercises::day18::run_part2(&input);
        //        }
        //        19 => {
        //            let input = aoc::common::read_file("./src/exercises/day19/input.txt");
        //            aoc::exercises::day19::run_part1(&input);
        //            aoc::exercises::day19::run_part2(&input);
        //        }
        //        20 => {
        //            let input = aoc::common::read_file("./src/exercises/day20/input.txt");
        //            aoc::exercises::day20::run_part1(&input);
        //            aoc::exercises::day20::run_part2(&input);
        //        }
        //        21 => {
        //            let input = aoc::common::read_file("./src/exercises/day21/input.txt");
        //            aoc::exercises::day21::run_part1(&input);
        //            aoc::exercises::day21::run_part2(&input);
        //        }
        //        22 => {
        //            let input = aoc::common::read_file("./src/exercises/day22/input.txt");
        //            aoc::exercises::day22::run_part1(&input);
        //            aoc::exercises::day22::run_part2(&input);
        //        }
        //        23 => {
        //            let input = aoc::common::read_file("./src/exercises/day23/input.txt");
        //            aoc::exercises::day23::run_part1(&input);
        //            aoc::exercises::day23::run_part2(&input);
        //        }
        //        24 => {
        //            let input = aoc::common::read_file("./src/exercises/day24/input.txt");
        //            aoc::exercises::day24::run_part1(&input);
        //            aoc::exercises::day24::run_part2(&input);
        //        }
        //        25 => {
        //            let input = aoc::common::read_file("./src/exercises/day25/input.txt");
        //            aoc::exercises::day25::run_part1(&input);
        //            aoc::exercises::day25::run_part2(&input);
        //        }
        _ => panic!("invalid day picked"),
    }
}
