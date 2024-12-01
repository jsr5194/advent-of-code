use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

use aoc2024 as aoc;

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
                b.iter(|| aoc::exercises::$day::run_part1(&processed_input))
            });
            c.bench_function(format!("{} part_2", stringify!($day)).as_str(), |b| {
                b.iter(|| aoc::exercises::$day::run_part2(&processed_input))
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

build_bench!(day1);
criterion_group!(single, day1);
criterion_main!(single);
