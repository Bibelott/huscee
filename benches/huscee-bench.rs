use std::hint::black_box;

use criterion::{self, BenchmarkId, Criterion, criterion_group, criterion_main};

use huscee::*;

pub fn perft_benchmark(c: &mut Criterion) {
    let board = Board::start_pos();
    let mut group = c.benchmark_group("perft");
    for depth in 1..5usize {
        group.bench_with_input(BenchmarkId::from_parameter(depth), &depth, |b, &depth| {
            b.iter(|| board.perft(black_box(depth)))
        });
    }

    group.finish();
}

pub fn negamax_benchmark(c: &mut Criterion) {
    let board = Board::start_pos();
    let mut group = c.benchmark_group("negamax");
    for depth in 1..5 {
        group.bench_with_input(BenchmarkId::from_parameter(depth), &depth, |b, &depth| {
            b.iter(|| negamax(f32::NEG_INFINITY, f32::INFINITY, depth, &board))
        });
    }

    group.finish();
}

criterion_group!(benches, perft_benchmark, negamax_benchmark);
criterion_main!(benches);
