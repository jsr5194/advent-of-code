use aoc2021 as aoc;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day1_benchmark(c: &mut Criterion) {
    c.bench_function("day 1 part 1", |b| b.iter(|| aoc::day1::run_part_1()));
    c.bench_function("day 1 part 2", |b| b.iter(|| aoc::day1::run_part_2()));
}

pub fn day2_benchmark(c: &mut Criterion) {
    c.bench_function("day 2 part 1", |b| b.iter(|| aoc::day2::run_part_1()));
    c.bench_function("day 2 part 2", |b| b.iter(|| aoc::day2::run_part_2()));
}

criterion_group!(benches, day2_benchmark);
criterion_main!(benches);