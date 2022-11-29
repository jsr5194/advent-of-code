use clap::Parser;
use log::info;

use aoc2022 as aoc;

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
            aoc::exercises::day1::run_part1();
            aoc::exercises::day1::run_part2();
        }
        2 => {
            aoc::exercises::day2::run_part1();
            aoc::exercises::day2::run_part2();
        }
        3 => {
            aoc::exercises::day3::run_part1();
            aoc::exercises::day3::run_part2();
        }
        4 => {
            aoc::exercises::day4::run_part1();
            aoc::exercises::day4::run_part2();
        }
        5 => {
            aoc::exercises::day5::run_part1();
            aoc::exercises::day5::run_part2();
        }
        6 => {
            aoc::exercises::day6::run_part1();
            aoc::exercises::day6::run_part2();
        }
        7 => {
            aoc::exercises::day7::run_part1();
            aoc::exercises::day7::run_part2();
        }
        8 => {
            aoc::exercises::day8::run_part1();
            aoc::exercises::day8::run_part2();
        }
        9 => {
            aoc::exercises::day9::run_part1();
            aoc::exercises::day9::run_part2();
        }
        10 => {
            aoc::exercises::day10::run_part1();
            aoc::exercises::day10::run_part2();
        }
        11 => {
            aoc::exercises::day11::run_part1();
            aoc::exercises::day11::run_part2();
        }
        12 => {
            aoc::exercises::day12::run_part1();
            aoc::exercises::day12::run_part2();
        }
        13 => {
            aoc::exercises::day13::run_part1();
            aoc::exercises::day13::run_part2();
        }
        14 => {
            aoc::exercises::day14::run_part1();
            aoc::exercises::day14::run_part2();
        }
        15 => {
            aoc::exercises::day15::run_part1();
            aoc::exercises::day15::run_part2();
        }
        16 => {
            aoc::exercises::day16::run_part1();
            aoc::exercises::day16::run_part2();
        }
        17 => {
            aoc::exercises::day17::run_part1();
            aoc::exercises::day17::run_part2();
        }
        18 => {
            aoc::exercises::day18::run_part1();
            aoc::exercises::day18::run_part2();
        }
        19 => {
            aoc::exercises::day19::run_part1();
            aoc::exercises::day19::run_part2();
        }
        20 => {
            aoc::exercises::day20::run_part1();
            aoc::exercises::day20::run_part2();
        }
        21 => {
            aoc::exercises::day21::run_part1();
            aoc::exercises::day21::run_part2();
        }
        22 => {
            aoc::exercises::day22::run_part1();
            aoc::exercises::day22::run_part2();
        }
        23 => {
            aoc::exercises::day23::run_part1();
            aoc::exercises::day23::run_part2();
        }
        24 => {
            aoc::exercises::day24::run_part1();
            aoc::exercises::day24::run_part2();
        }
        25 => {
            aoc::exercises::day25::run_part1();
            aoc::exercises::day25::run_part2();
        }
        _ => panic!("invalid day picked"),
    }
}
