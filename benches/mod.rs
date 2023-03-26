use criterion::criterion_main;

mod systems;

criterion_main!(spatial::spatial_benches,);
