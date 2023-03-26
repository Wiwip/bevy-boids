use std::collections::hash_map::Entry;
use std::collections::HashMap;
use bevy::math::ivec3;
use bevy::prelude::*;
use rand_distr::num_traits::pow;
use crate::spatial::random_points;


#[derive(Default)]
pub struct Benchmark {
    pub map: HashMap<IVec3, Vec<[f32; 3]>>,
    pub list_offsets: Vec<IVec3>,
    pub cell_size: f32,
}

impl Benchmark {
    pub fn build_tree(&mut self, count: i32) {
        let pts = random_points(count);
        self.bulk_insert(pts);
    }

    pub fn nearest(&mut self) {
        //self.tree.nearest_neighbor(&[0.0, 0.0, 0.0]);
    }

    pub fn within(&mut self) {
        let mut list = Vec::new();
        let local = self.global_to_map_loc([0.0, 0.0, 0.0]);

        // Broad range checks
        for offset in &self.list_offsets {
            let key = local + *offset;

            if let Some(tfs) = self.map.get(&key) {
                list.extend(tfs.clone());
            }
        }

        // Precise range check
        let perception_squared = pow(self.cell_size, 2);

        let mut temp_list = Vec::new();
        for pos in list {
            let other = Vec3::from(pos);
            let origin = Vec3::from([0.0, 0.0, 0.0]);
            if origin.distance_squared(other) <= perception_squared {
                temp_list.push(other);
            }
        }
    }

    fn insert(&mut self, global: [f32; 3]) {
        let local = self.global_to_map_loc(global);

        // Add entity to selected map cell
        match self.map.entry(local) {
            Entry::Occupied(mut o) => {
                o.get_mut().push(global);
            }
            Entry::Vacant(v) => {
                v.insert(vec![global]);
            }
        };
    }

    fn bulk_insert(&mut self, bulk: Vec<[f32; 3]>) {
        for p in bulk {
            self.insert(p);
        }
    }

    pub fn global_to_map_loc(&self, global: [f32; 3]) -> IVec3 {
        let x = f32::floor(global[0] / self.cell_size) as i32;
        let y = f32::floor(global[1] / self.cell_size) as i32;
        let z = f32::floor(global[2] / self.cell_size) as i32;
        return ivec3(x, y, z);
    }
}