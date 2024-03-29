use aoc2022 as aoc;
use criterion::{criterion_group, criterion_main, Criterion};

criterion_group!(benches, day1_benchmark);
criterion_main!(benches);

pub fn day1_benchmark(c: &mut Criterion) {
    c.bench_function("day 1 part 1", |b| {
        b.iter(|| aoc::exercises::day1::run_part1())
    });
    c.bench_function("day 1 part 2", |b| {
        b.iter(|| aoc::exercises::day1::run_part2())
    });
}

pub fn day2_benchmark(c: &mut Criterion) {
    c.bench_function("day 2 part 1", |b| {
        b.iter(|| aoc::exercises::day2::run_part1())
    });
    c.bench_function("day 2 part 2", |b| {
        b.iter(|| aoc::exercises::day2::run_part2())
    });
}

pub fn day3_benchmark(c: &mut Criterion) {
    c.bench_function("day 3 part 1", |b| {
        b.iter(|| aoc::exercises::day3::run_part1())
    });
    c.bench_function("day 3 part 2", |b| {
        b.iter(|| aoc::exercises::day3::run_part2())
    });
}

pub fn day4_benchmark(c: &mut Criterion) {
    c.bench_function("day 4 part 1", |b| {
        b.iter(|| aoc::exercises::day4::run_part1())
    });
    c.bench_function("day 4 part 2", |b| {
        b.iter(|| aoc::exercises::day4::run_part2())
    });
}

pub fn day5_benchmark(c: &mut Criterion) {
    c.bench_function("day 5 part 1", |b| {
        b.iter(|| aoc::exercises::day5::run_part1())
    });
    c.bench_function("day 5 part 2", |b| {
        b.iter(|| aoc::exercises::day5::run_part2())
    });
}

pub fn day6_benchmark(c: &mut Criterion) {
    c.bench_function("day 6 part 1", |b| {
        b.iter(|| aoc::exercises::day6::run_part1())
    });
    c.bench_function("day 6 part 2", |b| {
        b.iter(|| aoc::exercises::day6::run_part2())
    });
}

pub fn day7_benchmark(c: &mut Criterion) {
    c.bench_function("day 7 part 1", |b| {
        b.iter(|| aoc::exercises::day7::run_part1())
    });
    c.bench_function("day 7 part 2", |b| {
        b.iter(|| aoc::exercises::day7::run_part2())
    });
}

pub fn day8_benchmark(c: &mut Criterion) {
    c.bench_function("day 8 part 1", |b| {
        b.iter(|| aoc::exercises::day8::run_part1())
    });
    c.bench_function("day 8 part 2", |b| {
        b.iter(|| aoc::exercises::day8::run_part2())
    });
}

pub fn day9_benchmark(c: &mut Criterion) {
    c.bench_function("day 9 part 1", |b| {
        b.iter(|| aoc::exercises::day9::run_part1())
    });
    c.bench_function("day 9 part 2", |b| {
        b.iter(|| aoc::exercises::day9::run_part2())
    });
}

pub fn day10_benchmark(c: &mut Criterion) {
    c.bench_function("day 10 part 1", |b| {
        b.iter(|| aoc::exercises::day10::run_part1())
    });
    c.bench_function("day 10 part 2", |b| {
        b.iter(|| aoc::exercises::day10::run_part2())
    });
}

pub fn day11_benchmark(c: &mut Criterion) {
    c.bench_function("day 11 part 1", |b| {
        b.iter(|| aoc::exercises::day11::run_part1())
    });
    c.bench_function("day 11 part 2", |b| {
        b.iter(|| aoc::exercises::day11::run_part2())
    });
}

pub fn day12_benchmark(c: &mut Criterion) {
    c.bench_function("day 12 part 1", |b| {
        b.iter(|| aoc::exercises::day12::run_part1())
    });
    c.bench_function("day 12 part 2", |b| {
        b.iter(|| aoc::exercises::day12::run_part2())
    });
}

