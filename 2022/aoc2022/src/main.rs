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
            aoc::exercises::day1::run_part1("./day1/input.txt");
            aoc::exercises::day1::run_part2("./day1/input.txt");
        }
        2 => {
            aoc::exercises::day2::run_part1("./day2/input.txt");
            aoc::exercises::day2::run_part2("./day2/input.txt");
        }
        3 => {
            aoc::exercises::day3::run_part1("./day3/input.txt");
            aoc::exercises::day3::run_part2("./day3/input.txt");
        }
        4 => {
            aoc::exercises::day4::run_part1("./day4/input.txt");
            aoc::exercises::day4::run_part2("./day4/input.txt");
        }
        5 => {
            aoc::exercises::day5::run_part1("./day5/input.txt");
            aoc::exercises::day5::run_part2("./day5/input.txt");
        }
        6 => {
            aoc::exercises::day6::run_part1("./day6/input.txt");
            aoc::exercises::day6::run_part2("./day6/input.txt");
        }
        7 => {
            aoc::exercises::day7::run_part1("./day7/input.txt");
            aoc::exercises::day7::run_part2("./day7/input.txt");
        }
        8 => {
            aoc::exercises::day8::run_part1("./day8/input.txt");
            aoc::exercises::day8::run_part2("./day8/input.txt");
        }
        9 => {
            aoc::exercises::day9::run_part1("./day9/input.txt");
            aoc::exercises::day9::run_part2("./day9/input.txt");
        }
        10 => {
            aoc::exercises::day10::run_part1("./day10/input.txt");
            aoc::exercises::day10::run_part2("./day10/input.txt");
        }
        11 => {
            aoc::exercises::day11::run_part1("./day11/input.txt");
            aoc::exercises::day11::run_part2("./day11/input.txt");
        }
        12 => {
            aoc::exercises::day12::run_part1("./day12/input.txt");
            aoc::exercises::day12::run_part2("./day12/input.txt");
        }
        13 => {
            aoc::exercises::day13::run_part1("./day13/input.txt");
            aoc::exercises::day13::run_part2("./day13/input.txt");
        }
        14 => {
            aoc::exercises::day14::run_part1("./day14/input.txt");
            aoc::exercises::day14::run_part2("./day14/input.txt");
        }
        15 => {
            aoc::exercises::day15::run_part1("./day15/input.txt");
            aoc::exercises::day15::run_part2("./day15/input.txt");
        }
        16 => {
            aoc::exercises::day16::run_part1("./day16/input.txt");
            aoc::exercises::day16::run_part2("./day16/input.txt");
        }
        17 => {
            aoc::exercises::day17::run_part1("./day17/input.txt");
            aoc::exercises::day17::run_part2("./day17/input.txt");
        }
        18 => {
            aoc::exercises::day18::run_part1("./day18/input.txt");
            aoc::exercises::day18::run_part2("./day18/input.txt");
        }
        19 => {
            aoc::exercises::day19::run_part1("./day19/input.txt");
            aoc::exercises::day19::run_part2("./day19/input.txt");
        }
        20 => {
            aoc::exercises::day20::run_part1("./day20/input.txt");
            aoc::exercises::day20::run_part2("./day20/input.txt");
        }
        21 => {
            aoc::exercises::day21::run_part1("./day21/input.txt");
            aoc::exercises::day21::run_part2("./day21/input.txt");
        }
        22 => {
            aoc::exercises::day22::run_part1("./day22/input.txt");
            aoc::exercises::day22::run_part2("./day22/input.txt");
        }
        23 => {
            aoc::exercises::day23::run_part1("./day23/input.txt");
            aoc::exercises::day23::run_part2("./day23/input.txt");
        }
        24 => {
            aoc::exercises::day24::run_part1("./day24/input.txt");
            aoc::exercises::day24::run_part2("./day24/input.txt");
        }
        25 => {
            aoc::exercises::day25::run_part1("./day25/input.txt");
            aoc::exercises::day25::run_part2("./day25/input.txt");
        }
        _ => panic!("invalid day picked"),
    }
}
