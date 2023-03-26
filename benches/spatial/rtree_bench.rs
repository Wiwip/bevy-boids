use rstar::RTree;
use crate::spatial::random_points;

#[derive(Default)]
pub struct Benchmark {
    tree: RTree<[f32; 3]>,
}

impl Benchmark {
    pub fn new() -> Self {
        let tree = RTree::new();

        Self {
            tree,
        }
    }

    pub fn build_tree(&mut self, count: i32) {
        let pts = random_points(count);
        self.tree = RTree::bulk_load(pts);
    }

    pub fn nearest(&mut self) {
        self.tree.nearest_neighbor(&[0.0, 0.0, 0.0]);
    }

    pub fn within(&mut self) {
        self.tree.locate_within_distance([0.0, 0.0, 0.0], 32.0);
    }
}