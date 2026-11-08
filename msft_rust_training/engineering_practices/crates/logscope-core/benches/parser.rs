use criterion::{Criterion, criterion_group, criterion_main};
use logscope_core::parse_line;
use std::hint::black_box;

fn benchmark_parser(c: &mut Criterion) {
    let line = "2026-11-07T10:00:00 ERROR database connection failed";
    c.bench_function("parse_one_log_line", |b| {
        b.iter(|| parse_line(black_box(line)))
    });
}

criterion_group!(benches, benchmark_parser);
criterion_main!(benches);