pub fn day13_benchmark(c: &mut Criterion) {
    c.bench_function("day 13 part 1", |b| {
        b.iter(|| aoc::exercises::day13::run_part1())
    });
    c.bench_function("day 13 part 2", |b| {
        b.iter(|| aoc::exercises::day13::run_part2())
    });
}

pub fn day14_benchmark(c: &mut Criterion) {
    c.bench_function("day 14 part 1", |b| {
        b.iter(|| aoc::exercises::day14::run_part1())
    });
    c.bench_function("day 14 part 2", |b| {
        b.iter(|| aoc::exercises::day14::run_part2())
    });
}

pub fn day15_benchmark(c: &mut Criterion) {
    c.bench_function("day 15 part 1", |b| {
        b.iter(|| aoc::exercises::day15::run_part1())
    });
    c.bench_function("day 15 part 2", |b| {
        b.iter(|| aoc::exercises::day15::run_part2())
    });
}

pub fn day16_benchmark(c: &mut Criterion) {
    c.bench_function("day 16 part 1", |b| {
        b.iter(|| aoc::exercises::day16::run_part1())
    });
    c.bench_function("day 16 part 2", |b| {
        b.iter(|| aoc::exercises::day16::run_part2())
    });
}

pub fn day17_benchmark(c: &mut Criterion) {
    c.bench_function("day 17 part 1", |b| {
        b.iter(|| aoc::exercises::day17::run_part1())
    });
    c.bench_function("day 17 part 2", |b| {
        b.iter(|| aoc::exercises::day17::run_part2())
    });
}

pub fn day18_benchmark(c: &mut Criterion) {
    c.bench_function("day 18 part 1", |b| {
        b.iter(|| aoc::exercises::day18::run_part1())
    });
    c.bench_function("day 18 part 2", |b| {
        b.iter(|| aoc::exercises::day18::run_part2())
    });
}

pub fn day19_benchmark(c: &mut Criterion) {
    c.bench_function("day 19 part 1", |b| {
        b.iter(|| aoc::exercises::day19::run_part1())
    });
    c.bench_function("day 19 part 2", |b| {
        b.iter(|| aoc::exercises::day19::run_part2())
    });
}

pub fn day20_benchmark(c: &mut Criterion) {
    c.bench_function("day 20 part 1", |b| {
        b.iter(|| aoc::exercises::day20::run_part1())
    });
    c.bench_function("day 20 part 2", |b| {
        b.iter(|| aoc::exercises::day20::run_part2())
    });
}

pub fn day21_benchmark(c: &mut Criterion) {
    c.bench_function("day 21 part 1", |b| {
        b.iter(|| aoc::exercises::day21::run_part1())
    });
    c.bench_function("day 21 part 2", |b| {
        b.iter(|| aoc::exercises::day21::run_part2())
    });
}

pub fn day22_benchmark(c: &mut Criterion) {
    c.bench_function("day 22 part 1", |b| {
        b.iter(|| aoc::exercises::day22::run_part1())
    });
    c.bench_function("day 22 part 2", |b| {
        b.iter(|| aoc::exercises::day22::run_part2())
    });
}

pub fn day23_benchmark(c: &mut Criterion) {
    c.bench_function("day 23 part 1", |b| {
        b.iter(|| aoc::exercises::day23::run_part1())
    });
    c.bench_function("day 23 part 2", |b| {
        b.iter(|| aoc::exercises::day23::run_part2())
    });
}

pub fn day24_benchmark(c: &mut Criterion) {
    c.bench_function("day 24 part 1", |b| {
        b.iter(|| aoc::exercises::day24::run_part1())
    });
    c.bench_function("day 24 part 2", |b| {
        b.iter(|| aoc::exercises::day24::run_part2())
    });
}

pub fn day25_benchmark(c: &mut Criterion) {
    c.bench_function("day 25 part 1", |b| {
        b.iter(|| aoc::exercises::day25::run_part1())
    });
    c.bench_function("day 25 part 2", |b| {
        b.iter(|| aoc::exercises::day25::run_part2())
    });
}
