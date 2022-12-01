use aoc22::day01::{solve_part1, solve_part2};
use criterion::{criterion_group, criterion_main, Criterion};

mod common;

use crate::common::{bench_aoc, bench_aoc_nofile};

pub fn part1_benchmark(c: &mut Criterion) {
    bench_aoc(c, "day01 part1", "day01", |f| solve_part1(f));
}

pub fn part1_benchmark_nofile(c: &mut Criterion) {
    let input = aoc22::inline_input!("day01");
    bench_aoc_nofile(c, "day01 part1 (nofile)", &input, |f| solve_part1(f));
}

pub fn part2_benchmark(c: &mut Criterion) {
    bench_aoc(c, "day01 part2", "day01", |f| solve_part2(f));
}

pub fn part2_benchmark_nofile(c: &mut Criterion) {
    let input = aoc22::inline_input!("day01");
    bench_aoc_nofile(c, "day01 part2 (nofile)", &input, |f| solve_part2(f));
}

criterion_group!(
    day01,
    part1_benchmark,
    part1_benchmark_nofile,
    part2_benchmark,
    part2_benchmark_nofile
);
criterion_main!(day01);
