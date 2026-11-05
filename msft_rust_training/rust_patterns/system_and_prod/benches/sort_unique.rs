use std::hint::black_box;

use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use system_and_prod::how_to_proptest::sort_unique;

fn bench_sort_unique(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort_unique");

    for n in [100usize, 1_000, 10_000] {
        let data: Vec<i32> = (0..n as i32).rev().collect();

        group.bench_with_input(BenchmarkId::from_parameter(n), &data, |b, data| {
            b.iter_batched(
                || data.clone(),
                |input| sort_unique(black_box(input)),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_sort_unique);
criterion_main!(benches);
