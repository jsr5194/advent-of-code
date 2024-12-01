use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

use aoc2022 as aoc;

// based on @Knight-Ops' benchmarking macro
macro_rules! build_bench {
    ($day:ident) => {
        pub fn $day(c: &mut Criterion) {
            let input: String = aoc::common::read_file(
                format!("./src/exercises/{}/input.txt", stringify!($day)).as_str(),
            );
            let processed_input = aoc::exercises::$day::process_input(&input);

            c.bench_function("{} process_input", |b| {
                b.iter(|| aoc::exercises::$day::process_input(&input))
            });
            c.bench_function(format!("{} part_1", stringify!($day)).as_str(), |b| {
                b.iter(|| aoc::exercises::$day::part1(&processed_input))
            });
            c.bench_function(format!("{} part_2", stringify!($day)).as_str(), |b| {
                b.iter(|| aoc::exercises::$day::part2(&processed_input))
            });
        }
    };
}
macro_rules! build_bench_notrim {
    ($day:ident) => {
        pub fn $day(c: &mut Criterion) {
            let input: String = aoc::common::read_file_notrim(
                format!("./src/exercises/{}/input.txt", stringify!($day)).as_str(),
            );
            let processed_input = aoc::exercises::$day::process_input(&input);

            c.bench_function("{} process_input", |b| {
                b.iter(|| aoc::exercises::$day::process_input(&input))
            });
            c.bench_function(format!("{} part_1", stringify!($day)).as_str(), |b| {
                b.iter(|| aoc::exercises::$day::part1(&processed_input))
            });
            c.bench_function(format!("{} part_2", stringify!($day)).as_str(), |b| {
                b.iter(|| aoc::exercises::$day::part2(&processed_input))
            });
        }
    };
}
macro_rules! build_bench_mut {
    ($day:ident) => {
        pub fn $day(c: &mut Criterion) {
            let input: String = aoc::common::read_file(
                format!("./src/exercises/{}/input.txt", stringify!($day)).as_str(),
            );
            let mut processed_input = aoc::exercises::$day::process_input(&input);

            c.bench_function("{} process_input", |b| {
                b.iter(|| aoc::exercises::$day::process_input(&input))
            });
            c.bench_function(format!("{} part_1", stringify!($day)).as_str(), |b| {
                b.iter(|| aoc::exercises::$day::part1(&mut processed_input))
            });
            c.bench_function(format!("{} part_2", stringify!($day)).as_str(), |b| {
                b.iter(|| aoc::exercises::$day::part2(&mut processed_input))
            });
        }
    };
}
macro_rules! build_bench_notrim_mut {
    ($day:ident) => {
        pub fn $day(c: &mut Criterion) {
            let input: String = aoc::common::read_file_notrim(
                format!("./src/exercises/{}/input.txt", stringify!($day)).as_str(),
            );
            c.bench_function("{} process_input", |b| {
                b.iter(|| aoc::exercises::$day::process_input(&input))
            });
            c.bench_function(format!("{} part_1", stringify!($day)).as_str(), |b| {
                b.iter(|| {
                    aoc::exercises::$day::part1(&mut aoc::exercises::$day::process_input(&input))
                })
            });
            c.bench_function(format!("{} part_2", stringify!($day)).as_str(), |b| {
                b.iter(|| {
                    aoc::exercises::$day::part2(&mut aoc::exercises::$day::process_input(&input))
                })
            });
        }
    };
}

build_bench!(day11);
criterion_group!(single, day11);
criterion_main!(single);

