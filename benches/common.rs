use criterion::{BatchSize, Criterion};
use std::fs::File;
use std::io::{BufReader, Cursor};

const FILENAME_BASE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/inputs");

pub fn bench_aoc<O, F>(c: &mut Criterion, name: &str, input_day: &str, soln: F)
where
    F: Fn(&mut BufReader<File>) -> O,
{
    let filename = format!("{}/{}.txt", FILENAME_BASE, input_day);

    c.bench_function(name, |b| {
        b.iter_batched(
            || BufReader::new(std::fs::File::open(&filename).expect("file open failed")),
            |mut f| soln(&mut f),
            BatchSize::PerIteration,
        )
    });
}

pub fn bench_aoc_nofile<O, F, const N: usize>(
    c: &mut Criterion,
    name: &str,
    input: &Cursor<&[u8; N]>,
    soln: F,
) where
    F: Fn(&mut Cursor<&[u8; N]>) -> O,
{
    c.bench_function(name, |b| {
        b.iter_batched(
            || input.clone(),
            |mut f| soln(&mut f),
            BatchSize::PerIteration,
        )
    });
}
