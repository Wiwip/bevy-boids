use bevy::prelude::*;
use kd_tree::KdTree;
use crate::spatial::random_points;

pub struct Benchmark {
    tree: KdTree<[f32; 3]>,
}

impl Benchmark {
    pub fn new() -> Self {
        let tree = KdTree::build_by_ordered_float(Vec::new());

        Self {
            tree,
        }
    }

    pub fn build_tree(&mut self, count: i32) {
        let pts = random_points(count);
        self.tree = KdTree::build_by_ordered_float(pts);
    }

    pub fn nearest(&mut self) {
        self.tree.nearest(&[0.0, 0.0, 0.0]);
    }

    pub fn within(&mut self) {
        self.tree.within_radius(&[0.0, 0.0, 0.0], 32.0);
    }
}