//pub fn day1_benchmark(c: &mut Criterion) {
//    let input: String = aoc::common::read_file("./src/exercises/day1/input.txt");
//    let processed_input = aoc::exercises::day1::process_input(&input);
//    c.bench_function("day 1 process_input", |b| {
//        b.iter(|| aoc::exercises::day1::process_input(&input))
//    });
//    c.bench_function("day 1 part 1", |b| {
//        b.iter(|| aoc::exercises::day1::part1(&processed_input))
//    });
//    c.bench_function("day 1 part 1 iter", |b| {
//        b.iter(|| aoc::exercises::day1::part1_iter(&processed_input))
//    });
//    c.bench_function("day 1 part 2", |b| {
//        b.iter(|| aoc::exercises::day1::part2(&processed_input))
//    });
//    c.bench_function("day 1 part 2 iter", |b| {
//        b.iter(|| aoc::exercises::day1::part2_iter(&processed_input))
//    });
//}
//
//pub fn day2_benchmark(c: &mut Criterion) {
//    let input: String = aoc::common::read_file("./src/exercises/day2/input.txt");
//    let mut processed_input = aoc::exercises::day2::process_input(&input);
//    c.bench_function("day 2 process_input", |b| {
//        b.iter(|| aoc::exercises::day2::process_input(&input))
//    });
//    c.bench_function("day 2 part 1", |b| {
//        b.iter(|| aoc::exercises::day2::part1(&mut processed_input))
//    });
//    c.bench_function("day 2 part 2", |b| {
//        b.iter(|| aoc::exercises::day2::part2(&mut processed_input))
//    });
//}
//
////pub fn day3_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day3/input.txt");
////    let mut processed_input = aoc::exercises::day3::process_input(&input);
////    c.bench_function("day 3 process_input", |b| {
////        b.iter(|| aoc::exercises::day3::process_input(&input))
////    });
////    c.bench_function("day 3 part 1", |b| {
////        b.iter(|| aoc::exercises::day3::run_part1(&processed_input))
////    });
////    c.bench_function("day 3 part 2", |b| {
////        b.iter(|| aoc::exercises::day3::run_part2(&processed_input))
////    });
////}
////
////pub fn day4_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day4/input.txt");
////    let mut processed_input = aoc::exercises::day4::process_input(&input);
////    c.bench_function("day 4 process_input", |b| {
////        b.iter(|| aoc::exercises::day4::process_input(&input))
////    });
////    c.bench_function("day 4 part 1", |b| {
////        b.iter(|| aoc::exercises::day4::run_part1(&processed_input))
////    });
////    c.bench_function("day 4 part 2", |b| {
////        b.iter(|| aoc::exercises::day4::run_part2(&processed_input))
////    });
////}
////
////pub fn day5_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day5/input.txt");
////    let mut processed_input = aoc::exercises::day5::process_input(&input);
////    c.bench_function("day 5 process_input", |b| {
////        b.iter(|| aoc::exercises::day5::process_input(&input))
////    });
////    c.bench_function("day 5 part 1", |b| {
////        b.iter(|| aoc::exercises::day5::run_part1(&processed_input))
////    });
////    c.bench_function("day 5 part 2", |b| {
////        b.iter(|| aoc::exercises::day5::run_part2(&processed_input))
////    });
////}
////
////pub fn day6_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day6/input.txt");
////    let mut processed_input = aoc::exercises::day6::process_input(&input);
////    c.bench_function("day 6 process_input", |b| {
////        b.iter(|| aoc::exercises::day6::process_input(&input))
////    });
////    c.bench_function("day 6 part 1", |b| {
////        b.iter(|| aoc::exercises::day6::run_part1(&processed_input))
////    });
////    c.bench_function("day 6 part 2", |b| {
////        b.iter(|| aoc::exercises::day6::run_part2(&processed_input))
////    });
////}
////
////pub fn day7_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day7/input.txt");
////    let mut processed_input = aoc::exercises::day7::process_input(&input);
////    c.bench_function("day 7 process_input", |b| {
////        b.iter(|| aoc::exercises::day7::process_input(&input))
////    });
////    c.bench_function("day 7 part 1", |b| {
////        b.iter(|| aoc::exercises::day7::run_part1(&processed_input))
////    });
////    c.bench_function("day 7 part 2", |b| {
////        b.iter(|| aoc::exercises::day7::run_part2(&processed_input))
////    });
////}
////
////pub fn day8_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day8/input.txt");
////    let mut processed_input = aoc::exercises::day8::process_input(&input);
////    c.bench_function("day 8 process_input", |b| {
////        b.iter(|| aoc::exercises::day8::process_input(&input))
////    });
////    c.bench_function("day 8 part 1", |b| {
////        b.iter(|| aoc::exercises::day8::run_part1(&processed_input))
////    });
////    c.bench_function("day 8 part 2", |b| {
////        b.iter(|| aoc::exercises::day8::run_part2(&processed_input))
////    });
////}
////
////pub fn day9_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day9/input.txt");
////    let mut processed_input = aoc::exercises::day9::process_input(&input);
////    c.bench_function("day 9 process_input", |b| {
////        b.iter(|| aoc::exercises::day9::process_input(&input))
////    });
////    c.bench_function("day 9 part 1", |b| {
////        b.iter(|| aoc::exercises::day9::run_part1(&processed_input))
////    });
////    c.bench_function("day 9 part 2", |b| {
////        b.iter(|| aoc::exercises::day9::run_part2(&processed_input))
////    });
////}
////
////pub fn day10_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day10/input.txt");
////    let mut processed_input = aoc::exercises::day10::process_input(&input);
////    c.bench_function("day 10 process_input", |b| {
////        b.iter(|| aoc::exercises::day10::process_input(&input))
////    });
////    c.bench_function("day 10 part 1", |b| {
////        b.iter(|| aoc::exercises::day10::run_part1(&processed_input))
////    });
////    c.bench_function("day 10 part 2", |b| {
////        b.iter(|| aoc::exercises::day10::run_part2(&processed_input))
////    });
////}
////
////pub fn day11_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day11/input.txt");
////    let mut processed_input = aoc::exercises::day11::process_input(&input);
////    c.bench_function("day 11 process_input", |b| {
////        b.iter(|| aoc::exercises::day11::process_input(&input))
////    });
////    c.bench_function("day 11 part 1", |b| {
////        b.iter(|| aoc::exercises::day11::run_part1(&processed_input))
////    });
////    c.bench_function("day 11 part 2", |b| {
////        b.iter(|| aoc::exercises::day11::run_part2(&processed_input))
////    });
////}
////
////pub fn day12_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day12/input.txt");
////    let mut processed_input = aoc::exercises::day12::process_input(&input);
////    c.bench_function("day 12 process_input", |b| {
////        b.iter(|| aoc::exercises::day12::process_input(&input))
////    });
////    c.bench_function("day 12 part 1", |b| {
////        b.iter(|| aoc::exercises::day12::run_part1(&processed_input))
////    });
////    c.bench_function("day 12 part 2", |b| {
////        b.iter(|| aoc::exercises::day12::run_part2(&processed_input))
////    });
////}
////
////pub fn day13_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day13/input.txt");
////    let mut processed_input = aoc::exercises::day13::process_input(&input);
////    c.bench_function("day 13 process_input", |b| {
////        b.iter(|| aoc::exercises::day13::process_input(&input))
////    });
////    c.bench_function("day 13 part 1", |b| {
////        b.iter(|| aoc::exercises::day13::run_part1(&processed_input))
////    });
////    c.bench_function("day 13 part 2", |b| {
////        b.iter(|| aoc::exercises::day13::run_part2(&processed_input))
////    });
////}
////
////pub fn day14_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day14/input.txt");
////    let mut processed_input = aoc::exercises::day14::process_input(&input);
////    c.bench_function("day 14 process_input", |b| {
////        b.iter(|| aoc::exercises::day14::process_input(&input))
////    });
////    c.bench_function("day 14 part 1", |b| {
////        b.iter(|| aoc::exercises::day14::run_part1(&processed_input))
////    });
////    c.bench_function("day 14 part 2", |b| {
////        b.iter(|| aoc::exercises::day14::run_part2(&processed_input))
////    });
////}
////
////pub fn day15_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day15/input.txt");
////    let mut processed_input = aoc::exercises::day15::process_input(&input);
////    c.bench_function("day 15 process_input", |b| {
////        b.iter(|| aoc::exercises::day15::process_input(&input))
////    });
////    c.bench_function("day 15 part 1", |b| {
////        b.iter(|| aoc::exercises::day15::run_part1(&processed_input))
////    });
////    c.bench_function("day 15 part 2", |b| {
////        b.iter(|| aoc::exercises::day15::run_part2(&processed_input))
////    });
////}
////
////pub fn day16_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day16/input.txt");
////    let mut processed_input = aoc::exercises::day16::process_input(&input);
////    c.bench_function("day 16 process_input", |b| {
////        b.iter(|| aoc::exercises::day16::process_input(&input))
////    });
////    c.bench_function("day 16 part 1", |b| {
////        b.iter(|| aoc::exercises::day16::run_part1(&processed_input))
////    });
////    c.bench_function("day 16 part 2", |b| {
////        b.iter(|| aoc::exercises::day16::run_part2(&processed_input))
////    });
////}
////
////pub fn day17_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day17/input.txt");
////    let mut processed_input = aoc::exercises::day17::process_input(&input);
////    c.bench_function("day 17 process_input", |b| {
////        b.iter(|| aoc::exercises::day17::process_input(&input))
////    });
////    c.bench_function("day 17 part 1", |b| {
////        b.iter(|| aoc::exercises::day17::run_part1(&processed_input))
////    });
////    c.bench_function("day 17 part 2", |b| {
////        b.iter(|| aoc::exercises::day17::run_part2(&processed_input))
////    });
////}
////
////pub fn day18_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day18/input.txt");
////    let mut processed_input = aoc::exercises::day18::process_input(&input);
////    c.bench_function("day 18 process_input", |b| {
////        b.iter(|| aoc::exercises::day18::process_input(&input))
////    });
////    c.bench_function("day 18 part 1", |b| {
////        b.iter(|| aoc::exercises::day18::run_part1(&processed_input))
////    });
////    c.bench_function("day 18 part 2", |b| {
////        b.iter(|| aoc::exercises::day18::run_part2(&processed_input))
////    });
////}
////
////pub fn day19_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day19/input.txt");
////    let mut processed_input = aoc::exercises::day19::process_input(&input);
////    c.bench_function("day 19 process_input", |b| {
////        b.iter(|| aoc::exercises::day19::process_input(&input))
////    });
////    c.bench_function("day 19 part 1", |b| {
////        b.iter(|| aoc::exercises::day19::run_part1(&processed_input))
////    });
////    c.bench_function("day 19 part 2", |b| {
////        b.iter(|| aoc::exercises::day19::run_part2(&processed_input))
////    });
////}
////
////pub fn day20_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day20/input.txt");
////    let mut processed_input = aoc::exercises::day20::process_input(&input);
////    c.bench_function("day 20 process_input", |b| {
////        b.iter(|| aoc::exercises::day20::process_input(&input))
////    });
////    c.bench_function("day 20 part 1", |b| {
////        b.iter(|| aoc::exercises::day20::run_part1(&processed_input))
////    });
////    c.bench_function("day 20 part 2", |b| {
////        b.iter(|| aoc::exercises::day20::run_part2(&processed_input))
////    });
////}
////
////pub fn day21_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day21/input.txt");
////    let mut processed_input = aoc::exercises::day21::process_input(&input);
////    c.bench_function("day 21 process_input", |b| {
////        b.iter(|| aoc::exercises::day21::process_input(&input))
////    });
////    c.bench_function("day 21 part 1", |b| {
////        b.iter(|| aoc::exercises::day21::run_part1(&processed_input))
////    });
////    c.bench_function("day 21 part 2", |b| {
////        b.iter(|| aoc::exercises::day21::run_part2(&processed_input))
////    });
////}
////
////pub fn day22_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day22/input.txt");
////    let mut processed_input = aoc::exercises::day22::process_input(&input);
////    c.bench_function("day 22 process_input", |b| {
////        b.iter(|| aoc::exercises::day22::process_input(&input))
////    });
////    c.bench_function("day 22 part 1", |b| {
////        b.iter(|| aoc::exercises::day22::run_part1(&processed_input))
////    });
////    c.bench_function("day 22 part 2", |b| {
////        b.iter(|| aoc::exercises::day22::run_part2(&processed_input))
////    });
////}
////
////pub fn day23_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day23/input.txt");
////    let mut processed_input = aoc::exercises::day23::process_input(&input);
////    c.bench_function("day 23 process_input", |b| {
////        b.iter(|| aoc::exercises::day23::process_input(&input))
////    });
////    c.bench_function("day 23 part 1", |b| {
////        b.iter(|| aoc::exercises::day23::run_part1(&processed_input))
////    });
////    c.bench_function("day 23 part 2", |b| {
////        b.iter(|| aoc::exercises::day23::run_part2(&processed_input))
////    });
////}
////
////pub fn day24_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day24/input.txt");
////    let mut processed_input = aoc::exercises::day24::process_input(&input);
////    c.bench_function("day 24 process_input", |b| {
////        b.iter(|| aoc::exercises::day24::process_input(&input))
////    });
////    c.bench_function("day 24 part 1", |b| {
////        b.iter(|| aoc::exercises::day24::run_part1(&processed_input))
////    });
////    c.bench_function("day 24 part 2", |b| {
////        b.iter(|| aoc::exercises::day24::run_part2(&processed_input))
////    });
////}
////
////pub fn day25_benchmark(c: &mut Criterion) {
////    let input: String = aoc::common::read_file("./src/exercises/day25/input.txt");
////    let mut processed_input = aoc::exercises::day25::process_input(&input);
////    c.bench_function("day 25 process_input", |b| {
////        b.iter(|| aoc::exercises::day25::process_input(&input))
////    });
////    c.bench_function("day 25 part 1", |b| {
////        b.iter(|| aoc::exercises::day25::run_part1(&processed_input))
////    });
////    c.bench_function("day 25 part 2", |b| {
////        b.iter(|| aoc::exercises::day25::run_part2(&processed_input))
////    });
////}
