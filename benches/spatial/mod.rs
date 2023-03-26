use bevy::math::ivec3;
use bevy::prelude::*;
use criterion::{criterion_group, Criterion, BenchmarkId};
use rand::distributions::Uniform;
use rand::distributions::Distribution;
use rand::Rng;

mod bruteforce;
mod voxel_space;
mod voxel_bench;
mod rtree_space;
mod kdtree_space;
mod kdtree_bench;
mod rtree_bench;

criterion_group!(
    spatial_benches,
    spatial_tests,
);

fn spatial_tests(c: &mut Criterion) {
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

    let mut g = c.benchmark_group("Spatial Comparison");
    for i in [500, 1000, 2500, 5000, 10000].iter() {
        let mut kd_bench = kdtree_bench::Benchmark::new();
        g.bench_with_input(BenchmarkId::new("KdTree", i), i,
                           |b, i| b.iter(|| kd_bench.build_tree(*i)));

        let mut rtree_bench = rtree_bench::Benchmark::new();
        g.bench_with_input(BenchmarkId::new("RTree", i), i,
                           |b, i| b.iter(|| rtree_bench.build_tree(*i)));

        let mut hashmap_bench = voxel_bench::Benchmark{ cell_size: 32.0, ..default() };
        g.bench_with_input(BenchmarkId::new("HashMap", i), i,
                           |b, i| b.iter(|| hashmap_bench.build_tree(*i)));
    }
    g.finish();

    let mut g = c.benchmark_group("Spatial Queries Comparison");
    for i in [500, 1000, 2500, 5000, 10000].iter() {
        let mut kd_bench = kdtree_bench::Benchmark::new();
        kd_bench.build_tree(*i);
        g.bench_with_input(BenchmarkId::new("KdTree", i), i,
                           |b, i| b.iter(|| kd_bench.within()));

        let mut rtree_bench = rtree_bench::Benchmark::new();
        rtree_bench.build_tree(*i);
        g.bench_with_input(BenchmarkId::new("RTree", i), i,
                           |b, i| b.iter(|| rtree_bench.within()));

        let mut hashmap_bench = voxel_bench::Benchmark{ cell_size: 32.0, list_offsets: vec![
            ivec3(-1, 1, 0),
            ivec3(0, 1, 0),
            ivec3(1, 1, 0),
            ivec3(-1, 0, 0),
            ivec3(0, 0, 0),
            ivec3(1, 0, 0),
            ivec3(-1, -1, 0),
            ivec3(0, -1, 0),
            ivec3(1, -1, 0),
        ], ..default()
        };
        hashmap_bench.build_tree(*i);
        g.bench_with_input(BenchmarkId::new("HashMap", i), i,
                           |b, i| b.iter(|| hashmap_bench.within()));
    }
    g.finish();

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