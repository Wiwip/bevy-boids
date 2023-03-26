use criterion::{criterion_group, Criterion};
use rand::distributions::Uniform;
use rand::prelude::*;

mod bruteforce;
mod kdtree_space;
mod rtree_space;
mod voxel_space;

criterion_group!(system_benches, spatial_system_benches,);

fn spatial_system_benches(c: &mut Criterion) {
    // let mut g = &c.benchmark_group("Spatial Structure Tests");
    // let baseline = random_points(10000);

    /*
    let mut bench = bruteforce::Benchmark::new(baseline.clone());
    g.bench_function("brute_force", |b| {
        b.iter(|| bench.run());
    });

    let mut bench = voxel_space::Benchmark::new(baseline.clone());
    g.bench_function("voxel_hashmap", |b| {
        b.iter(|| bench.run());
    });

    let mut bench = kdtree_space::Benchmark::new(baseline.clone());
    g.bench_function("kdree_map", |b| {
        b.iter(|| bench.run());
    });

    let mut bench = rtree_space::Benchmark::new(baseline.clone());
    g.bench_function("rtree_map", |b| {
        b.iter(|| bench.run());
    });
    g.finish();
    */

    /*
        g.bench_function("KdTree: Nearest Query", |b| {
            let mut bench = kdtree_bench::Benchmark::new();
            &bench.build_tree(baseline.clone());
            b.iter(|| bench.nearest() )
        });

        g.bench_function("KdTree: Within Radius", |b| {
            let mut bench = kdtree_bench::Benchmark::new();
            &bench.build_tree(baseline.clone());
            b.iter(|| bench.within() )
        });

        g.bench_function("RTree: Build Tree", |b| {
            let mut bench = rtree_bench::Benchmark::new();
            b.iter(|| bench.build_tree(baseline.clone()));
        });

        g.bench_function("RTree: Nearest Query", |b| {
            let mut bench = rtree_bench::Benchmark::new();
            &bench.build_tree(baseline.clone());
            b.iter(|| bench.nearest() )
        });

        g.bench_function("RTree: Within Radius", |b| {
            let mut bench = rtree_bench::Benchmark::new();
            &bench.build_tree(baseline.clone());
            b.iter(|| bench.within() )
        });

    */
}

fn random_points(count: i32) -> Vec<[f32; 3]> {
    let mut list = Vec::new();

    for _ in 0..count {
        list.push(uniform_position());
    }

    return list;
}

fn uniform_position() -> [f32; 3] {
    let mut rng = rand::thread_rng();
    let mut dist = Uniform::from(-1000.0..1000.0);

    let pos = [
        dist.sample(&mut rng),
        dist.sample(&mut rng),
        dist.sample(&mut rng),
    ];

    pos
}
