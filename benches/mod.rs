use criterion::{criterion_group, criterion_main, Criterion};

mod coherence_bench;
mod system_bench;

criterion_main!(benches);
criterion_group!(benches, iterator_tests,);

pub fn iterator_tests(c: &mut Criterion) {
    let mut bench = coherence_bench::Benchmark::new();

    // Start benchmark
    let mut g = c.benchmark_group("Looping Method Comparison");
    g.bench_function("Hash System Cost", |b| b.iter(|| bench.run_hash_system()));
    g.bench_function("Iterator", |b| b.iter(|| bench.run_iterator()));
    g.bench_function("Foreach", |b| b.iter(|| bench.run_forloop()));
    g.bench_function("Lazy O(n^2)", |b| b.iter(|| bench.run_lazy()));
    g.finish();
}
