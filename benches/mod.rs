use criterion::{Criterion, criterion_group, criterion_main};

mod spatial_bench;
mod system_bench;

criterion_main!(benches);
criterion_group!(benches,
  //  move_system_test,
  //  spatial_tests,
    iterator_tests,
);


pub fn move_system_test(c: &mut Criterion) {
    let mut bench = system_bench::Benchmark::new();
    c.bench_function("Movement System", |b| b.iter(|| bench.run()));
}

pub fn spatial_tests(c: &mut Criterion) {
    let mut bench = spatial_bench::Benchmark::new();
    c.bench_function("Spatial Hash", |b| b.iter(|| bench.run()));
    c.bench_function("Coherence Only", |b| b.iter(|| bench.run_coherence_only()));

    let mut g = c.benchmark_group("Coherence System");
    g.bench_function("Brute Force Method", |b| b.iter(|| bench.run_brute()));
    g.bench_function("HashMap Query", |b| b.iter(|| bench.run_full_query()));
    g.finish();

}

pub fn iterator_tests(c: &mut Criterion) {
    let mut bench = spatial_bench::Benchmark::new();
    let mut g = c.benchmark_group("Iterator or Combinators");
    g.bench_function("Iterator", |b| b.iter(|| bench.run_foreach_iterator()));
    g.bench_function("Combinators", |b| b.iter(|| bench.run_combinator()));
    g.finish();
}