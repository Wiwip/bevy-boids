
use bevy::prelude::*;
use criterion::criterion_main;

mod spatial;


criterion_main!(
    spatial::spatial_benches,
);

