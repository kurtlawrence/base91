#[macro_use]
extern crate criterion;

use base91::*;
use criterion::Criterion;

fn benchmark(c: &mut Criterion) {
    let upwards: Vec<u8> = (0..=255u8).collect();
    let downwards: Vec<u8> = (255..=0).collect();
    let large: Vec<u8> = upwards
        .iter()
        .chain(&downwards)
        .cycle()
        .take(5120)
        .map(|x| *x)
        .collect();

    c.bench_function("encode iter", |b| {
        b.iter(|| iter_encode(large.iter().map(|x| *x), |_| ()))
    });

    let encoded = slice_encode(&large);
    c.bench_function("encode iter", |b| {
        b.iter(|| iter_decode(encoded.iter().map(|x| *x), |_| ()))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
