use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use sum_of_multiples::{sum_of_multiples, sum_of_multiples_slow};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sum of multiples");
    let factors = &[2, 3, 5, 7, 11];
    for i in [10_000u32, 100_000u32, 1_000_000u32].iter() {
        group.bench_with_input(BenchmarkId::new("inclusive", i), i, |b, i| {
            b.iter(|| sum_of_multiples_slow(*i, factors))
        });
        group.bench_with_input(BenchmarkId::new("exclusive", i), i, |b, i| {
            b.iter(|| sum_of_multiples(*i, factors))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
